# Spring Initializr CLI

**Spring Initializr CLI** is a command-line tool built in [Rust](https://www.rust-lang.org/) that allows developers to quickly and easily create new Spring Boot projects directly from the terminal. This tool is connected to the [Spring Initializr API](https://start.spring.io/) and enables you to generate a Spring Boot project with all the initial configurations you need without having to interact with the Spring Initializr web interface.


## Features

- Fast generation of Spring Boot projects.
- Customization of project settings directly from the terminal.
- Color output to enhance user experience.
- Interactive dialogs to facilitate option selection.

## Install on your PC!

## Locally project installation

To install **Spring Initializr CLI**, make sure you have Rust installed on your system. Then, you can clone this repository and build the project:

```bash
git clone https://github.com/albertlnz/spring-initializr-cli.git
cd spring-initializr-cli
cargo build --release
