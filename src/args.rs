use libc::pid_t;

pub struct Args {
    pub pid: pid_t
}

fn build_args(matches: getopts::Matches) -> Option<Args> {
    Some(
        Args {
            pid: matches.free.get(1)?.parse().ok().and_then(
                |p| if p == 0 {
                    None
                } else {
                    Some(p)
                }
            )? // The first free argument in getopts is program name
        }
    )
}

use getopts::Options;
use std::{ env, process::exit };

pub fn get_program_name() -> String {
    env::args().nth(0).unwrap_or(
        env!("CARGO_PKG_NAME").to_string()
    )
}

pub fn get() -> Args {
    let mut opts = Options::new();

    opts.optflag("h", "help", "display help and exit");
    opts.optflag("v", "version", "output version information and exit");

    if let Ok(matches) = opts.parse(env::args()) {
        if matches.opt_present("help") {
            println!("{}",
                opts.usage(
                    &format!(
                        "Usage: {} [OPTION]... PID\n\
                        Wait for a process to finish.",
                        get_program_name()
                    )
                )
            );

            exit(0);
        } else if matches.opt_present("version") {
            println!("pfin v{}", env!("CARGO_PKG_VERSION"));

            exit(0);
        } else if let Some(args) = build_args(matches) {
            return args;
        }
    }

    eprintln!("{} PID",
        opts.short_usage(&get_program_name())
    );

    exit(1);
}
