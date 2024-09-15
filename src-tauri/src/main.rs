// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;
use dotenv::dotenv;
use reqwest::{Client, header};
use crate::model::{JiraProject, JiraProjectInfo, JiraVersionCreateRequest, JiraVersionCreateResponse};

mod model;

// GET Jira projects command (existing one)
#[tauri::command]
async fn get_jira_projects() -> Result<Vec<JiraProjectInfo>, String> {
    dotenv().ok();

    let jira_url = env::var("JIRA_API_URL").map_err(|_| "Error reading Jira API URL".to_string())?;
    let jira_key = env::var("JIRA_API_KEY").map_err(|_| "Error reading Jira API Key".to_string())?;
    let auth_header = format!("Bearer {}", jira_key);
    let project_endpoint: String = format!("{}/project", jira_url);

    let client = Client::new();

    let response = client
        .get(&project_endpoint)
        .header(header::AUTHORIZATION, auth_header)
        .send()
        .await;

    match response {
        Ok(resp) => {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_else(|_| "Unable to read response body".to_string());

            if !status.is_success() {
                println!("Error: Status code = {}, Response = {}", status, text);
                return Err(format!("Error: Received status code {} with body: {}", status, text));
            }

            match serde_json::from_str::<Vec<JiraProject>>(&text) {
                Ok(parsed_response) => {
                    let project_infos: Vec<JiraProjectInfo> = parsed_response
                        .into_iter()
                        .map(|project| project.to_info())
                        .collect();

                    Ok(project_infos)
                },
                Err(e) => {
                    println!("Failed to parse response: {}, Response body: {}", e, text);
                    Err(format!("Failed to parse response as Vec<JiraProject>: {}", e))
                }
            }
        },
        Err(e) => {
            println!("Failed to send request: {}", e);
            Err(format!("Failed to send request: {}", e))
        }
    }
}

// POST handler for creating a Jira version
#[tauri::command]
async fn create_jira_version(version_data: JiraVersionCreateRequest) -> Result<JiraVersionCreateResponse, String> {
    dotenv().ok();

    let jira_url = env::var("JIRA_API_URL").map_err(|_| "Error reading Jira API URL".to_string())?;
    let jira_key = env::var("JIRA_API_KEY").map_err(|_| "Error reading Jira API Key".to_string())?;
    let auth_header = format!("Bearer {}", jira_key);
    let version_endpoint: String = format!("{}/version", jira_url);

    let client = Client::new();

    let response = client
        .post(&version_endpoint)
        .header(header::AUTHORIZATION, auth_header)
        .json(&version_data) // Send the version data in JSON format
        .send()
        .await;

    match response {
        Ok(resp) => {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_else(|_| "Unable to read response body".to_string());

            if !status.is_success() {
                println!("Error: Status code = {}, Response = {}", status, text);
                return Err(format!("Error: Received status code {} with body: {}", status, text));
            }

            // Deserialize the response into the JiraVersionCreateResponse struct
            match serde_json::from_str::<JiraVersionCreateResponse>(&text) {
                Ok(created_version) => Ok(created_version),
                Err(e) => {
                    println!("Failed to parse response: {}, Response body: {}", e, text);
                    Err(format!("Failed to parse response as JiraVersionCreateResponse: {}", e))
                }
            }
        },
        Err(e) => {
            println!("Failed to send request: {}", e);
            Err(format!("Failed to send request: {}", e))
        }
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_jira_projects, create_jira_version]) // Add the new POST handler here
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
