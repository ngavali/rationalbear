use std::{
    collections::{HashMap, VecDeque},
    io::{Read, Write},
    net::TcpStream,
    os::fd::{AsRawFd, FromRawFd},
};

use libc;

trait EndPoint {
    fn get_fd(&self) -> libc::c_int;
    fn write(&mut self, data: &[u8]) -> std::io::Result<usize>;
    fn read(&mut self, data: &mut [u8]) -> std::io::Result<usize>;
}

#[derive(Debug)]
struct FrontendStream {
    stream: TcpStream,
}

impl FrontendStream {
    fn new(fd: libc::c_int) -> Self {
        let tcp_stream = unsafe { TcpStream::from_raw_fd(fd) };
        FrontendStream { stream: tcp_stream }
    }
}

impl Drop for FrontendStream {
    fn drop(&mut self) {
        //Included only for demostration purposes only.
        //shutdown will be handled once value is dropped.
        match self.stream.shutdown(std::net::Shutdown::Both) {
            Ok(_) => {
                println!("Connection closed. fd={}", self.get_fd());
            }
            Err(_err) => {
                println!("Error while closing the fd. Error={}", _err.to_string());
            }
        }
        println!("FrontendStream Dropped : {:#?}", self);
    }
}

impl EndPoint for FrontendStream {
    fn get_fd(&self) -> libc::c_int {
        self.stream.as_raw_fd()
    }

    fn write(&mut self, data: &[u8]) -> std::io::Result<usize> {
        self.stream.write(data)
    }

    fn read(&mut self, data: &mut [u8]) -> std::io::Result<usize> {
        self.stream.read(data)
    }
}

#[derive(Debug)]
struct BackendStream {
    stream: TcpStream,
}

impl Drop for BackendStream {
    fn drop(&mut self) {
        self.stream.shutdown(std::net::Shutdown::Both).unwrap();
        println!("BackendStream Dropped : {:#?}", self);
    }
}

impl BackendStream {
    fn new(address: &str) -> Self {
        let stream = TcpStream::connect(address).unwrap();
        BackendStream { stream }
    }
}

impl EndPoint for BackendStream {
    fn get_fd(&self) -> libc::c_int {
        self.stream.as_raw_fd()
    }

    fn write(&mut self, data: &[u8]) -> std::io::Result<usize> {
        self.stream.write(data)
    }

    fn read(&mut self, data: &mut [u8]) -> std::io::Result<usize> {
        self.stream.read(data)
    }
}

#[derive(Debug, Clone)]
struct Epoll {
    epoll_fd: i32,
}

impl Epoll {
    fn new() -> Self {
        let epoll_fd = unsafe { libc::epoll_create1(0) };
        println!("Epoll instance created. fd={}", epoll_fd);
        Epoll { epoll_fd }
    }

    fn remove_from_interest_list(&mut self, fd: libc::c_int) {
        println!("Remove fd from event listener");
        unsafe {
            let flags: u32 = libc::EPOLLIN as u32;
            let frontend_epoll_event = &mut libc::epoll_event {
                events: flags,
                u64: fd as u64,
            };

            libc::epoll_ctl(self.epoll_fd, libc::EPOLL_CTL_DEL, fd, frontend_epoll_event);
        }
    }

    fn add_to_interest_list(&self, fd: libc::c_int) {
        println!("Add fd to event listener");
        unsafe {
            let flags: u32 = (libc::EPOLLIN) as u32;
            let frontend_epoll_event = &mut libc::epoll_event {
                events: flags,
                u64: fd as u64,
            };

            libc::epoll_ctl(self.epoll_fd, libc::EPOLL_CTL_ADD, fd, frontend_epoll_event);
        }
    }

    fn ready_events(&self) -> (i32, Vec<libc::epoll_event>) {
        let mut events: Vec<libc::epoll_event> = Vec::with_capacity(200);
        events.clear();
        let res = unsafe {
            let res: i32 = libc::epoll_wait(
                self.epoll_fd,
                events.as_mut_ptr() as *mut libc::epoll_event,
                100,
                100,
            );

            if res > 0 {
                events.set_len(res as usize);
            }
            res
        };
        (res, events)
    }
}

#[derive(Debug, Clone)]
struct Backend<'a> {
    host: &'a str,
    port: i32,
}

impl<'a> Backend<'a> {
    fn new(host: &'a str, port: i32) -> Self {
        Backend { host, port }
    }

    fn get_connection(&self) -> BackendStream {
        let address = format!("{}:{}", self.host, self.port);
        let stream = BackendStream::new(address.as_str());
        stream
    }
}

struct LoadBalancer<'a> {
    listen_port: i32,
    backends: VecDeque<Backend<'a>>,
    epoll: Epoll,
    fd_map: HashMap<i32, i32>,
    end_points_map: HashMap<i32, Box<dyn EndPoint>>,
}

use core::fmt::Debug;
impl Debug for LoadBalancer<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "LoadBalancer {{
    port: {}
    backends: {:?}
    epoll_fd: {:?}
    fd_map {:?}
}}",
            self.listen_port, self.backends, self.epoll, self.fd_map
        )
    }
}

impl<'a> LoadBalancer<'a> {
    fn new(listen_port: i32, backends: VecDeque<Backend<'a>>) -> Self {
        LoadBalancer {
            listen_port,
            backends,
            epoll: Epoll::new(),
            fd_map: HashMap::new(),
            end_points_map: HashMap::new(),
        }
    }

    fn cleanup_endpoint(&mut self, fd1: libc::c_int, fd2: libc::c_int) {
        self.remove_endpoint_fd(fd1);
        self.remove_endpoint_fd(fd2);
    }

    fn remove_endpoint_fd(&mut self, fd: libc::c_int) {
        self.epoll_remove_from_interest_list(fd);
        self.fd_map.remove(&fd);
        self.end_points_map.remove(&fd);
    }

    fn epoll_remove_from_interest_list(&mut self, fd: libc::c_int) {
        self.epoll.remove_from_interest_list(fd);
    }

    fn epoll_add_to_interest_list(&mut self, fd: libc::c_int) {
        self.epoll.add_to_interest_list(fd);
    }

    //Select backend in Round-Robin fashion
    fn select_backend(&mut self) -> Backend<'_> {
        match self.backends.pop_front() {
            Some(backend) => {
                self.backends.push_back(backend.clone());
                backend
            }
            None => {
                panic!("Something terrible happened!");
            }
        }
    }

    fn add_connection(&mut self, frontend_fd: libc::c_int) {
        let backend = self.select_backend().get_connection();
        println!(
            "Connection fe_fd:{} be_fd:{}",
            frontend_fd,
            backend.get_fd()
        );
        self.epoll_add_to_interest_list(frontend_fd);
        self.epoll_add_to_interest_list(backend.get_fd());
        let frontend = FrontendStream::new(frontend_fd);
        self.fd_map.insert(backend.get_fd(), frontend_fd);
        self.fd_map.insert(frontend_fd, backend.get_fd());
        self.end_points_map
            .insert(backend.get_fd(), Box::new(backend));
        self.end_points_map.insert(frontend_fd, Box::new(frontend));
    }

    fn handle_endpoints(&mut self) {
        match self.epoll.ready_events() {
            (res, events) if res >= 0 => {
                if events.len() > 0 {
                    let mut do_cleanup = true;
                    for event in events.iter() {
                        match self.fd_map.get(&(event.u64 as i32)) {
                            Some(write_to_fd) => {
                                let read_from_fd = event.u64 as i32;
                                let read_from_stream =
                                    match self.end_points_map.get_mut(&read_from_fd) {
                                        Some(read_from_stream) => read_from_stream,
                                        None => {
                                            println!("1a. Cleanup");
                                            //Since we have cleaned up both endpoints,
                                            //If event is not handled from this fd, simply ignore that and continue!!!
                                            continue;
                                        }
                                    };
                                let mut buf: [u8; 16384] = [0; 16384];
                                match read_from_stream.read(&mut buf) {
                                    Ok(read_bytes) if read_bytes > 0 => {
                                        let write_to_stream =
                                            self.end_points_map.get_mut(&write_to_fd).unwrap();

                                        match write_to_stream.write(&buf[..read_bytes]) {
                                            Ok(res) if res > 0 => {
                                                do_cleanup = false;
                                            }
                                            Ok(_) => {
                                                println!("3a. Cleanup");
                                            }
                                            Err(_err) => {
                                                println!(
                                                    "3b. Cleanup = {} {}\nError = {}",
                                                    write_to_stream.get_fd(),
                                                    do_cleanup,
                                                    _err.to_string()
                                                );
                                            }
                                        }
                                    }
                                    Ok(_) => {
                                        println!("2a. Cleanup = {} {}", read_from_fd, do_cleanup);
                                    }
                                    Err(_err) => {
                                        println!(
                                            "2b. Cleanup = {} {}\nError = {}",
                                            read_from_fd,
                                            do_cleanup,
                                            _err.to_string()
                                        );
                                    }
                                };

                                if do_cleanup {
                                    self.cleanup_endpoint(*write_to_fd, read_from_fd);
                                }
                            }
                            None => {}
                        }
                    }
                }
            }
            (_, _) => {
                println!("No events");
            }
        };
    }

    fn start(&mut self) -> std::io::Result<()> {
        println!("Starting Listener!!! on Port {}", self.listen_port);
        unsafe {
            match libc::socket(libc::AF_INET, libc::SOCK_STREAM, 0) {
                fd if fd > 0 => {
                    let opt = 1;
                    match libc::setsockopt(
                        fd,
                        libc::SOL_SOCKET,
                        libc::SO_REUSEADDR | libc::SO_REUSEPORT,
                        &opt as *const _ as *const libc::c_void,
                        std::mem::size_of_val(&opt) as libc::socklen_t,
                    ) {
                        res if !(res < 0) => {
                            println!("Socket created {:x}", fd);

                            let res = libc::fcntl(fd, libc::F_SETFL, libc::SOCK_NONBLOCK);
                            if res < 0 {
                                println!("Set NON-BLOCKING failed {}", res);
                                libc::exit(res);
                            }

                            let port_lower = (self.listen_port & 0xff) as i8;
                            let port_higher = ((self.listen_port >> 8) & 0xff) as i8;

                            let mut socket_addr = libc::sockaddr {
                                sa_family: libc::AF_INET as u16,
                                sa_data: [
                                    port_higher,
                                    port_lower,
                                    0,
                                    0,
                                    0,
                                    0,
                                    0,
                                    0,
                                    0,
                                    0,
                                    0,
                                    0,
                                    0,
                                    0,
                                ],
                            };

                            let mut addrlen: libc::socklen_t =
                                std::mem::size_of_val(&socket_addr) as libc::socklen_t;

                            match libc::bind(fd, &socket_addr, addrlen) {
                                res if !(res < 0) => {
                                    println!("Binded!!!");
                                    match libc::listen(fd, 2000) {
                                        0 => {
                                            println!("Listening");
                                            let mut conn_count = 0;
                                            loop {
                                                match libc::accept(
                                                    fd,
                                                    &mut socket_addr,
                                                    &mut addrlen,
                                                ) {
                                                    frontend_fd if !(frontend_fd < 0) => {
                                                        self.add_connection(frontend_fd);
                                                        conn_count += 1;
                                                        println!(
                                                            "Connection count - {}",
                                                            conn_count
                                                        );
                                                    }
                                                    _ => {}
                                                }
                                                self.handle_endpoints();
                                            }
                                        }
                                        res => {
                                            println!(
                                                "Listen error: {}, Error: {:?}",
                                                res,
                                                *libc::__errno_location()
                                            );
                                        }
                                    }
                                }
                                res => {
                                    println!(
                                        "Failed to bind: {}, Error: {:?}",
                                        res,
                                        *libc::__errno_location()
                                    );
                                }
                            }
                        }
                        res => {
                            println!("Set Socket options failed: {}", res);
                        }
                    }
                }
                res => {
                    println!("Create Socket failed: {}", res);
                }
            }
        }
        Ok(())
    }
}

fn main() -> std::io::Result<()> {
    let backend_db_1 = Backend::new("mysql-db-1", 3306);
    let backend_db_2 = Backend::new("mysql-db-2", 3306);

    let mut backends = VecDeque::new();
    backends.push_back(backend_db_1);
    backends.push_back(backend_db_2);
    let mut my_lb = LoadBalancer::new(3306, backends);
    println!("{:#?}", my_lb);
    my_lb.start()
}
