use std::fs::File;
use std::path::PathBuf;
use std::process::{Command, Stdio};

pub fn home_dir() -> PathBuf {
    let home = std::env::var("HOME").expect("$HOME not found");
    PathBuf::from(home)
}

pub fn run_gcloud(verbose: bool, output: Option<File>, args: Vec<&str>) -> bool {
    let result = Command::new("gcloud")
        .args(args)
        .stdout(if let Some(file) = output {
            file.into()
        } else {
            Stdio::inherit()
        })
        .stderr(if verbose {
            Stdio::inherit()
        } else {
            Stdio::null()
        })
        .spawn();
    let mut child = match result {
        Ok(_) => result.unwrap(),
        Err(ref e) if e.kind() == std::io::ErrorKind::NotFound => {
            panic!("Command 'gcloud' not found, please make sure it is installed and in the PATH.");
        }
        Err(e) => panic!("Error while running gcloud command: {:?}", e),
    };
    match child.try_wait() {
        Ok(Some(status)) => status.success(),
        Ok(None) => {
            let res = child.wait();
            res.unwrap().success()
        }
        Err(e) => panic!("Error attempting to wait: {e}"),
    }
}
