use std::io;
use std::process::Command;

use colored::*;
use dialoguer::{theme::Theme, MultiSelect, Select};
use reqwest;
use serde::Deserialize;
use serde_json::Value;
use std::error::Error;

struct SpringInitializrData {
    spring_languages: (Vec<String>, Option<usize>),
    spring_boot_versions: (Vec<String>, Option<usize>),
    spring_packagings: (Vec<String>, Option<usize>),
    java_versions: (Vec<String>, Option<usize>),
    java_dependencies: Vec<String>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct VersionInfo<T> {
    #[serde(rename = "type")]
    version_type: String,
    default: String,
    values: Vec<T>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct VersionValue {
    id: String,
    name: String,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct JavaDependency {
    #[serde(rename = "type")]
    version_type: String,
    values: Vec<DependenciesValue>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct DependenciesValue {
    name: String,
    values: Option<Vec<DependencyValue>>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct DependencyValue {
    id: String,
    name: String,
    description: Option<String>,
}

struct CustomTheme;

impl Theme for CustomTheme {
    // Prompt heading
    fn format_prompt(&self, f: &mut dyn std::fmt::Write, prompt: &str) -> std::fmt::Result {
        write!(f, "{} {}", "◉".cyan().bold(), prompt)
    }
}

fn main() {
    println!("{}", "Spring Initializr CLI!".bright_green().bold());
    println!(
        "{}",
        "https://github.com/AlbertLnz/spring-initializr-cli".bright_yellow()
    );
    println!("{}", "Created by AlbertLnz\n".bright_magenta().italic());

    // fetching action_()
    let data = match action_() {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };

    // LANGUAGE
    let (languages, default_language_index) = data.spring_languages;

    let language_selection = Select::with_theme(&CustomTheme)
        .with_prompt("Select the language".cyan().to_string())
        .items(&languages)
        .default(default_language_index.unwrap_or(0))
        .interact()
        .expect("Failed to read selection");

    let spring_language = languages[language_selection].clone();
    println!("{}\n", spring_language);

    // PROJECT
    let options = vec!["Gradle", "Maven"];

    let project_selection = Select::with_theme(&CustomTheme)
        .with_prompt("Select the project:".cyan().to_string())
        .items(&options)
        .default(1)
        .interact()
        .expect("Failed to read selection");

    let project = options[project_selection];
    println!("{}\n", project);

    // SPRING BOOTVERSION
    let (boot_versions, default_boot_index) = data.spring_boot_versions;

    let boot_selection: usize = Select::with_theme(&CustomTheme)
        .with_prompt("Select the Spring Boot version:".cyan().to_string())
        .items(&boot_versions)
        .default(default_boot_index.unwrap_or(0))
        .interact()
        .expect("Failed to read selection");

    let spring_boot_version = boot_versions[boot_selection].clone();
    println!("{}\n", spring_boot_version);

    // PROJECT METADATA
    println!("{}", "Enter the group:".cyan());
    let mut project_group = String::new();
    io::stdin()
        .read_line(&mut project_group)
        .expect("Failed to read input!");
    let project_group: &str = project_group.trim();

    println!("{}", "Enter the name:".cyan());
    let mut project_name = String::new();
    io::stdin()
        .read_line(&mut project_name)
        .expect("Failed to read input!");
    let project_name: &str = project_name.trim();

    println!("{}", "Enter the description:".cyan());
    let mut project_description = String::new();
    io::stdin()
        .read_line(&mut project_description)
        .expect("Failed to read input!");
    let project_description: &str = project_description.trim();

    println!("{}", "Enter the version:".cyan());
    let mut project_version = String::new();
    io::stdin()
        .read_line(&mut project_version)
        .expect("Failed to read input!");
    let project_version: &str = project_version.trim();

    // PACKAGING
    let (spring_packaging, default_spring_packaging_index) = data.spring_packagings;

    let packaging_selection = Select::with_theme(&CustomTheme)
        .with_prompt("Select the packaging:".cyan().to_string())
        .items(&spring_packaging)
        .default(default_spring_packaging_index.unwrap_or(0))
        .interact()
        .expect("Failed to read selection");

    let spring_packaging = spring_packaging[packaging_selection].clone();
    println!("{}\n", spring_packaging);

    // JAVA VERSION
    let (java_version, default_java_version_index) = data.java_versions;

    let version_selection = Select::with_theme(&CustomTheme)
        .with_prompt("Select the Java version:".cyan().to_string())
        .items(&java_version)
        .default(default_java_version_index.unwrap_or(0))
        .interact()
        .expect("Failed to read selection");

    let java_version = java_version[version_selection].clone();
    println!("{}\n", java_version);

    // DEPENDENCIES
    let java_dependency_names = data.java_dependencies;
    let mut selected_dependencies = vec![];

    let java_dependency_selection = MultiSelect::with_theme(&CustomTheme)
        .with_prompt("Select the dependencies:".cyan().to_string())
        .items(&java_dependency_names)
        .interact()
        .expect("Failed to read selection");

    for &index in &java_dependency_selection {
        selected_dependencies.push(java_dependency_names[index].clone());
    }

    println!("Selected dependencies: {:?}\n", selected_dependencies);

    let command = generate_spring_init_command(
        spring_language,
        project,
        spring_boot_version,
        project_group,
        project_name,
        project_description,
        project_version,
        spring_packaging,
        java_version,
        &selected_dependencies,
    );

    execute_command(command);
}

fn action_() -> Result<SpringInitializrData, Box<dyn Error>> {
    let json = fetch_spring_initializr_api()?;
    let spring_language = get_spring_language(json.clone())?;
    let spring_boot_version = get_spring_boot_version(json.clone())?;
    let spring_packaging = get_spring_packaging(json.clone())?;
    let java_version = get_java_version(json.clone())?;
    let java_dependency = get_java_dependency(json)?;

    Ok(SpringInitializrData {
        spring_languages: spring_language,
        spring_boot_versions: spring_boot_version,
        spring_packagings: spring_packaging,
        java_versions: java_version,
        java_dependencies: java_dependency,
    })
}

fn fetch_spring_initializr_api() -> Result<Value, Box<dyn Error>> {
    let url = "https://start.spring.io/metadata/client";
    let response = reqwest::blocking::get(url)?;
    let response_json: Value = response.json()?;
    Ok(response_json)
}

fn get_versions(
    response_json: Value,
    key: &str,
) -> Result<(Vec<String>, Option<usize>), Box<dyn Error>> {
    let version_info: VersionInfo<VersionValue> =
        serde_json::from_value(response_json[key].clone())?;

    let ids: Vec<String> = version_info.values.iter().map(|v| v.id.clone()).collect();
    let names: Vec<String> = version_info.values.iter().map(|v| v.name.clone()).collect();

    let default_index = ids
        .iter()
        .position(|id| id.trim() == version_info.default.trim());

    Ok((names, default_index))
}

fn get_spring_language(
    response_json: Value,
) -> Result<(Vec<String>, Option<usize>), Box<dyn Error>> {
    get_versions(response_json, "language")
}

fn get_spring_boot_version(
    response_json: Value,
) -> Result<(Vec<String>, Option<usize>), Box<dyn Error>> {
    get_versions(response_json, "bootVersion")
}

fn get_spring_packaging(
    response_json: Value,
) -> Result<(Vec<String>, Option<usize>), Box<dyn Error>> {
    get_versions(response_json, "packaging")
}

fn get_java_version(response_json: Value) -> Result<(Vec<String>, Option<usize>), Box<dyn Error>> {
    get_versions(response_json, "javaVersion")
}

fn get_java_dependency(response_json: Value) -> Result<Vec<String>, Box<dyn Error>> {
    let java_depen: JavaDependency = serde_json::from_value(response_json["dependencies"].clone())?;

    let mut ids = Vec::new();

    for value in java_depen.values {
        if let Some(nested_values) = value.values {
            for nested in nested_values {
                ids.push(nested.id);
            }
        }
    }

    Ok(ids)
}

fn generate_spring_init_command(
    spring_language: String,
    project: &str,
    spring_boot_version: String,
    project_group: &str,
    project_name: &str,
    project_description: &str,
    project_version: &str,
    spring_packaging: String,
    java_version: String,
    selected_java_dependencies: &[String],
) -> String {
    format!(
        "spring init --name={} --groupId={} --artifactId={} --version={} --description=\"{}\" \
        --package-name={}.{} --dependencies={} --build={} --type={}-project --java-version={} \
        --language={} --boot-version={} --packaging={} {}",
        project_name,
        project_group,
        project_name,
        project_version,
        project_description,
        project_group.replace('.', ".").to_lowercase(),
        project_name,
        selected_java_dependencies.join(","),
        project.to_lowercase(),
        project.to_lowercase(),
        java_version,
        spring_language.to_lowercase(),
        spring_boot_version,
        spring_packaging.to_lowercase(),
        project_name
    )
}

fn execute_command(command: String) {
    let mut parts = command.split_whitespace();
    let executable = parts.next().expect("No se encontró el ejecutable");
    let args: Vec<&str> = parts.collect();

    let output = Command::new(executable)
        .args(&args)
        .output()
        .expect("Error al ejecutar el comando");

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("Creating the project:{}", stdout);
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("Error:\n{}", stderr);
    }
}
