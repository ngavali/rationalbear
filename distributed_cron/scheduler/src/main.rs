/***
 * Worker model Prototype
 * Author: gavali.nilesh80186@gmail.com
 *
 * Producer
 * 1. Publishes a job schedule
 * 2. Clean failed job schedule
 ***/

use std::time;

use openssl::ssl::{SslConnector, SslFiletype, SslMethod, SslVerifyMode};
use postgres::{Client, NoTls, Transaction};
use postgres_openssl::MakeTlsConnector;

use std::vec::Vec;

#[derive(Debug, Clone)]
struct Job {
    id: i64,
    frequency: i64,
    next_run_epoch: i64,
}

//Place holder
struct Schedule;

impl Schedule {
    fn add(&self, atomic: &mut Transaction, job: Job) -> Option<i64> {
        //this is as good as pushing a message to Queue
        //We are using database for this purpose here. All in the name of "ACID"
        match atomic.query_one(
            "insert into rudra.schedule (jobid) values ($1) RETURNING id", // , host, pid) values ($1, cast(host(inet_client_addr()::inet) as varchar), $2 )",
            &[&job.id],
        ) {
            Ok(x) => {
                let schedule_id: i64 = x.get(0);
                Some(schedule_id)
            }
            _ => None,
        }
    }
}

impl Job {
    //Set next run time for a job to trigger scheduling
    fn next_run_epoch(&self, atomic: &mut Transaction) -> bool {
        match atomic.execute(
            "update rudra.job set next_run_epoch = $2 where id = $1",
            &[&self.id, &self.next_run_epoch],
        ) {
            Ok(x) if x == 1 => true,
            _ => false,
        }
    }
}

fn get_runnable_jobs(conn: &mut Client) -> Vec<Job> {
    let result = conn
        .query(
            "select 
            id ,
            frequency,
            next_run_epoch
            from rudra.job 
            where next_run_epoch <= extract(epoch from now()) 
            and is_active = true
            and NOT ( is_unique and id IN ( select jobid from rudra.schedule group by jobid ) )",
            &[],
        )
        .unwrap();
    let mut jobs: Vec<Job> = Vec::new();
    for row in result {
        let id: i64 = row.get(0);
        let frequency: i64 = row.get(1);
        let next_run_epoch: i64 = row.get(2);

        jobs.push(Job {
            id,
            frequency,
            next_run_epoch,
        });
    }
    jobs
}

//Using advisory locks, other method is to SERIALIZE the transaction.
//Must be a single authority... distributed and we fail!
fn advisory_lock(conn: &mut Client, name_space: String) -> bool {
    //SELECT pg_advisory_unlock(1)
    let result = conn
        .query("SELECT pg_try_advisory_lock(hashtext($1))", &[&name_space])
        .unwrap();
    let mut lock: bool = false;

    for row in result {
        lock = row.get(0);
        println!("Lock {:?}", lock);
    }

    lock
}

fn create_schedule(client: &mut Client, runnable_jobs: Vec<Job>) {
    let schedule = Schedule;
    for mut job in runnable_jobs {
        let mut atomic = client.transaction().unwrap();
        let mut success: bool = false;
        let mut schedule_id: i64 = 0;

        //Compute next run epoch
        let now = time::SystemTime::now()
            .duration_since(time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
        let next_run_epoch = ((now + job.frequency) / job.frequency) * job.frequency;

        job.next_run_epoch = next_run_epoch;

        let j = job.clone();
        match schedule.add(&mut atomic, j) {
            //we can schedule add jobs and then schedule.save (where schedule struct then holds collection of jobs)
            //schedule.save then actually tried to insert job and update its next runtime
            //different thought???
            Some(s_id) => {
                schedule_id = s_id;
                if job.next_run_epoch(&mut atomic) {
                    success = true;
                }
                //we can decide what to do if next_run_epoch couldnt be updated. Check if same job appears mutliple time in it's frequency window.
            }
            None => {}
        }
        if success {
            println!(
                "Job {:5} scheduled with schedule id -> {:5} -> next_run_epoch -> {:10}",
                job.id, schedule_id, next_run_epoch
            );
            atomic.commit().unwrap();
        } else {
            println!("Unable to schedule job {:5}", job.id);
            atomic.rollback().unwrap();
        }
        //create_schedule(&mut client, schedule);
    }
}

fn cleanup_failed_schedule(client: &mut Client) {
    let mut atomic = client.transaction().unwrap();
    match atomic.execute(
        "insert into rudra.stats (id, jobid, host, status, start_time, run_count)
        select s.id, s.jobid, s.host, s.status, s.start_time, s.run_count
        from 
            rudra.schedule s 
        left join 
            rudra.job j 
        on 
            s.jobid = j.id
        where 
            s.run_count >= retries and 
            status = 'error'",
        &[],
    ) {
        Ok(_) => {
            match atomic.execute(
                "delete from rudra.schedule s where s.id IN ( select id from rudra.stats where status = 'error')",
                &[],
            ) {
                Ok(cleaned_count) => {
                    println!("Cleaned {:100} failed jobs", cleaned_count);
                }
                Err(_) => {}
            };
            atomic.commit().unwrap();
        }
        Err(err) => {
            println!("Cleanup error -> {}", err);
            atomic.rollback().unwrap();
        }
    }
}

/*
fn cleanup_long_running_schedule(client: &mut Client) {
    //to be done
}
*/
fn main() {
    let secure = false; //change value accordingly while testing, placing certs and keys at rigth location
                        //or use appropriate paths below
    let dsn = "host=192.168.56.102 user=ngavali password=ngavali dbname=mydb";

    let mut client = match secure {
        true => {
            //For TLS connection
            let mut builder = SslConnector::builder(SslMethod::tls()).unwrap();
            builder.set_verify(SslVerifyMode::all());
            builder
                .set_ca_file("/home/ngavali/certs/cacert.pem")
                .unwrap();
            builder
                .set_private_key_file("/home/ngavali/certs/mycert.key", SslFiletype::PEM)
                .unwrap();
            builder
                .set_certificate_file("/home/ngavali/certs/mycert.crt", SslFiletype::PEM)
                .unwrap();

            let connector = MakeTlsConnector::new(builder.build());
            Client::connect(dsn, connector)
        }
        false => Client::connect(dsn, NoTls),
    }
    .unwrap();

    //advisory_lock_(client);
    //Both advisory lock and transaction are part of the same session.
    //As long as transactions succeed  we are holding a lock, if they fail due to some network
    //conditions we donot have ownership of the lock too, then defer from all actions and exit
    if advisory_lock(&mut client, String::from("myname_space")) {
        let runnable_jobs = get_runnable_jobs(&mut client);
        println!("Runnable jobs {:?}", runnable_jobs);

        if runnable_jobs.len() == 0 {
            println!("No runnable jobs found.")
        } else {
            create_schedule(&mut client, runnable_jobs);
        }

        //remove any messages that cannot be processed
        cleanup_failed_schedule(&mut client);

        //        cleanup_long_running_schedule(&mut client);
    }
    //in daemon mode, we have two independent threads
    //One periodically scanning for ready jobs (get_runnable_jobs)
    //Another listening to add job to the schedule (create_schedule [bulk addition] or schedule_job [one at a time]) <-- depends on messaging model support

    //some cleanup not required though
    drop(client);
}
