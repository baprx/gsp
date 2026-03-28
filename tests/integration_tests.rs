use std::fs;
use std::path::PathBuf;
use std::process::Command;
use tempfile::TempDir;

fn setup_test_env() -> (TempDir, PathBuf, PathBuf) {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let cache_dir = temp_dir.path().join(".cache").join("gsp");
    let config_dir = temp_dir.path().join(".config").join("gcloud");
    let configurations_dir = config_dir.join("configurations");

    fs::create_dir_all(&cache_dir).expect("Failed to create cache directory");
    fs::create_dir_all(&configurations_dir).expect("Failed to create config directory");

    let cache_file = cache_dir.join("projects.json");
    let cache_content = r#"[
  {
    "name": "Example Project",
    "projectId": "dummy-example-project",
    "projectNumber": "902838561285"
  }
]"#;
    fs::write(&cache_file, cache_content).expect("Failed to write cache file");

    (temp_dir, cache_dir, config_dir)
}

fn run_gsp(args: &[&str], home: &PathBuf) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_gsp"))
        .args(args)
        .env("HOME", home)
        .output()
        .expect("Failed to execute gsp command")
}

#[test]
fn test_current_no_project_set() {
    let (temp_dir, _, _) = setup_test_env();
    let home = temp_dir.path().to_path_buf();

    let output = run_gsp(&["current"], &home);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stdout.contains("No current project set") || stderr.contains("No current project set"));
}

#[test]
fn test_current_with_project_set() {
    let (temp_dir, _, config_dir) = setup_test_env();
    let home = temp_dir.path().to_path_buf();

    fs::write(config_dir.join("active_config"), "default").expect("Failed to write active_config");

    let config_content = r#"[core]
project = dummy-example-project
"#;
    fs::write(
        config_dir.join("configurations").join("config_default"),
        config_content,
    )
    .expect("Failed to write config file");

    let output = run_gsp(&["current"], &home);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("dummy-example-project"));
}

#[test]
fn test_list() {
    let (temp_dir, _, _) = setup_test_env();
    let home = temp_dir.path().to_path_buf();

    let output = run_gsp(&["list"], &home);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("dummy-example-project"));
    assert!(stdout.contains("Example Project"));
    assert!(stdout.contains("902838561285"));
}

#[test]
fn test_generate_completions_bash() {
    let (temp_dir, _, _) = setup_test_env();
    let home = temp_dir.path().to_path_buf();

    let output = run_gsp(&["generate-completions", "--shell", "bash"], &home);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("complete"));
}

#[test]
fn test_generate_completions_zsh() {
    let (temp_dir, _, _) = setup_test_env();
    let home = temp_dir.path().to_path_buf();

    let output = run_gsp(&["generate-completions", "--shell", "zsh"], &home);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("compdef"));
}

#[test]
fn test_generate_completions_fish() {
    let (temp_dir, _, _) = setup_test_env();
    let home = temp_dir.path().to_path_buf();

    let output = run_gsp(&["generate-completions", "--shell", "fish"], &home);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("complete"));
}

#[test]
fn test_switch_project_already_selected() {
    let (temp_dir, _, config_dir) = setup_test_env();
    let home = temp_dir.path().to_path_buf();

    fs::write(config_dir.join("active_config"), "default").expect("Failed to write active_config");

    let config_content = r#"[core]
project = dummy-example-project
"#;
    fs::write(
        config_dir.join("configurations").join("config_default"),
        config_content,
    )
    .expect("Failed to write config file");

    let output = run_gsp(&["dummy-example-project"], &home);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Already on project"));
}

#[test]
fn test_switch_project_with_partial_match() {
    let (temp_dir, _, config_dir) = setup_test_env();
    let home = temp_dir.path().to_path_buf();

    fs::write(config_dir.join("active_config"), "default").expect("Failed to write active_config");

    let config_content = r#"[core]
project = other-project
"#;
    fs::write(
        config_dir.join("configurations").join("config_default"),
        config_content,
    )
    .expect("Failed to write config file");

    let output = run_gsp(&["dummy"], &home);

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let output_combined = format!("{}{}", stdout, stderr);
    assert!(output_combined.contains("Successfully switched"));
}

#[test]
fn test_help() {
    let (temp_dir, _, _) = setup_test_env();
    let home = temp_dir.path().to_path_buf();

    let output = run_gsp(&["help"], &home);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Simple CLI to switch between gcloud projects"));
}

#[test]
fn test_version() {
    let (temp_dir, _, _) = setup_test_env();
    let home = temp_dir.path().to_path_buf();

    let output = run_gsp(&["--version"], &home);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("gsp"));
}
