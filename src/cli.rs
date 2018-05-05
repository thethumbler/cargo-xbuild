use std::env;

pub struct Args {
    all: Vec<String>,
    target: Option<String>,
}

impl Args {
    pub fn all(&self) -> &[String] {
        &self.all
    }

    pub fn target(&self) -> Option<&str> {
        self.target.as_ref().map(|s| &**s)
    }

    pub fn verbose(&self) -> bool {
        self.all
            .iter()
            .any(|a| a == "--verbose" || a == "-v" || a == "-vv")
    }
}

pub fn args() -> Result<(Command, Args), String> {
    let mut args = env::args().skip(1);
    if args.next() != Some("xbuild".into()) {
        Err("must be invoked as cargo subcommand: `cargo xbuild`")?;
    }
    let all = args.collect::<Vec<_>>();
    let command = match all.first().map(|s| s.as_str()) {
        Some("-h") | Some("--help") => Command::Help,
        Some("-v") | Some("--version") => Command::Version,
        _ => Command::Build,
    };

    let mut target = None;
    {
        let mut args = all.iter();
        while let Some(arg) = args.next() {
            if arg == "--target" {
                target = args.next().map(|s| s.to_owned());
            } else if arg.starts_with("--target=") {
                target = arg.splitn(2, '=').nth(1).map(|s| s.to_owned());
            }
        }
    }

    let args = Args {
        all: all,
        target: target,
    };
    Ok((command, args))
}

#[derive(Clone, PartialEq)]
pub enum Command {
    Build,
    Help,
    Version,
}
