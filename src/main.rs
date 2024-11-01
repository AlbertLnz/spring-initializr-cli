use std::io;

use colored::*;
use dialoguer::{theme::ColorfulTheme, Select};
use reqwest;
use serde::Deserialize;
use serde_json::Value;
use std::error::Error;

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
struct VersionValue {
    id: String,
    name: String,
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
        let options = vec!["Java", "Kotlin", "Groovy"];

        let selection = Select::with_theme(&ColorfulTheme::default())
            .items(&options)
            .default(0)
            .interact()
            .expect("Failed to read selection");

        let language = options[selection];
        println!(" {}\n", language);

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
            Ok((names, default_index)) => {
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

        // PACKAGING
        println!("{}", "Select the packaging:".bright_green());
        let options = vec!["Jar", "War"];

        let selection = Select::with_theme(&ColorfulTheme::default())
            .items(&options)
            .default(0)
            .interact()
            .expect("Failed to read selection");

        let packaging = options[selection];
        println!(" {}\n", packaging);

        println!(
            "You selected: {} - {} - {} - {} - {} - {} - {}",
            language,
            project,
            spring_boot_version,
            project_group,
            project_name,
            project_description,
            packaging
        );
    }
}

fn action_() -> Result<(Vec<String>, Option<usize>), Box<dyn Error>> {
    let json = fetch_spring_initializr_api()?;
    let spring_boot_version = get_spring_boot_version(json)?;

    Ok(spring_boot_version)
}

fn fetch_spring_initializr_api() -> Result<Value, Box<dyn Error>> {
    let url = "https://start.spring.io/metadata/client";
    let response = reqwest::blocking::get(url)?;
    let response_json: Value = response.json()?;
    Ok(response_json)
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
