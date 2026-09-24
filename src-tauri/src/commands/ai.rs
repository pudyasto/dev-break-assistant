use tauri::State;
use reqwest::Client;
use serde_json::json;
use serde::Deserialize;

use crate::errors::AppError;
use crate::state::AppState;
use crate::infrastructure::database::repositories;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveChatMessageInput { pub conversation_id: i64, pub role: String, pub content: String }

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArchiveChatInput { pub conversation_id: i64, pub archived: bool }

#[derive(Debug, Deserialize)]
pub struct CreateChatInput { pub title: String }

fn activity_context(stats: &[crate::domain::statistics::TodayStatistics]) -> String {
    if stats.is_empty() {
        return "No activity has been recorded in the last 7 days.".to_string();
    }

    let days = stats.len();
    let active_seconds: i64 = stats.iter().map(|s| s.active_seconds).sum();
    let idle_seconds: i64 = stats.iter().map(|s| s.idle_seconds).sum();
    let break_seconds: i64 = stats.iter().map(|s| s.break_seconds).sum();
    let breaks: i64 = stats.iter().map(|s| s.break_count).sum();
    let skipped: i64 = stats.iter().map(|s| s.skipped_break_count).sum();
    let longest_streak = stats.iter().map(|s| s.longest_active_streak_seconds).max().unwrap_or(0);

    format!(
        "Activity summary for the last {} recorded day(s): {:.1} active hours, {:.1} idle hours, {:.1} break hours, {} completed breaks, {} skipped reminders, and a longest focus streak of {:.0} minutes. This summary contains no app names, window titles, or keystrokes.",
        days,
        active_seconds as f64 / 3600.0,
        idle_seconds as f64 / 3600.0,
        break_seconds as f64 / 3600.0,
        breaks,
        skipped,
        longest_streak as f64 / 60.0,
    )
}

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
pub async fn generate_daily_tip(
    state: State<'_, AppState>,
    language: Option<String>,
) -> Result<Option<String>, AppError> {
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

    let language_instruction = if language.as_deref() == Some("id") {
        "Respond only in Indonesian (Bahasa Indonesia)."
    } else {
        "Respond only in English."
    };

    let prompt = if let Some(s) = stats {
        let active_hours = s.active_seconds as f64 / 3600.0;
        let score = s.desk_habit_score.unwrap_or(100);
        format!("You are a desktop wellness assistant. Yesterday the user worked for {:.1} hours, took {} breaks, skipped {} breaks, and had a wellness score of {}/100. Write a single, brief, encouraging 1-sentence wellness tip for them today based on these stats. Do not greet or explain, just the tip. {}", active_hours, s.break_count, s.skipped_break_count, score, language_instruction)
    } else {
        format!("You are a desktop wellness assistant. Write a single, brief, encouraging 1-sentence wellness tip for a software developer today. Do not greet or explain, just the tip. {}", language_instruction)
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

#[tauri::command]
pub async fn chat_with_ai(
    state: State<'_, AppState>,
    messages: Vec<ChatMessage>,
    include_activity: bool,
    language: Option<String>,
) -> Result<String, AppError> {
    let ai_enabled = repositories::get_setting(&state.db, "ai.enabled").await?
        .unwrap_or_else(|| "false".to_string());
    if ai_enabled != "true" {
        return Err(AppError::Internal("AI belum diaktifkan. Aktifkan integrasi AI di Pengaturan terlebih dahulu.".to_string()));
    }

    let raw_endpoint = repositories::get_setting(&state.db, "ai.endpoint").await?
        .unwrap_or_else(|| "http://localhost:11434/api/generate".to_string());
    let endpoint = format_ollama_endpoint(&raw_endpoint);
    let model = repositories::get_setting(&state.db, "ai.model").await?
        .unwrap_or_else(|| "llama3".to_string());
    let language_instruction = if language.as_deref() == Some("id") {
        "Respond in Indonesian (Bahasa Indonesia)."
    } else {
        "Respond in English."
    };

    let recent_messages = messages.iter().rev().take(12).collect::<Vec<_>>().into_iter().rev();
    let conversation = recent_messages
        .filter(|m| (m.role == "user" || m.role == "assistant") && !m.content.trim().is_empty())
        .map(|m| format!("{}: {}", if m.role == "assistant" { "Assistant" } else { "User" }, m.content.trim()))
        .collect::<Vec<_>>()
        .join("\n\n");

    if conversation.is_empty() {
        return Err(AppError::Internal("Pesan tidak boleh kosong.".to_string()));
    }

    let context = if include_activity {
        let end = chrono::Local::now().format("%Y-%m-%d").to_string();
        let start = (chrono::Local::now() - chrono::Duration::days(6)).format("%Y-%m-%d").to_string();
        activity_context(&repositories::get_statistics_range(&state.db, &start, &end).await?)
    } else {
        "The user chose not to share their activity data for this message.".to_string()
    };

    let prompt = format!(
        "You are DevBreak AI, a supportive desktop wellness assistant for developers. Answer questions, help the user reflect on work and break habits, and provide practical, non-diagnostic wellness guidance. Do not claim medical expertise; advise seeking a qualified professional for pain, injury, or urgent health concerns. Be concise, warm, and avoid inventing data. {}\n\nPrivacy context: {}\n\nConversation:\n{}\n\nAssistant:",
        language_instruction, context, conversation
    );

    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(60))
        .build()
        .map_err(|e| AppError::Internal(format!("Failed to build client: {}", e)))?;
    let res = client.post(&endpoint)
        .json(&json!({ "model": model, "prompt": prompt, "stream": false }))
        .send()
        .await
        .map_err(|e| AppError::Internal(format!("AI Request failed: {}", e)))?;

    if !res.status().is_success() {
        let status = res.status();
        let error_text = res.text().await.unwrap_or_default();
        return Err(AppError::Internal(format!("AI server error (HTTP {}): {}", status, error_text)));
    }
    let body: serde_json::Value = res.json().await.map_err(|e| AppError::Internal(e.to_string()))?;
    body.get("response")
        .and_then(|response| response.as_str())
        .map(|response| response.trim().to_string())
        .filter(|response| !response.is_empty())
        .ok_or_else(|| AppError::Internal("AI tidak mengembalikan respons.".to_string()))
}

#[tauri::command]
pub async fn list_chat_conversations(state: State<'_, AppState>, archived: bool) -> Result<Vec<repositories::ChatConversation>, AppError> {
    repositories::list_chat_conversations(&state.db, archived).await
}

#[tauri::command]
pub async fn create_chat_conversation(state: State<'_, AppState>, input: CreateChatInput) -> Result<repositories::ChatConversation, AppError> {
    let title = input.title.trim();
    if title.is_empty() { return Err(AppError::Internal("Chat title cannot be empty.".to_string())); }
    repositories::create_chat_conversation(&state.db, title).await
}

#[tauri::command]
pub async fn get_chat_messages(state: State<'_, AppState>, conversation_id: i64) -> Result<Vec<repositories::StoredChatMessage>, AppError> {
    repositories::get_chat_messages(&state.db, conversation_id).await
}

#[tauri::command]
pub async fn save_chat_message(state: State<'_, AppState>, input: SaveChatMessageInput) -> Result<repositories::StoredChatMessage, AppError> {
    if !matches!(input.role.as_str(), "user" | "assistant") || input.content.trim().is_empty() { return Err(AppError::Internal("Invalid chat message.".to_string())); }
    repositories::save_chat_message(&state.db, input.conversation_id, &input.role, input.content.trim()).await
}

#[tauri::command]
pub async fn archive_chat_conversation(state: State<'_, AppState>, input: ArchiveChatInput) -> Result<(), AppError> {
    repositories::set_chat_archived(&state.db, input.conversation_id, input.archived).await
}

#[tauri::command]
pub async fn delete_chat_conversation(state: State<'_, AppState>, conversation_id: i64) -> Result<(), AppError> {
    repositories::delete_chat_conversation(&state.db, conversation_id).await
}

#[tauri::command]
pub async fn delete_all_chat_conversations(state: State<'_, AppState>) -> Result<(), AppError> {
    repositories::delete_all_chat_conversations(&state.db).await
}
