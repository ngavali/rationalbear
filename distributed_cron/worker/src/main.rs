/* The MIT License

   Copyright (c) 2008, 2009 by gavali.nilesh80186 <gavali.nilesh80186@gmail.com>

   Permission is hereby granted, free of charge, to any person obtaining
   a copy of this software and associated documentation files (the
   "Software"), to deal in the Software without restriction, including
   without limitation the rights to use, copy, modify, merge, publish,
   distribute, sublicense, and/or sell copies of the Software, and to
   permit persons to whom the Software is furnished to do so, subject to
   the following conditions:

   The above copyright notice and this permission notice shall be
   included in all copies or substantial portions of the Software.

   THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
   EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
   MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
   NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
   BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
   ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
   CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
   SOFTWARE.
*/

use postgres::{Client, NoTls, Transaction};

use std::time;
use std::time::{Duration, SystemTime};

use std::process::{Command, ExitStatus};
use std::thread;
use std::thread::JoinHandle;

use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};

use libc;
use log;
//use procinfo::pid;

struct Db {
    host: String,
    port: u32,
    user: String,
    pass: String,
    dbname: String,
    crt: String,
    key: String,
    cacrt: String,
}

impl Db {
    pub fn new() -> Db {
        Db {
            host: String::from("192.168.56.102"),
            port: 5432,
            user: String::from("ngavali"),
            pass: String::from("ngavali"),
            dbname: String::from("mydb"),
            crt: String::from(""),
            key: String::from(""),
            cacrt: String::from(""),
        }
    }

    pub fn connect(&self) -> Client {
        let dsn = format!(
            "host={} port={} user={} password={} dbname={}",
            self.host, self.port, self.user, self.pass, self.dbname
        );
        let client = Client::connect(dsn.as_str(), NoTls).unwrap();
        client
    }
}

#[derive(Debug, Clone)]
struct Job {
    schedule_id: i64,
    job_id: i64,
    env: String,
    command: String,
    pid: i64,
    start_time: i64,
    end_time: i64,
    usr_time: i64,
    sys_time: i64,
    rss_max: i64,
    exit_status: i32,
}

impl Job {
    fn claim(&self, client: &mut Transaction) -> bool {
        match client.execute(
            "update rudra.schedule 
                set 
                status = 'claimed', run_count = run_count+1 
            where 
                status IN ( 'ready', 'error' ) and 
                id = $1",
            &[&self.schedule_id],
        ) {
            Ok(1) => {
                thread::sleep(Duration::new(1, 0));
                log::debug!("Claimed {}", self.schedule_id);
                true
            }
            Err(err) => {
                println!("Claim error {:?}", err);
                false
            }
            _ => {
                println!("I just didn't make it.");
                false
            }
        }
    }

    fn started(&self, client: &mut Transaction) -> bool {
        match client.execute(
            "update rudra.schedule
        set
        status = 'started',
        host = host(inet_client_addr()::inet)::varchar,
        start_time = $1,
        pid = $2
        where id = $3",
            &[&self.start_time, &self.pid, &self.schedule_id],
        ) {
            Ok(1) => {
                log::debug!("Updated status");
                true
            }
            _ => false,
        }
    }

    fn save_stats(&self, client: &mut Transaction) -> bool {
        println!("Save job stats");
        match client.execute(
            //query.as_str(),
            //&[],
            "insert into rudra.stats
                 select
                     id,
                     jobid,
                     host,
                     'completed' status,
                     run_count,
                     start_time,
                     $1 end_time,
                     $2 usr_time,
                     $3 sys_time,
                     $4 rss_max
                 from rudra.schedule where id = $5 and jobid= $6",
            &[
                &self.end_time,
                &self.usr_time,
                &self.sys_time,
                &self.rss_max,
                &self.schedule_id,
                &self.job_id,
            ],
        ) {
            Ok(1) => true,
            Ok(x) => {
                println!("cannot insert {}", x);
                false
            }
            Err(err) => {
                println!("{:?}", err);
                false
            }
        }
    }

    fn cleanup(&self, mut client: &mut Transaction) -> bool {
        if self.exit_status != 0 {
            self.failed(&mut client)
        } else {
            match client.execute(
                "delete from rudra.schedule 
                where host = host(inet_client_addr()::inet)::varchar and
                pid = $1 and
                id = $2 and
                jobid = $3
                ",
                &[&self.pid, &self.schedule_id, &self.job_id],
            ) {
                Ok(1) => true,
                _ => false,
            }
        }
    }

    fn failed(&self, client: &mut Transaction) -> bool {
        match client.execute(
            "update rudra.schedule
            set status =  'error'
            where 
                pid = $1 and
                id = $2 and
                jobid = $3
        ",
            &[&self.pid, &self.schedule_id, &self.job_id],
        ) {
            Ok(1) => true,
            _ => false,
        }
    }
}

fn fetch_schedule(database: &Db, name_space: String, max_jobs: i64) -> Vec<Job> {
    let mut schedule: Vec<Job> = Vec::new();
    let mut client = database.connect();
    let result = client
        .query(
            "select 
            s.id, s.jobid, e.name, j.command
        from rudra.schedule s 
            left join rudra.job j 
                on s.jobid = j.id 
            left join rudra.env e
                on j.env_id = e.id
        where 
            s.status IN ( 'ready', 'error') and j.is_active = true and s.run_count < j.retries
        order by j.priority desc
        limit $1",
            &[&max_jobs],
        )
        .unwrap();

    //create schedule
    for row in result {
        schedule.push(Job {
            schedule_id: row.get(0),
            job_id: row.get(1),
            env: row.get(2),
            command: row.get(3),
            pid: 0,
            start_time: 0,
            end_time: 0,
            usr_time: 0,
            sys_time: 0,
            rss_max: 0,
            exit_status: 0,
        })
    }

    schedule
}

fn process_schedule(database: &Db, schedule: Vec<Job>, tx: Sender<Job>) {
    let mut max_jobs = 2;
    let mut threads: Vec<JoinHandle<()>> = Vec::with_capacity(max_jobs);

    let mut client = database.connect();
    for mut job in schedule {
        let mut atomic = client.transaction().unwrap();

        if job.claim(&mut atomic) {
            let mut cmd = Command::new(job.env.clone());
            cmd.args(&["-c", job.command.as_str()]);

            //push job execution to the background and wait for child to exit in a thread.
            let start_time = time::SystemTime::now()
                .duration_since(time::UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64;

            let child = cmd.spawn().unwrap();
            let pid = child.id();

            println!("Started process with pid: {}", pid);
            job.start_time = start_time;
            job.pid = pid as i64;

            if job.started(&mut atomic) {
                //by this point we have started the process.
                atomic.commit().unwrap();
                //?? shall we terminate above job if commit doesn't succeed?
                //at this point we have lost track in persistent storage that the job is running and couldn't update it state
                //try sending kill signal and handle the situation as much as you, beyond that its out of
                //control == print pid in the logs and externally try to kill by debuggin through logs
                //or
                //Drop an alert/email to the user.

                max_jobs -= 1;

                //Push wait from child into a separate thread
                let job_status = tx.clone();
                threads.push(thread::spawn(move || {
                    unsafe {
                        let mut status: i32 = 0;
                        let status_ptr: *mut i32 = &mut status;

                        let options: i32 = 0;

                        let mut rusage: libc::rusage =
                            std::mem::MaybeUninit::zeroed().assume_init(); //std::mem::uninitialized();
                        let rusage_ptr: *mut libc::rusage = &mut rusage;

                        libc::wait4(job.pid as i32, status_ptr, options, rusage_ptr);
                        //let exit_status = child.wait().unwrap(); //usual but won't give us the
                        //resource usage, for using unsafe libc wait
                        /*
                                               println!(
                                                   "Result:
                        Exit status    -> {}
                        Max RSS        -> {}
                        usr time sec   -> {}
                        usr time usec  -> {}
                        sys time sec   -> {}
                        sys time usec  -> {}
                        ",
                                                   status,
                                                   rusage.ru_maxrss,
                                                   rusage.ru_utime.tv_sec,
                                                   rusage.ru_utime.tv_usec,
                                                   rusage.ru_stime.tv_sec,
                                                   rusage.ru_stime.tv_usec
                                               );*/
                        job.end_time = time::SystemTime::now()
                            .duration_since(time::UNIX_EPOCH)
                            .unwrap()
                            .as_secs() as i64;
                        job.usr_time = rusage.ru_utime.tv_sec;
                        job.sys_time = rusage.ru_stime.tv_sec;
                        job.rss_max = rusage.ru_maxrss;
                        job.exit_status = status;
                        job_status.send(job).unwrap();
                    }
                }));
            } else {
                //Let the process continue and leave dangling state in database
                atomic.rollback().unwrap();
            }
        } else {
            println!(
                "Couldn't claim job {} with schedule id -> {}",
                job.job_id, job.schedule_id
            );

            atomic.rollback().unwrap();
        }

        if max_jobs == 0 {
            break;
        }
    }

    for t in threads {
        t.join().unwrap();
    }
}

fn save_job_stats(database: &Db, job: Job) -> bool {
    if job.exit_status != 0 {
        true
    } else {
        let mut client = database.connect();
        let mut atomic = client.transaction().unwrap();
        if job.save_stats(&mut atomic) {
            atomic.commit().unwrap();
            true
        } else {
            atomic.rollback().unwrap();
            false
        }
    }
}

fn job_cleanup(database: &Db, job: Job) -> bool {
    let mut client = database.connect();
    let mut atomic = client.transaction().unwrap();
    if job.cleanup(&mut atomic) {
        atomic.commit().unwrap();
        true
    } else {
        atomic.rollback().unwrap();
        false
    }
}

fn main() {
    let db_final_handler = Db::new();
    let db_scheduler = Db::new();

    let schedule = fetch_schedule(&db_scheduler, String::from("mynamespace"), 10);

    let (tx, rx): (Sender<Job>, Receiver<Job>) = mpsc::channel();
    //start job completion regiter thread
    let final_handler = thread::spawn(move || {
        for job in rx {
            println!(
                "Completed job -> {:5} with schedule id -> {:5} with exitstatus -> {:x}",
                job.job_id, job.schedule_id, job.exit_status
            );
            if save_job_stats(&db_final_handler, job.clone()) {
                job_cleanup(&db_final_handler, job);
            }
        }
    });

    process_schedule(&db_scheduler, schedule, tx);

    //Wait for all job state to be updated
    final_handler.join().unwrap();

    drop(db_scheduler);

    println!("Worker process ended!!!")
}
