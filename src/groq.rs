use reqwest::Client;
use serde_json::{json, Value};

pub async fn ask_groq(prompt: &str, api_key: &str) -> Result<String, String> {
    let client = Client::new();

    let res = client
        .post("https://api.groq.com/openai/v1/chat/completions")
        .bearer_auth(api_key)
        .json(&json!({
            "model": "llama-3.1-8b-instant",
            "messages": [
                { "role": "user", "content": prompt }
            ]
        }))
        .send()
        .await
        .map_err(|e| format!("Request error: {}", e))?;

    let text = res.text().await.map_err(|e| e.to_string())?;

    // 🔍 TEMP DEBUG PRINT
    println!("\n[GROQ RAW RESPONSE]\n{}\n", text);

    let json: Value = serde_json::from_str(&text)
        .map_err(|e| format!("JSON parse error: {}", e))?;

    // ✅ Handle Groq error properly
    if let Some(err) = json.get("error") {
        return Err(err["message"]
            .as_str()
            .unwrap_or("Unknown Groq error")
            .to_string());
    }

    Ok(json["choices"][0]["message"]["content"]
        .as_str()
        .unwrap_or("Empty response")
        .to_string())
}

