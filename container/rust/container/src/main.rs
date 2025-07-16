
//extern crate libc;
//extern crate serde;
use libc::{
    CLONE_NEWIPC, CLONE_NEWNET, CLONE_NEWNS, CLONE_NEWPID, CLONE_NEWUSER, CLONE_NEWUTS, SIGCHLD,
};
use serde::{Serialize, Deserialize};
use serde_json;
use std::ffi::CString;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::Error;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Cmd {
    executable: String,
    args: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct MountBinds {
    source: String,
    target: String,
    read_only: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct UserMap {
    uid: u16,
    gid: u16,
}

#[derive(Serialize, Deserialize,Debug, Clone)]
struct NsConfig {
    hostname: Option<String>,
    program: Cmd,
    user_map: Option<UserMap>,
    mount_bindings: Option<Vec<MountBinds>>,
    chroot: String,
}

impl NsConfig {
    fn from_file(config_file_name: String) -> NsConfig {
        let error_message = format!("Config file not found: {}", config_file_name);
        let mut file = File::open(config_file_name).expect(error_message.as_str());
        let mut server_config = String::new();
        file.read_to_string(&mut server_config).unwrap();
        let ns_config: NsConfig = serde_json::from_str(&mut server_config).unwrap();
        ns_config
    }

    fn program(&self) -> Cmd {
        self.program.clone()
    }

    fn bind_map(&self) -> Option<Vec<MountBinds>> {
        self.mount_bindings.clone()
    }

    fn user_map(&self) -> Option<UserMap> {
        self.user_map.clone()
    }

    fn chroot(&self) -> String {
        self.chroot.clone()
    }

    fn hostname(&self) -> Option<String> {
        self.hostname.clone()
    }
}

fn create_namespace_mapping(UserMap { uid, gid }: UserMap) {
    let pid = "self";
    let setgroups_file = format!("/proc/{}/setgroups", pid);
    let uid_map_file = format!("/proc/{}/uid_map", pid);
    let gid_map_file = format!("/proc/{}/gid_map", pid);
    let mut setgroups = OpenOptions::new()
        .write(true)
        .read(true)
        .open(&setgroups_file)
        .unwrap();
    setgroups
        .write_all(b"deny")
        .expect("Unable to write to setgroups");
    setgroups.flush().unwrap();
    drop(setgroups);
    let mut uid_map = OpenOptions::new()
        .read(true)
        .write(true)
        .open(uid_map_file)
        .unwrap();
    let uid_map_content = format!("0 {} 1", uid);
    uid_map
        .write_all(uid_map_content.as_bytes())
        .expect("Unable to write UID map");
    uid_map.flush().unwrap();
    drop(uid_map);
    let mut gid_map = OpenOptions::new()
        .read(true)
        .write(true)
        .open(gid_map_file)
        .unwrap();
    let gid_map_content = format!("0 {} 1", gid);
    gid_map
        .write_all(gid_map_content.as_bytes())
        .expect("Unable to write GID map");
    gid_map.flush().unwrap();
    drop(gid_map);
}

fn setup_mount_binds(mount_bind_maps: Vec<MountBinds>) {
    unsafe {
        //Bind mounts before chroot
        for bm in mount_bind_maps {
            let res = libc::mount(
                CString::new(bm.source.as_bytes()).unwrap().as_ptr(),
                CString::new(bm.target.as_bytes()).unwrap().as_ptr(),
                std::ptr::null(),
                libc::MS_BIND, //Note readonly flag wont work here - handle RO binds later
                std::ptr::null(),
                );
            if res != 0 {
                println!(
                    "bind mount failed with an error -> {:?}",
                    Error::last_os_error()
                    );
            } else {
                //Handle RO binds
                if let Some(true) = bm.read_only {
                    let res = libc::mount(
                        std::ptr::null(),
                        CString::new(bm.target.as_bytes()).unwrap().as_ptr(),
                        std::ptr::null(),
                        libc::MS_RDONLY | libc::MS_BIND | libc::MS_REMOUNT,
                        std::ptr::null(),
                        );
                    if res != 0 {
                        println!(
                            "bind readonly switch failed with an error -> {:?}",
                            Error::last_os_error()
                            );
                    }
                }
            }
        }
    }
}

fn setup_chroot_env(chroot_path: String) {
    unsafe {
        let res = libc::chroot(CString::new(chroot_path.as_bytes()).unwrap().as_ptr());
        if res != 0 {
            println!(
                "chroot failed with an error -> {:?}",
                Error::last_os_error()
                );
        }
        let res = libc::chdir(CString::new("/").unwrap().as_ptr());
        if res != 0 {
            println!("chdir failed with an error -> {:?}", Error::last_os_error());
        }
        //Mount proc filesystem
        let res = libc::mount(
            std::ptr::null(),
            CString::new("/proc").unwrap().as_ptr(),
            CString::new("proc").unwrap().as_ptr(),
            0u64,
            std::ptr::null(),
            );
        if res != 0 {
            println!(
                "proc mount failed with an error -> {:?}",
                Error::last_os_error()
                );
        }
        //Mount dev filesystem
        let res = libc::mount(
            std::ptr::null(),
            CString::new("/dev").unwrap().as_ptr(),
            CString::new("tmpfs").unwrap().as_ptr(),
            0u64,
            std::ptr::null(),
            );
        if res != 0 {
            println!(
                "dev mount failed with an error -> {:?}",
                Error::last_os_error()
                );
        }
        //Mount run filesystem
        let res = libc::mount(
            std::ptr::null(),
            CString::new("/run").unwrap().as_ptr(),
            CString::new("tmpfs").unwrap().as_ptr(),
            0u64,
            std::ptr::null(),
            );
        if res != 0 {
            println!(
                "run mount failed with an error -> {:?}",
                Error::last_os_error()
                );
        }
    }
}

fn exec_in_ns(Cmd { executable, args }: Cmd) -> libc::c_int {
    unsafe {
        //First argument will be the program name itself. See execv syscall description
        let mut args_cstring: Vec<CString> = vec![CString::new(executable.as_bytes()).unwrap()];
        for arg in args {
            args_cstring.push(CString::new(arg.as_bytes()).unwrap());
        }
        let mut args_c_char: Vec<*const libc::c_char> =
            args_cstring.iter().map(|arg| arg.as_ptr()).collect();
        args_c_char.push(std::ptr::null()); //NULL terminated
        let res = libc::execv(
            CString::new(executable.as_bytes()).unwrap().as_ptr(),
            args_c_char.as_ptr(),
            );
        if res != 0 {
            println!(
                "Entry point execution failed with an error -> {:?}",
                Error::last_os_error()
                );
        }
        res
    }
}

extern "C" fn setup_ns(ns_config: *mut NsConfig) -> libc::c_int {
    let ns_config: &NsConfig = unsafe { &mut *ns_config };
    //Set hostname
    unsafe {
        if let Some(hostname) = ns_config.hostname() {
            let res = libc::sethostname(
                CString::new(hostname.as_bytes()).unwrap().as_ptr(),
                hostname.as_bytes().len(),
                );
            if res != 0 {
                println!(
                    "proc mount failed with an error -> {:?}",
                    Error::last_os_error()
                    );
            }
        }
        //Do mappings UID and GID mappings
        println!("setting up uid and gid mappings");
        match ns_config.user_map() {
            Some(user_map) => create_namespace_mapping(user_map),
            None => {}
        }

        //Mount bindings
        println!("setting up binds");
        match ns_config.bind_map() {
            Some(bind_map) => setup_mount_binds(bind_map),
            None => {}
        }

        //Chroot and Proc Mount
        println!("setting up chroot");
        //setup_chroot_env(ns_config.chroot());

        //EXECV the program replacing the clone completely
        println!("setting up to execute program");
        exec_in_ns(ns_config.program())
    }
}

fn main() {
    //CLONE and then EXEC entry point in CLONE
    unsafe {
        let mut ns_config = NsConfig::from_file("container.json".to_string());
        let mut nstack = [0u8; 4096];
        let ptr = nstack.as_mut_ptr().offset(nstack.len() as isize);
        let ptr_aligned = ptr.offset((ptr as usize % 16) as isize * -1);
        //CLONE FLAGS for namespace
        let mut ns_flags: libc::c_int =
            CLONE_NEWNS | CLONE_NEWUTS | CLONE_NEWIPC | CLONE_NEWPID | CLONE_NEWNET | SIGCHLD;
        //NEWUSER if user mappings are present
        match ns_config.user_map {
            Some(_) => ns_flags = ns_flags | CLONE_NEWUSER,
            None => {}
        }
        let pid = libc::clone(
            std::mem::transmute(setup_ns as extern "C" fn(*mut NsConfig) -> libc::c_int),
            ptr_aligned as *mut libc::c_void,
            ns_flags,
            &mut ns_config as *mut _ as *mut libc::c_void,
            );
        //If NEWUSER is not used/user mappings are not provided then you got to be root to perform clone
        if pid != 0 {
            println!("Program PID -> {}", pid);

            let mut rusage: libc::rusage = std::mem::MaybeUninit::uninit().assume_init();
            let mut status: i32 = 0;
            let options: i32 = 0;

            let res = libc::wait4(pid, &mut status, options, &mut rusage);
            println!("CN WAIT RESULT -> {}", res);
            println!("CN RUSAGE -> {:#?}", rusage);
            println!("CN WAIT STATUS -> {}", status);
            if status != 0 {
                println!(
                    "CN WAIT ERROR WHILE RUNNING -> {:?}",
                    Error::last_os_error()
                    );
            }
        } // no else
    }
}
