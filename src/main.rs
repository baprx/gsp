mod cmd;
mod utils;
use ini::Ini;
use log::LevelFilter;
use prettytable::{row, Table};
use serde::Deserialize;
use simplelog::{
    debug, info, trace, warn, Color, ColorChoice, ConfigBuilder, Level, TermLogger, TerminalMode,
};
use skim::prelude::*;
use std::fs;
use std::fs::File;
use std::path::PathBuf;
use std::process;
use std::str::FromStr;

#[derive(Deserialize, Debug, Clone)]
struct Projects {
    #[serde(alias = "name")]
    name: String,
    #[serde(alias = "projectId")]
    project_id: String,
    #[serde(alias = "projectNumber")]
    project_number: String,
}

impl SkimItem for Projects {
    fn text(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.project_id)
    }

    fn preview(&self, _context: PreviewContext) -> ItemPreview {
        let preview = format!(
            r#"
# Project {}
Name:   {}
Number: {}
"#,
            self.project_id, self.name, self.project_number
        );
        ItemPreview::Text(preview)
    }
}

fn get_current_project() -> String {
    let active_config_file = utils::home_dir()
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
    let config_file_path = utils::home_dir()
        .join(".config")
        .join("gcloud")
        .join("configurations")
        .join(format!("config_{}", active_profile));
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
    let cache_file_path = utils::home_dir()
        .join(".cache")
        .join("gsp")
        .join("projects.json");
    fs::create_dir_all(cache_file_path.parent().unwrap())
        .expect("Error while creating the cache directory.");
    let cache_file = File::create(cache_file_path).expect("Failed to open the cache file.");

    let success = utils::run_gcloud(
        verbose,
        Some(cache_file),
        vec![
            "projects",
            "list",
            "--format",
            "json(name,projectId,projectNumber)",
        ],
    );
    if success {
        info!("The cache was successfully refreshed.")
    } else {
        panic!("Error while refreshing the cache.")
    };
}

fn load_cache(verbose: bool) -> Vec<Projects> {
    let cache_file_path = utils::home_dir()
        .join(".cache")
        .join("gsp")
        .join("projects.json");
    if !PathBuf::from(&cache_file_path).exists() {
        refresh_projects(verbose)
    }
    let cache_str: String = match fs::read_to_string(&cache_file_path) {
        Ok(content) => content,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
    serde_json::from_str(cache_str.as_str()).expect("JSON was not well-formatted")
}

fn list_projects(verbose: bool, refresh: bool) {
    if refresh {
        refresh_projects(verbose)
    }
    let projects = load_cache(verbose);
    let mut table = Table::new();

    table.set_titles(row!["Project ID", "Project number", "Project name"]);
    for p in &projects {
        table.add_row(row![p.project_id, p.project_number, p.name]);
    }

    table.printstd();
}

fn find_match(projects: Vec<Projects>, project_from_user: String) -> String {
    let options = SkimOptionsBuilder::default()
        .query(Some(project_from_user))
        .preview(Some("".to_string()))
        .select_1(true)
        .build()
        .unwrap();
    let (tx, rx): (SkimItemSender, SkimItemReceiver) = unbounded();
    for p in &projects {
        tx.send(Arc::new(p.to_owned())).unwrap();
    }
    drop(tx);
    let selected_items = Skim::run_with(&options, Some(rx))
        .map(|out| match out.is_abort {
            true => process::exit(0),
            false => out.selected_items,
        })
        .unwrap_or_default()
        .iter()
        .map(|selected_item| {
            (**selected_item)
                .as_any()
                .downcast_ref::<Projects>()
                .unwrap()
                .to_owned()
        })
        .collect::<Vec<Projects>>();

    selected_items.first().unwrap().project_id.clone()
}

fn project_switch(verbose: bool, refresh: bool, project_from_user: Vec<String>) {
    if refresh {
        refresh_projects(verbose)
    }
    let projects = load_cache(verbose);
    let project_id = find_match(projects, project_from_user.join(" "));
    if project_id != get_current_project() {
        let success = utils::run_gcloud(
            verbose,
            None,
            vec!["config", "set", "project", project_id.as_str()],
        );
        if success {
            info!("Successfully switched to {}.", project_id)
        } else {
            panic!("Error while switching project.")
        };
    } else {
        info!(
            "The current and target project ID are the same: <b>{}</b>",
            project_id
        )
    }
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
    let is_verbose: bool = LevelFilter::ge(&log_level, &LevelFilter::Debug);
    debug!("Log level: {}", log_level.as_str());
    match &cli.command {
        Some(cmd::Commands::Current) => info!(
            "Current project: <green><b>{}</b></>",
            get_current_project()
        ),
        Some(cmd::Commands::Refresh) => refresh_projects(is_verbose),
        Some(cmd::Commands::List) => list_projects(is_verbose, cli.refresh),
        Some(cmd::Commands::GenerateCompletions { shell }) => {
            let mut command = cmd::command();
            cmd::print_completions(shell.to_owned(), &mut command);
        }
        None => project_switch(is_verbose, cli.refresh, cli.project.unwrap_or_default()),
    }
}
