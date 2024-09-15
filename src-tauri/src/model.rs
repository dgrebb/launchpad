use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct JiraProject {
    pub id: String,
    pub key: String,
    pub name: String,
    #[serde(rename = "avatarUrls")]
    pub avatar: AvatarUrls,
}

#[derive(Serialize, Deserialize)]
pub struct AvatarUrls {
    #[serde(rename = "48x48")]
    pub avatar_url: String,
}

// Flattened structure
#[derive(Serialize, Deserialize, Debug)]
pub struct JiraProjectInfo {
    pub id: String,
    pub key: String,
    pub name: String,
    pub avatar_url: String,
}

impl JiraProject {
    pub fn to_info(&self) -> JiraProjectInfo {
        JiraProjectInfo {
            id: self.id.clone(),
            key: self.key.clone(),
            name: self.name.clone(),
            avatar_url: self.avatar.avatar_url.clone(),
        }
    }
}

// Wrapper for Jira projects response
#[derive(Serialize, Deserialize)]
pub struct JiraProjectsResponse {
    pub projects: Vec<JiraProject>,
}

// Request body for creating a new version
#[derive(Serialize, Deserialize, Debug)]
pub struct JiraVersionCreateRequest {
    pub archived: bool,
    pub description: String,
    pub name: String,
    #[serde(rename = "projectId")]
    pub project_id: u32,
    #[serde(rename = "releaseDate")]
    pub release_date: String,
    #[serde(rename = "startDate")]
    pub start_date: String,
    pub released: bool,
}

// Response data for created version
#[derive(Serialize, Deserialize, Debug)]
pub struct JiraVersionCreateResponse {
    #[serde(rename = "self")]
    pub self_url: String,
    pub id: String,
    pub archived: bool,
    pub description: String,
    pub name: String,
    #[serde(rename = "projectId")]
    pub project_id: u32,
    #[serde(rename = "releaseDate")]
    pub release_date: String,
    #[serde(rename = "startDate")]
    pub start_date: String,
    pub released: bool,
}
