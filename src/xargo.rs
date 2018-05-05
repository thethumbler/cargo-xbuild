use std::path::{Display, PathBuf};
use std::process::{Command, ExitStatus};
use std::mem;
use std::io::{self, Write};
use std::path::Path;

use rustc_version::VersionMeta;

use CompilationMode;
use cargo::Rustflags;
use cli::Args;
use errors::*;
use extensions::CommandExt;
use flock::{FileLock, Filesystem};

pub fn run(
    args: &Args,
    cmode: &CompilationMode,
    rustflags: Rustflags,
    home: &Home,
    meta: &VersionMeta,
    verbose: bool,
) -> Result<ExitStatus> {
    let mut cmd = Command::new("cargo");
    cmd.arg("build");
    cmd.args(args.all());

    let flags = rustflags.for_xargo(home);
    if verbose {
        writeln!(io::stderr(), "+ RUSTFLAGS={:?}", flags).ok();
    }
    cmd.env("RUSTFLAGS", flags);

    let locks = (home.lock_ro(&meta.host), home.lock_ro(cmode.triple()));

    let status = cmd.run_and_get_status(verbose)?;

    mem::drop(locks);

    Ok(status)
}

pub struct Home {
    path: Filesystem,
}

impl Home {
    pub fn display(&self) -> Display {
        self.path.display()
    }

    fn path(&self, triple: &str) -> Filesystem {
        self.path.join("lib/rustlib").join(triple)
    }

    pub fn lock_ro(&self, triple: &str) -> Result<FileLock> {
        let fs = self.path(triple);

        fs.open_ro(".sentinel", &format!("{}'s sysroot", triple))
            .chain_err(|| {
                format!("couldn't lock {}'s sysroot as read-only", triple)
            })
    }

    pub fn lock_rw(&self, triple: &str) -> Result<FileLock> {
        let fs = self.path(triple);

        fs.open_rw(".sentinel", &format!("{}'s sysroot", triple))
            .chain_err(|| {
                format!("couldn't lock {}'s sysroot as read-only", triple)
            })
    }
}

pub fn home(target_directory: &Path) -> Result<Home> {
    let mut p = PathBuf::from(target_directory);
    p.push("sysroot");

    Ok(Home {
        path: Filesystem::new(p),
    })
}
