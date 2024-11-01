use std::io;

use colored::*;
use dialoguer::{theme::ColorfulTheme, MultiSelect, Select};
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
struct LanguageVersion {
    #[serde(rename = "type")]
    version_type: String,
    default: String,
    values: Vec<VersionValue>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct BootVersion {
    #[serde(rename = "type")]
    version_type: String,
    default: String,
    values: Vec<VersionValue>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct Packaging {
    #[serde(rename = "type")]
    version_type: String,
    default: String,
    values: Vec<VersionValue>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct JavaVersion {
    #[serde(rename = "type")]
    version_type: String,
    default: String,
    values: Vec<VersionValue>,
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
struct VersionValue {
    id: String,
    name: String,
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

fn main() {
    println!("{}", "Spring Initializr CLI!".bright_green().bold());
    println!(
        "{}",
        "https://github.com/AlbertLnz/spring-initializr-cli".bright_yellow()
    );
    println!("{}", "Created by AlbertLnz".bright_cyan());

    loop {
        // LANGUAGE:
        println!("{}", "Select the language:".bright_green());
        let mut spring_language = String::new();

        match action_() {
            Ok(data) => {
                let (names, default_index) = data.spring_languages;
                let default_position = default_index.unwrap_or(0);

                let selection = Select::with_theme(&ColorfulTheme::default())
                    .items(&names)
                    .default(default_position)
                    .interact()
                    .expect("Failed to read selection");

                spring_language = names[selection].clone();
                println!("Selected Java Version: {}\n", spring_language);
            }
            Err(e) => eprintln!("Error: {}", e),
        }

        // PROJECT
        println!("{}", "Select the project:".bright_green());
        let options = vec!["Gradle", "Maven"];

        let selection = Select::with_theme(&ColorfulTheme::default())
            .items(&options)
            .default(1)
            .interact()
            .expect("Failed to read selection");

        let project = options[selection];
        println!(" {}\n", project);

        // SPRING BOOTVERSION
        println!("{}", "Select the Spring Boot version:".bright_green());
        let mut spring_boot_version = String::new();

        match action_() {
            Ok(data) => {
                let (names, default_index) = data.spring_boot_versions;
                let default_position = default_index.unwrap_or(0);

                let selection = Select::with_theme(&ColorfulTheme::default())
                    .items(&names)
                    .default(default_position)
                    .interact()
                    .expect("Failed to read selection");

                spring_boot_version = names[selection].clone();
                println!("Selected Spring Boot Version: {}\n", spring_boot_version);
            }
            Err(e) => eprintln!("Error: {}", e),
        }

        // PROJECT METADATA
        println!("{}", "Enter the group:".bright_yellow());
        let mut project_group = String::new();
        io::stdin()
            .read_line(&mut project_group)
            .expect("Failed to read input!");
        let project_group: &str = project_group.trim();

        println!("{}", "Enter the name:".bright_yellow());
        let mut project_name = String::new();
        io::stdin()
            .read_line(&mut project_name)
            .expect("Failed to read input!");
        let project_name: &str = project_name.trim();

        println!("{}", "Enter the description:".bright_yellow());
        let mut project_description = String::new();
        io::stdin()
            .read_line(&mut project_description)
            .expect("Failed to read input!");
        let project_description: &str = project_description.trim();

        println!("{}", "Enter the version:".bright_yellow());
        let mut project_version = String::new();
        io::stdin()
            .read_line(&mut project_version)
            .expect("Failed to read input!");
        let project_version: &str = project_version.trim();

        // PACKAGING
        println!("{}", "Select the packaging:".bright_green());
        let mut spring_packaging = String::new();

        match action_() {
            Ok(data) => {
                let (names, default_index) = data.spring_packagings;
                let default_position = default_index.unwrap_or(0);

                let selection = Select::with_theme(&ColorfulTheme::default())
                    .items(&names)
                    .default(default_position)
                    .interact()
                    .expect("Failed to read selection");

                spring_packaging = names[selection].clone();
                println!("Selected Java Version: {}\n", spring_packaging);
            }
            Err(e) => eprintln!("Error: {}", e),
        }

        // JAVA VERSION
        println!("{}", "Select the Java version:".bright_green());
        let mut java_version = String::new();

        match action_() {
            Ok(data) => {
                let (names, default_index) = data.java_versions;
                let default_position = default_index.unwrap_or(0);

                let selection = Select::with_theme(&ColorfulTheme::default())
                    .items(&names)
                    .default(default_position)
                    .interact()
                    .expect("Failed to read selection");

                java_version = names[selection].clone();
                println!("Selected Java Version: {}\n", java_version);
            }
            Err(e) => eprintln!("Error: {}", e),
        }

        // DEPENDENCIES
        println!("{}", "Select the dependencies:".bright_green());
        let mut selected_java_dependencies = vec![];
        match action_() {
            Ok(data) => {
                let java_dependency_names = data.java_dependencies;

                let java_dependency_selection = MultiSelect::with_theme(&ColorfulTheme::default())
                    .items(&java_dependency_names)
                    .interact()
                    .expect("Failed to read selection");

                for &index in &java_dependency_selection {
                    selected_java_dependencies.push(java_dependency_names[index].clone());
                }

                println!(
                    "Selected Java Dependencies: {:?}\n",
                    selected_java_dependencies
                );
            }
            Err(e) => eprintln!("Error: {}", e),
        }

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
            &selected_java_dependencies,
        );

        println!("{}", command);
    }
}

fn action_() -> Result<SpringInitializrData, Box<dyn Error>> {
    let json = fetch_spring_initializr_api()?;
    let spring_language = get_spring_langauge(json.clone())?;
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

fn get_spring_langauge(
    response_json: Value,
) -> Result<(Vec<String>, Option<usize>), Box<dyn Error>> {
    let boot_version: LanguageVersion = serde_json::from_value(response_json["language"].clone())?;

    let ids: Vec<String> = boot_version.values.iter().map(|v| v.id.clone()).collect();
    let names: Vec<String> = boot_version.values.iter().map(|v| v.name.clone()).collect();

    let default_index = ids
        .iter()
        .position(|id| id.trim() == boot_version.default.trim());

    Ok((names, default_index))
}

fn get_spring_boot_version(
    response_json: Value,
) -> Result<(Vec<String>, Option<usize>), Box<dyn Error>> {
    let boot_version: BootVersion = serde_json::from_value(response_json["bootVersion"].clone())?;

    let ids: Vec<String> = boot_version.values.iter().map(|v| v.id.clone()).collect();
    let names: Vec<String> = boot_version.values.iter().map(|v| v.name.clone()).collect();

    let default_index = ids
        .iter()
        .position(|id| id.trim() == boot_version.default.trim());

    Ok((names, default_index))
}

fn get_spring_packaging(
    response_json: Value,
) -> Result<(Vec<String>, Option<usize>), Box<dyn Error>> {
    let boot_version: Packaging = serde_json::from_value(response_json["packaging"].clone())?;

    let ids: Vec<String> = boot_version.values.iter().map(|v| v.id.clone()).collect();
    let names: Vec<String> = boot_version.values.iter().map(|v| v.name.clone()).collect();

    let default_index = ids
        .iter()
        .position(|id| id.trim() == boot_version.default.trim());

    Ok((names, default_index))
}

fn get_java_version(response_json: Value) -> Result<(Vec<String>, Option<usize>), Box<dyn Error>> {
    let boot_version: JavaVersion = serde_json::from_value(response_json["javaVersion"].clone())?;

    let ids: Vec<String> = boot_version.values.iter().map(|v| v.id.clone()).collect();
    let names: Vec<String> = boot_version.values.iter().map(|v| v.name.clone()).collect();

    let default_index = ids
        .iter()
        .position(|id| id.trim() == boot_version.default.trim());

    Ok((names, default_index))
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
        "spring init \\\n  --name={} \\\n  --groupId={} \\\n  --artifactId={} \\\n  --version={} \\\n  --description=\"{}\" \\\n  --package-name={} \\\n  --dependencies={} \\\n  --build={} \\\n  --type={}-project \\\n  --java-version={} \\\n  --language={} \\\n  --boot-version={} \\\n  --packaging={} \\\n  {}",
        project_name,
        project_group,
        project_name,
        project_version,
        project_description,
        project_group.replace('.', ".").to_lowercase() + "." + project_name,
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
