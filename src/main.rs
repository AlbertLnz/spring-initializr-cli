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

        match get_spring_boot_version() {
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

        println!(
            "You selected: {} - {} - {}",
            language, project, spring_boot_version
        );
    }
}

fn get_spring_boot_version() -> Result<(Vec<String>, Option<usize>), Box<dyn Error>> {
    let url = "https://start.spring.io/metadata/client";
    let response = reqwest::blocking::get(url)?;
    let response_json: Value = response.json()?;

    let boot_version: BootVersion = serde_json::from_value(response_json["bootVersion"].clone())?;

    let ids: Vec<String> = boot_version.values.iter().map(|v| v.id.clone()).collect();
    let names: Vec<String> = boot_version.values.iter().map(|v| v.name.clone()).collect();

    let default_index = ids
        .iter()
        .position(|id| id.trim() == boot_version.default.trim());

    Ok((names, default_index))
}
