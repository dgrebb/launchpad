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

// Define the flattened structure we want to convert to.
#[derive(Serialize, Deserialize, Debug)]
pub struct JiraProjectInfo {
    pub id: String,
    pub key: String,
    pub name: String,
    pub avatar_url: String,
}

impl JiraProject {
    // Conversion function to flatten the JiraProject into JiraProjectInfo
    pub fn to_info(&self) -> JiraProjectInfo {
        JiraProjectInfo {
            id: self.id.clone(),
            key: self.key.clone(),
            name: self.name.clone(),
            avatar_url: self.avatar.avatar_url.clone(),
        }
    }
}

// Assuming the response is a list of JiraProject
#[derive(Serialize, Deserialize)]
pub struct JiraProjectsResponse {
    pub projects: Vec<JiraProject>,
}
