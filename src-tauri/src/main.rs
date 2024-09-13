// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::env;
use dotenv::dotenv;
use reqwest::{Client, header};
use crate::model::{JiraProject, JiraProjectInfo};

mod model;

#[tauri::command]
async fn get_jira_projects() -> Result<Vec<JiraProjectInfo>, String> {
    dotenv().ok();

    let jira_url = env::var("JIRA_API_URL").map_err(|_| "Error reading Jira API URL".to_string())?;
    let jira_key = env::var("JIRA_API_KEY").map_err(|_| "Error reading Jira API Key".to_string())?;
    let auth_header = format!("Bearer {}", jira_key);

    let client = Client::new();

    // Make the request and handle possible errors
    let response = client
        .get(&jira_url)
        .header(header::AUTHORIZATION, auth_header)
        .send()
        .await;

    match response {
        Ok(resp) => {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_else(|_| "Unable to read response body".to_string());

            if !status.is_success() {
                // Log the error for non-success status codes
                println!("Error: Status code = {}, Response = {}", status, text);
                return Err(format!("Error: Received status code {} with body: {}", status, text));
            }

            // Deserialize directly into a Vec<JiraProject> since it's an array
            match serde_json::from_str::<Vec<JiraProject>>(&text) {
                Ok(parsed_response) => {
                    // Convert the raw projects into the flattened structure
                    let project_infos: Vec<JiraProjectInfo> = parsed_response
                        .into_iter()
                        .map(|project| project.to_info())
                        .collect();

                    Ok(project_infos)
                },
                Err(e) => {
                    // If deserialization fails, log the error and the response.
                    println!("Failed to parse response: {}, Response body: {}", e, text);
                    Err(format!("Failed to parse response as Vec<JiraProject>: {}", e))
                }
            }
        },
        Err(e) => {
            // Handle any request errors
            println!("Failed to send request: {}", e);
            Err(format!("Failed to send request: {}", e))
        }
    }
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![get_jira_projects])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
