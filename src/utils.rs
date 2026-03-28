use std::fs::File;
use std::path::PathBuf;
use std::process::{Command, Stdio};

pub fn home_dir() -> PathBuf {
    let home = std::env::var("HOME").expect("$HOME not found");
    PathBuf::from(home)
}

pub fn run_gcloud(_verbose: bool, output: Option<File>, args: Vec<&str>) -> bool {
    let result = Command::new("gcloud")
        .args(args)
        .stdout(if let Some(file) = output {
            file.into()
        } else {
            Stdio::inherit()
        })
        .stderr(Stdio::piped())
        .spawn();
    let mut child = match result {
        Ok(_) => result.unwrap(),
        Err(ref e) if e.kind() == std::io::ErrorKind::NotFound => {
            panic!("Command 'gcloud' not found, please make sure it is installed and in the PATH.");
        }
        Err(e) => panic!("Error while running gcloud command: {:?}", e),
    };
    let exit_status = match child.try_wait() {
        Ok(Some(status)) => status,
        Ok(None) => child.wait().unwrap(),
        Err(e) => panic!("Error attempting to wait: {e}"),
    };

    if !exit_status.success() {
        if let Ok(stderr) = std::io::read_to_string(child.stderr.take().unwrap()) {
            eprintln!("{}", stderr);
        }
    }

    exit_status.success()
}
