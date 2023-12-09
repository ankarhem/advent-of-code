use advent_of_code::template::commands::{all, download, read, scaffold, solve};
use args::{parse, AppArguments};

mod args {
    use std::process;

    use advent_of_code::{Day, Year};

    pub enum AppArguments {
        Download {
            year: Year,
            day: Day,
        },
        Read {
            year: Year,
            day: Day,
        },
        Scaffold {
            year: Year,
            day: Day,
        },
        Solve {
            year: Year,
            day: Day,
            release: bool,
            time: bool,
            submit: Option<u8>,
        },
        All {
            release: bool,
            time: bool,
        },
    }

    pub fn parse() -> Result<AppArguments, Box<dyn std::error::Error>> {
        let mut args = pico_args::Arguments::from_env();

        let app_args = match args.subcommand()?.as_deref() {
            Some("all") => AppArguments::All {
                release: args.contains("--release"),
                time: args.contains("--time"),
            },
            Some("download") => AppArguments::Download {
                year: args.free_from_str()?,
                day: args.free_from_str()?,
            },
            Some("read") => AppArguments::Read {
                year: args.free_from_str()?,
                day: args.free_from_str()?,
            },
            Some("scaffold") => AppArguments::Scaffold {
                year: args.free_from_str()?,
                day: args.free_from_str()?,
            },
            Some("solve") => AppArguments::Solve {
                year: args.free_from_str()?,
                day: args.free_from_str()?,
                release: args.contains("--release"),
                submit: args.opt_value_from_str("--submit")?,
                time: args.contains("--time"),
            },
            Some(x) => {
                eprintln!("Unknown command: {x}");
                process::exit(1);
            }
            None => {
                eprintln!("No command specified.");
                process::exit(1);
            }
        };

        let remaining = args.finish();
        if !remaining.is_empty() {
            eprintln!("Warning: unknown argument(s): {remaining:?}.");
        }

        Ok(app_args)
    }
}

fn main() {
    match parse() {
        Err(err) => {
            eprintln!("Error: {err}");
            std::process::exit(1);
        }
        Ok(args) => match args {
            AppArguments::All { release, time } => all::handle(release, time),
            AppArguments::Download { year, day } => download::handle(year, day),
            AppArguments::Read { year, day } => read::handle(day),
            AppArguments::Scaffold { year, day } => scaffold::handle(year, day),
            AppArguments::Solve {
                year,
                day,
                release,
                time,
                submit,
            } => solve::handle(day, release, time, submit),
        },
    };
}
