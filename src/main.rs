mod config;

use config::BotConfig;
use reqwest::Client;
use serde_json::json;
use std::io::{self, Write};
use futures_util::StreamExt;

#[tokio::main]
async fn main() {
    let mut config = BotConfig::load_or_create();
    let client = Client::new();

    println!("\n══════════════════════════════════════");
    println!("🤖 {} is online", config.ai_name);
    println!("Type 'help' for commands");
    println!("══════════════════════════════════════\n");

    loop {
        print!("🧑 You > ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        if input == "exit" || input == "quit" {
            println!("👋 Goodbye!");
            break;
        }

        if input == "help" {
            show_help();
            continue;
        }

        if input == "clear" {
            print!("\x1B[2J\x1B[1;1H");
            continue;
        }

        if input.starts_with("set name ") {
            let name = input.replace("set name ", "");
            config.ai_name = name.clone();
            config.save();
            println!("✅ AI renamed to {}", name);
            continue;
        }

        if input == "debug on" {
            config.debug = true;
            config.save();
            println!("🐞 Debug ON");
            continue;
        }

        if input == "debug off" {
            config.debug = false;
            config.save();
            println!("🐞 Debug OFF");
            continue;
        }

        print!("🤖 {}: ", config.ai_name);
        io::stdout().flush().unwrap();

        if let Err(e) = stream_groq_response(&client, &config, input).await {
            println!("\n❌ Error: {}", e);
        }

        println!();
    }
}

async fn stream_groq_response(
    client: &Client,
    config: &BotConfig,
    prompt: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let res = client
        .post("https://api.groq.com/openai/v1/chat/completions")
        .bearer_auth(&config.api_key)
        .json(&json!({
            "model": "llama-3.1-8b-instant",
            "stream": true,
            "messages": [
                { "role": "system", "content": "You are a helpful AI assistant." },
                { "role": "user", "content": prompt }
            ]
        }))
        .send()
        .await?;

    let mut stream = res.bytes_stream();

    while let Some(chunk) = stream.next().await {
        let chunk = chunk?; // ✅ FIX
        let text = String::from_utf8_lossy(&chunk);

        for line in text.lines() {
            if line.starts_with("data: ") {
                let data = &line[6..];
                if data == "[DONE]" {
                    return Ok(());
                }

                if let Ok(json) = serde_json::from_str::<serde_json::Value>(data) {
                    if let Some(token) =
                        json["choices"][0]["delta"]["content"].as_str()
                    {
                        print!("{}", token);
                        io::stdout().flush().unwrap();
                    }
                }
            }
        }
    }

    Ok(())
}

fn show_help() {
    println!();
    println!("Commands:");
    println!(" help        → Show this menu");
    println!(" exit / quit → Exit BotR");
    println!(" clear       → Clear screen");
    println!(" set name X  → Rename AI");
    println!(" debug on    → Enable debug");
    println!(" debug off   → Disable debug");
    println!();
}
