use serde::{Deserialize, Serialize};
use std::{fs, io::{self, Write}, path::PathBuf};

#[derive(Serialize, Deserialize, Debug)]
pub struct BotConfig {
    pub api_key: String,
    pub ai_name: String,
    pub model: String,
    pub debug: bool,
}

impl BotConfig {
    pub fn load_or_create() -> Self {
        let config_path = get_config_path();

        if config_path.exists() {
            let content = fs::read_to_string(&config_path)
                .expect("Failed to read config file");
            toml::from_str(&content).expect("Invalid config format")
        } else {
            first_time_setup(&config_path)
        }
    }

    pub fn save(&self) {
        let config_path = get_config_path();
        let toml_data = toml::to_string_pretty(self).unwrap();
        fs::write(config_path, toml_data).expect("Failed to save config");
    }
}

fn first_time_setup(path: &PathBuf) -> BotConfig {
    println!("🚀 Welcome to BotR (first-time setup)");

    let api_key = prompt("Enter your Groq API key");
    let ai_name = prompt_default("AI name", "BotR");
    let model = prompt_default("Groq model", "llama-3.1-8b-instant");

    let config = BotConfig {
        api_key,
        ai_name,
        model,
        debug: false,
    };

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }

    config.save();
    println!("✅ Setup complete! Config saved.");

    config
}

fn get_config_path() -> PathBuf {
    let mut path = dirs::config_dir().expect("Cannot find config directory");
    path.push("botr");
    path.push("config.toml");
    path
}

fn prompt(label: &str) -> String {
    print!("{}: ", label);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn prompt_default(label: &str, default: &str) -> String {
    print!("{} [{}]: ", label, default);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let input = input.trim();
    if input.is_empty() {
        default.to_string()
    } else {
        input.to_string()
    }
}
