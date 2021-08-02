mod args;

use std::process::exit;

fn main() {
    let args = args::get();

    let pid_fd = unsafe {
        /*
            There's no wrapper for pidfd_open, so we make the syscall directly.
            The flags argument should currently be set to 0. See pidfd_open(2)
            for more info.
        */
        libc::syscall(libc::SYS_pidfd_open, args.pid, 0)
    };

    if pid_fd == -1 {
        eprintln!("{}: can't wait for {}: {}",
            args::get_program_name(),
            args.pid,
            std::io::Error::last_os_error()
        );

        exit(1);
    }

    let poller = polling::Poller::new().unwrap();

    poller.add(
        &(pid_fd as std::os::unix::io::RawFd),
        polling::Event::readable(4) // Arbitrary number to be used as a key
    ).unwrap();

    poller.wait(&mut Vec::with_capacity(1), None).unwrap();

    if unsafe {
        libc::close(pid_fd as libc::c_int)
    } == libc::EXIT_SUCCESS {
        exit(0);
    } else {
        eprintln!("{}: couldn't close FD {}",
            args::get_program_name(),
            pid_fd
        );

        exit(1);
    }
}
