use libc::rlimit;

pub fn setup_process_env_for_exec() -> std::io::Result<()> {
    unsafe {
        libc::setrlimit(libc::RLIMIT_AS, &rlimit {rlim_cur: 10485760, rlim_max: 10485760});
        libc::setrlimit(libc::RLIMIT_CORE, &rlimit {rlim_cur: 0, rlim_max: 0});
        libc::setrlimit(libc::RLIMIT_DATA, &rlimit {rlim_cur: 5242880, rlim_max: 5242880});
        libc::setrlimit(libc::RLIMIT_FSIZE, &rlimit {rlim_cur: 0, rlim_max: 0});
        libc::setrlimit(libc::RLIMIT_NPROC, &rlimit {rlim_cur: 2, rlim_max: 2});
        libc::setrlimit(libc::RLIMIT_STACK, &rlimit {rlim_cur: 2097152, rlim_max: 2097152});
        libc::setrlimit(libc::RLIMIT_NOFILE, &rlimit {rlim_cur: 4, rlim_max: 4});

        libc::setpriority(libc::PRIO_PROCESS, 0, 19);
    }

    Ok(())
}