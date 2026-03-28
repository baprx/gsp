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

fn get_current_project() -> Option<String> {
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
        warn!(
            "The configuration file doesn't exist: {}",
            config_file_path.display()
        );
        return None;
    };
    let config: Ini = Ini::load_from_file(config_file_path).ok()?;
    let core = config.section(Some("core"))?;
    core.get("project").map(|p| p.to_string())
}

fn refresh_projects(_verbose: bool) {
    let cache_file_path = utils::home_dir()
        .join(".cache")
        .join("gsp")
        .join("projects.json");
    fs::create_dir_all(cache_file_path.parent().unwrap())
        .expect("Error while creating the cache directory.");

    let output = std::process::Command::new("gcloud")
        .args([
            "projects",
            "list",
            "--format",
            "json(name,projectId,projectNumber)",
        ])
        .output();

    let output = match output {
        Ok(out) => out,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            panic!("Command 'gcloud' not found, please make sure it is installed and in the PATH.");
        }
        Err(e) => panic!("Error while running gcloud command: {:?}", e),
    };

    if !output.status.success() {
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
        panic!("Failed to refresh project cache. Run 'gcloud auth login' if not authenticated.");
    }

    let projects_json = output.stdout;
    if projects_json.is_empty() {
        panic!("gcloud returned no projects. Ensure you have access to at least one Google Cloud project.");
    }

    fs::write(&cache_file_path, projects_json).expect("Failed to write cache file.");
    info!("The cache was successfully refreshed.")
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
    if cache_str.is_empty() {
        warn!("Cache file is empty, attempting to refresh...");
        refresh_projects(verbose);
        let cache_str = match fs::read_to_string(&cache_file_path) {
            Ok(content) => content,
            Err(error) => panic!("Problem opening the file: {:?}", error),
        };
        if cache_str.is_empty() {
            panic!("Cache file is still empty. Run 'gcloud auth login' to authenticate first.")
        }
        serde_json::from_str(cache_str.as_str()).expect("JSON was not well-formatted")
    } else {
        serde_json::from_str(cache_str.as_str()).expect("JSON was not well-formatted")
    }
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
    if projects.is_empty() {
        panic!("No projects found. Run 'gcloud auth login' to authenticate and 'gsp refresh' to fetch projects.");
    }
    let options = SkimOptionsBuilder::default()
        .query(project_from_user)
        .preview("")
        .select_1(true)
        .build()
        .unwrap();
    let (tx, rx): (SkimItemSender, SkimItemReceiver) = unbounded();
    let items: Vec<Arc<dyn SkimItem>> = projects
        .iter()
        .map(|p| Arc::new(p.to_owned()) as Arc<dyn SkimItem>)
        .collect();
    tx.send(items).unwrap();
    drop(tx);
    let selected_items = Skim::run_with(options, Some(rx))
        .map(|out| match out.is_abort {
            true => process::exit(0),
            false => out.selected_items,
        })
        .unwrap_or_default();

    if selected_items.is_empty() {
        panic!("No project selected. Please select a project from the list.");
    }

    // Get the text representation of the selected item (which is the project_id)
    selected_items.first().unwrap().text().to_string()
}

fn project_switch(verbose: bool, refresh: bool, project_from_user: Vec<String>) {
    if refresh {
        refresh_projects(verbose)
    }
    let projects = load_cache(verbose);
    let project_id = find_match(projects, project_from_user.join(" "));
    let current_project = get_current_project();
    if current_project.as_ref() != Some(&project_id) {
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
        info!("Already on project: <b>{}</b>", project_id)
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
        Some(cmd::Commands::Current) => match get_current_project() {
            Some(project) => info!("Current project: <green><b>{}</b></>", project),
            None => warn!(
                "No current project set. Run 'gcloud config set project PROJECT_ID' to set one."
            ),
        },
        Some(cmd::Commands::Refresh) => refresh_projects(is_verbose),
        Some(cmd::Commands::List) => list_projects(is_verbose, cli.refresh),
        Some(cmd::Commands::GenerateCompletions { shell }) => {
            let mut command = cmd::command();
            cmd::print_completions(shell.to_owned(), &mut command);
        }
        None => project_switch(is_verbose, cli.refresh, cli.project.unwrap_or_default()),
    }
}
