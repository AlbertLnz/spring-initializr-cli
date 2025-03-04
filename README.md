<div align="center">

  # Spring Initializr CLI

  **Spring Initializr CLI** is a command-line tool built in [Rust](https://www.rust-lang.org/) that allows developers to quickly and easily create new Spring Boot projects directly from the terminal. This tool is connected to the [Spring Initializr API](https://start.spring.io/) and enables you to generate a Spring Boot project with all the initial configurations you need without having to interact with the Spring Initializr web interface.

</div>

> Dependencies <br>
>> reqwest,
>> dialoguer,
>> inquire,
>> colored,
>> serde,
>> termion

## 🏁 Features

- Fast generation of Spring Boot projects.
- Customization of project settings directly from the terminal.
- Color output to enhance user experience.
- Interactive dialogs to facilitate option selection.


## ▶️ Demo


## 💻 Install on your PC! (Linux)

  1.   
      > [!IMPORTANT]
      > Before using this CLI app, you need to install **Spring Boot CLI**. See the ***'Install Spring Boot CLI'*** section.
  
  2. Download the file from 'targets/linux/spring-initializr-cli': [spring-initializr-cli](https://raw.githubusercontent.com/AlbertLnz/spring-initializr-cli/master/targets/linux/spring-initializr-cli)

  3. 
        > [!TIP]
        > I recommend to create an alias for execute the programm. See the ***'Create an alias'***
  
  4. Ubicate the download file wherever you want (according to alias)

  5. Run!

  ### ‒ Install Spring Boot CLI
  You can install **Spring Boot CLI** easily using SDKMan
  
  ### ‒ Create an alias
  
  1. Open: ``` sudo nano ~/.bashrc ```
  2. Add this example line: ``` alias spring-cli='~/Documents/spring-initializr-cli' ```


## ⬇️ Locally project installation

To install **Spring Initializr CLI**, make sure you have Rust and Spring Boot CLI installed on your system. Then, you can clone this repository and build the project:

```bash
git clone https://github.com/albertlnz/spring-initializr-cli.git
cd spring-initializr-cli
cargo build --release
```

## ☕ Buy me a coffee

[!["Buy Me A Coffee"](https://www.buymeacoffee.com/assets/img/custom_images/orange_img.png)](https://www.buymeacoffee.com/albertlnz)
