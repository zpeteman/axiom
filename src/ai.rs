use serde_json::json;
use reqwest::Client;
use std::env;
use anyhow::Result;
use std::time::Duration;

pub async fn ask_ai(prompt: &str) -> Result<String> {
    dotenv::dotenv().ok();
   
    // read api key and model name from .env
    let api_key = env::var("OPENROUTER_API_KEY")
        .expect("OPENROUTER_API_KEY not set in .env");
    let model_name = env::var("OPENROUTER_NAME")
        .unwrap_or_else(|_| "gryphe/mythomax-l2-13b".to_string());
    
    let client = Client::builder()
        .timeout(Duration::from_secs(30))  // Increased timeout
        .build()?;
    
    let body = json!({
        "model": model_name,
        "messages": [{"role": "user", "content": prompt}],
    });
    
    let res = client
        .post("https://openrouter.ai/api/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&body)
        .send()
        .await?;
    
    // Check if response is successful
    if !res.status().is_success() {
        eprintln!("API Error: Status {}", res.status());
        let error_text = res.text().await?;
        eprintln!("Error body: {}", error_text);
        return Err(anyhow::anyhow!("API request failed"));
    }
    
    let json: serde_json::Value = res.json().await?;
    let reply = json["choices"][0]["message"]["content"]
        .as_str()
        .unwrap_or("...")
        .to_string();
    
    Ok(reply)
}