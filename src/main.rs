extern crate clap;

mod anyerror;
mod rmdrop;

use anyerror::AnyError;
use rmdrop::RemoveOnDrop;

use clap::{App, AppSettings, Arg, SubCommand};

use std::process::{self, Command};
use std::fs;

fn run() -> Result<(), AnyError> {
    let matches = App::new("cargo at")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Jonas Schievink <jonas@schievink.net>")
        .about("Sync source files and run Cargo on a remote host")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(SubCommand::with_name("at")
            .setting(AppSettings::TrailingVarArg)
            .arg(Arg::with_name("remote")
                .takes_value(true)
                .required(true))
            .arg(Arg::with_name("port")
                .short("p")
                .long("port")
                .takes_value(true))
            .arg(Arg::with_name("subcommand")
                .takes_value(true)
                .required(true)
                .multiple(true)))
        .get_matches();

    let matches = matches.subcommand_matches("at").unwrap();
    let remote: &str = matches.value_of("remote").unwrap();
    let subcmd: Vec<&str> = matches.values_of("subcommand").unwrap().collect();
    let port: Option<u16> = match matches.value_of("port") {
        Some(port) => Some(try!(port.parse())),
        None => None,
    };

    // Temporary sanity check:
    if fs::metadata(".gitignore").is_err() {
        return Err(AnyError::from(".gitignore doesn't exist (please only run this command in the \
            root of your repo)"));
    }

    // Start master connection (this will fork to background, so we can continue after the user
    // authenticates)
    const CONTROL_FILE: &'static str = ".cargo_at_ssh_control";

    if fs::metadata(CONTROL_FILE).is_ok() {
        // Control file already exists, remove it
        try!(fs::remove_file(CONTROL_FILE));
    }

    // Remove the control file after we're done, so we don't clutter the directory.
    // FIXME: This will probably fail on Windows (but it's questionable whether this can work at all
    // on Windows)
    let _remove_on_drop = RemoveOnDrop::new(CONTROL_FILE);
    let mut cmd = Command::new("ssh");
    if let Some(port) = port {
        cmd.args(&["-p", &port.to_string()]);
    }

    cmd.arg("-fNM")
       .args(&["-S", CONTROL_FILE])
       .arg(remote);    // user@host

    let status = try!(cmd.status());
    if !status.success() {
        return Err("failed to spawn SSH master connection".into());
    }

    println!("[[[ Synchronizing project with remote ]]]");

    // FIXME Properly generate the cache dir (based on project name/repo name/topmost pwd dir)
    let cache_dir = ".cache/cargo-at/";

    // FIXME Catch Ctrl+C and remove the temp file above (might kill us when rsync is running)
    // Otherwise the SSH control file stays around - We can also try to just create it somewhere
    // else.
    let status = try!(Command::new("rsync")
        .arg("--exclude=/.git")
        .arg("--filter=:- .gitignore")
        .arg("-aP")
        .arg("-e")
        .arg(format!("ssh -S {}", CONTROL_FILE))
        .arg(".")
        .arg(format!("{}:{}", remote, cache_dir))
        .status());

    if !status.success() {
        return Err("failed to synchronize project with rsync".into());
    }

    println!("[[[ Running command on remote host ]]]");

    // cd to the sync'd directory and run cargo
    let status = try!(Command::new("ssh")
        .arg("-t")  // allocate a pty so we can have graphical stuff going on
        .args(&["-S", CONTROL_FILE])
        .arg(remote)
        .arg(format!("cd '{}'; cargo {}", cache_dir, subcmd.join(" ")))
        .status());

    if !status.success() {
        return Err("command exited with error".into());
    }

    Ok(())
}

fn main() {
    match run() {
        Ok(()) => {},
        Err(AnyError(string)) => {
            println!("error: {}", string);
            process::exit(1);
        }
    }
}
