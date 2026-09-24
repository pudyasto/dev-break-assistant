use tauri::State;
use reqwest::Client;
use serde_json::json;

use crate::errors::AppError;
use crate::state::AppState;
use crate::infrastructure::database::repositories;

pub fn format_ollama_endpoint(raw: &str) -> String {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return "http://localhost:11434/api/generate".to_string();
    }
    let without_trailing = trimmed.trim_end_matches('/');
    if without_trailing.ends_with("/api/generate") {
        without_trailing.to_string()
    } else {
        format!("{}/api/generate", without_trailing)
    }
}

#[tauri::command]
pub async fn test_ai_connection(
    endpoint: String,
    model: String,
) -> Result<String, AppError> {
    let target_endpoint = format_ollama_endpoint(&endpoint);
    let target_model = if model.trim().is_empty() {
        "llama3".to_string()
    } else {
        model.trim().to_string()
    };

    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(15))
        .build()
        .map_err(|e| AppError::Internal(format!("Failed to build client: {}", e)))?;

    let res = client.post(&target_endpoint)
        .json(&json!({
            "model": target_model,
            "prompt": "Respond with 'OK' and nothing else.",
            "stream": false
        }))
        .send()
        .await
        .map_err(|e| AppError::Internal(format!("Koneksi gagal ke {}: {}", target_endpoint, e)))?;

    if res.status().is_success() {
        let json_body: serde_json::Value = res.json().await.map_err(|e| AppError::Internal(e.to_string()))?;
        if let Some(resp) = json_body.get("response").and_then(|r| r.as_str()) {
            return Ok(resp.trim().to_string());
        }
        return Ok("Koneksi berhasil terhubung ke Ollama!".to_string());
    }

    let status = res.status();
    let error_text = res.text().await.unwrap_or_default();
    Err(AppError::Internal(format!("Ollama server error (HTTP {}): {}", status, error_text)))
}

#[tauri::command]
pub async fn generate_daily_tip(state: State<'_, AppState>) -> Result<Option<String>, AppError> {
    let track_ai = repositories::get_setting(&state.db, "ai.enabled").await?.unwrap_or_else(|| "false".to_string());
    if track_ai != "true" {
        return Ok(None);
    }
    
    let raw_endpoint = repositories::get_setting(&state.db, "ai.endpoint").await?.unwrap_or_else(|| "http://localhost:11434/api/generate".to_string());
    let endpoint = format_ollama_endpoint(&raw_endpoint);
    let model = repositories::get_setting(&state.db, "ai.model").await?.unwrap_or_else(|| "llama3".to_string());
    
    // Get yesterday's stats
    let yesterday = (chrono::Local::now() - chrono::Duration::days(1)).format("%Y-%m-%d").to_string();
    let stats_list = repositories::get_statistics_range(&state.db, &yesterday, &yesterday).await?;
    let stats = stats_list.first();

    let prompt = if let Some(s) = stats {
        let active_hours = s.active_seconds as f64 / 3600.0;
        let score = s.desk_habit_score.unwrap_or(100);
        format!("You are a desktop wellness assistant. Yesterday the user worked for {:.1} hours, took {} breaks, skipped {} breaks, and had a wellness score of {}/100. Write a single, brief, encouraging 1-sentence wellness tip for them today based on these stats. Do not greet or explain, just the tip.", active_hours, s.break_count, s.skipped_break_count, score)
    } else {
        "You are a desktop wellness assistant. Write a single, brief, encouraging 1-sentence wellness tip for a software developer today. Do not greet or explain, just the tip.".to_string()
    };

    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .map_err(|e| AppError::Internal(format!("Failed to build client: {}", e)))?;

    let res = client.post(&endpoint)
        .json(&json!({
            "model": model,
            "prompt": prompt,
            "stream": false
        }))
        .send()
        .await
        .map_err(|e| AppError::Internal(format!("AI Request failed: {}", e)))?;

    if res.status().is_success() {
        let json_body: serde_json::Value = res.json().await.map_err(|e| AppError::Internal(e.to_string()))?;
        if let Some(resp) = json_body.get("response").and_then(|r| r.as_str()) {
            return Ok(Some(resp.trim().to_string()));
        }
    }

    Ok(None)
}
