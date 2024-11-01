use colored::*;
use dialoguer::{theme::ColorfulTheme, Select};

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

        let selection = Select::with_theme(&ColorfulTheme::default())
            .items(&options)
            .default(0)
            .interact()
            .expect("Failed to read selection");

        let project = options[selection];
        println!(" {}\n", project);

        println!("You selected: {} - {}", language, project);
    }
}
