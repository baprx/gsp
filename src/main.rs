mod cmd;
use ini::Ini;
use log::LevelFilter;
use simplelog::{
    debug, info, trace, warn, Color, ColorChoice, ConfigBuilder, Level, TermLogger, TerminalMode,
};
use std::fs;
use std::fs::File;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::str::FromStr;

fn home_dir() -> PathBuf {
    let home = std::env::var("HOME").expect("$HOME not found");
    PathBuf::from(home)
}

fn get_current_project() -> String {
    let active_config_file = home_dir()
        .join(".config")
        .join("gcloud")
        .join("active_config");
    trace!(
        "Getting current configuration from {}",
        &active_config_file.display()
    );
    let active_profile: String = if PathBuf::from(&active_config_file).exists() {
        let profile: String = match fs::read_to_string(&active_config_file) {
            Ok(content) => content,
            Err(error) => panic!("Problem opening the file: {:?}", error),
        };
        profile
    } else {
        warn!("The active config file couldn't be found, using default config.");
        "default".to_string()
    };
    let config_file_path = home_dir()
        .join(".config")
        .join("gcloud")
        .join("configurations")
        .join(&format!("config_{}", active_profile));
    if !PathBuf::from(&config_file_path).exists() {
        panic!(
            "The configuration file doesn't exist: {}",
            config_file_path.into_os_string().into_string().unwrap()
        );
    };
    let config: Ini = Ini::load_from_file(config_file_path).unwrap();
    let core = config.section(Some("core")).unwrap();
    let current_project: &str = core.get("project").unwrap();
    current_project.to_string()
}

fn refresh_projects(verbose: bool) {
    let cache_file_path = home_dir().join(".cache").join("gsp").join("projects.json");
    fs::create_dir_all(cache_file_path.parent().unwrap())
        .expect("Error while creating the cache directory.");
    let cache_file = File::create(cache_file_path).expect("Failed to open the cache file.");
    let result = Command::new("gcloud")
        .args(["projects", "list", "--format", "json"])
        .stdout(cache_file)
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
        Err(e) => panic!(
            "Error while refreshing the available projects. Detail: {:?}",
            e
        ),
    };
    let success = match child.try_wait() {
        Ok(Some(status)) => status.success(),
        Ok(None) => {
            let res = child.wait();
            res.unwrap().success()
        }
        Err(e) => panic!("Error attempting to wait: {e}"),
    };
    if success {
        info!("The cache was successfully refreshed.")
    } else {
        panic!("Error while refreshing the cache, try again with a log level >= DEBUG for more detail.")
    };
}

fn project_switch() {
    info!("Switching!")
}

fn main() {
    let cli = cmd::parse();
    let log_level = LevelFilter::from_str(&cli.log_level).unwrap();
    let config = ConfigBuilder::new()
        .set_level_color(Level::Info, Some(Color::Green))
        .set_level_color(Level::Trace, Some(Color::Yellow))
        .set_time_level(LevelFilter::Trace)
        .set_location_level(LevelFilter::Trace)
        .build();
    TermLogger::init(log_level, config, TerminalMode::Mixed, ColorChoice::Auto)
        .expect("Failed to start simplelog");
    debug!("Log level: {}", log_level.as_str());
    match &cli.command {
        Some(cmd::Commands::Current) => info!(
            "Current project: <green><b>{}</b></>",
            get_current_project()
        ),
        Some(cmd::Commands::Refresh) => {
            refresh_projects(LevelFilter::ge(&log_level, &LevelFilter::Debug))
        }
        None => project_switch(),
    }
}
