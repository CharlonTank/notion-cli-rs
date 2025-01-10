use anyhow::{Result, Context};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fmt;

use crate::config::Config;

const NOTION_API_VERSION: &str = "2022-06-28";
const NOTION_API_URL: &str = "https://api.notion.com/v1";

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum TaskStatus {
    NotStarted,
    InProgress,
    Done,
}

impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TaskStatus::NotStarted => write!(f, "Not started"),
            TaskStatus::InProgress => write!(f, "In progress"),
            TaskStatus::Done => write!(f, "Done"),
        }
    }
}

impl From<&str> for TaskStatus {
    fn from(status: &str) -> Self {
        match status {
            "In progress" => TaskStatus::InProgress,
            "Done" => TaskStatus::Done,
            _ => TaskStatus::NotStarted,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub status: TaskStatus,
    pub url: Option<String>,
}

impl Task {
    pub fn status_symbol(&self) -> &str {
        match self.status {
            TaskStatus::NotStarted => " ",
            TaskStatus::InProgress => "⏳",
            TaskStatus::Done => "✓",
        }
    }
}

#[derive(Debug)]
pub struct NotionClient {
    client: reqwest::Client,
    config: Config,
}

impl NotionClient {
    pub fn new(config: Config) -> Result<Self> {
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", config.notion_token))
                .context("Failed to create authorization header")?,
        );
        headers.insert(
            "Notion-Version",
            HeaderValue::from_str(NOTION_API_VERSION)
                .context("Failed to create Notion version header")?,
        );
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .context("Failed to create HTTP client")?;

        Ok(NotionClient { client, config })
    }

    pub async fn add_task(&self, title: &str) -> Result<Task> {
        let request_body = json!({
            "parent": { "database_id": self.config.database_id },
            "properties": {
                "Name": {
                    "title": [
                        {
                            "type": "text",
                            "text": {
                                "content": title,
                                "link": null
                            },
                            "annotations": {
                                "bold": false,
                                "italic": false,
                                "strikethrough": false,
                                "underline": false,
                                "code": false,
                                "color": "default"
                            }
                        }
                    ]
                },
                "Status": {
                    "status": {
                        "name": "Not started"
                    }
                }
            }
        });

        let response = self
            .client
            .post(&format!("{}/pages", NOTION_API_URL))
            .json(&request_body)
            .send()
            .await
            .context("Failed to send create task request")?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!(
                "Failed to create task. Status: {}, Error: {}",
                response.status(),
                response.text().await?
            ));
        }

        let response_value = response.json::<serde_json::Value>().await
            .context("Failed to parse create task response")?;

        Ok(Task {
            id: response_value["id"].as_str()
                .context("Missing id in response")?
                .to_string(),
            title: title.to_string(),
            status: TaskStatus::NotStarted,
            url: response_value["url"].as_str().map(String::from),
        })
    }

    pub async fn list_tasks(&self) -> Result<Vec<Task>> {
        let response = self
            .client
            .post(&format!(
                "{}/databases/{}/query",
                NOTION_API_URL, self.config.database_id
            ))
            .json(&json!({}))
            .send()
            .await
            .context("Failed to send list tasks request")?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!(
                "Failed to list tasks. Status: {}, Error: {}",
                response.status(),
                response.text().await?
            ));
        }

        let response_value = response.json::<serde_json::Value>().await
            .context("Failed to parse list tasks response")?;

        let results = response_value["results"]
            .as_array()
            .context("Missing results in response")?
            .iter()
            .filter_map(|page| {
                let id = page["id"].as_str()?;
                let title = page["properties"]["Name"]["title"][0]["text"]["content"].as_str()?;
                let status = page["properties"]["Status"]["status"]["name"].as_str()
                    .unwrap_or("Not started");
                let url = page["url"].as_str().map(String::from);

                Some(Task {
                    id: id.to_string(),
                    title: title.to_string(),
                    status: TaskStatus::from(status),
                    url,
                })
            })
            .collect();

        Ok(results)
    }

    pub async fn update_task_status(&self, task_id: &str, status: TaskStatus) -> Result<()> {
        let response = self
            .client
            .patch(&format!("{}/pages/{}", NOTION_API_URL, task_id))
            .json(&json!({
                "properties": {
                    "Status": {
                        "status": {
                            "name": status.to_string()
                        }
                    }
                }
            }))
            .send()
            .await
            .context("Failed to send update task request")?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!(
                "Failed to update task. Status: {}, Error: {}",
                response.status(),
                response.text().await?
            ));
        }

        Ok(())
    }

    pub async fn delete_task(&self, task_id: &str) -> Result<()> {
        let response = self
            .client
            .patch(&format!("{}/pages/{}", NOTION_API_URL, task_id))
            .json(&json!({
                "archived": true
            }))
            .send()
            .await
            .context("Failed to send delete task request")?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!(
                "Failed to delete task. Status: {}, Error: {}",
                response.status(),
                response.text().await?
            ));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_status_display() {
        assert_eq!(TaskStatus::NotStarted.to_string(), "Not started");
        assert_eq!(TaskStatus::InProgress.to_string(), "In progress");
        assert_eq!(TaskStatus::Done.to_string(), "Done");
    }

    #[test]
    fn test_task_status_from_str() {
        assert_eq!(TaskStatus::from("Not started"), TaskStatus::NotStarted);
        assert_eq!(TaskStatus::from("In progress"), TaskStatus::InProgress);
        assert_eq!(TaskStatus::from("Done"), TaskStatus::Done);
        assert_eq!(TaskStatus::from("Unknown"), TaskStatus::NotStarted);
    }

    #[test]
    fn test_task_status_symbol() {
        let task = Task {
            id: "123".to_string(),
            title: "Test".to_string(),
            status: TaskStatus::NotStarted,
            url: None,
        };
        assert_eq!(task.status_symbol(), " ");

        let task = Task {
            id: "123".to_string(),
            title: "Test".to_string(),
            status: TaskStatus::InProgress,
            url: None,
        };
        assert_eq!(task.status_symbol(), "⏳");

        let task = Task {
            id: "123".to_string(),
            title: "Test".to_string(),
            status: TaskStatus::Done,
            url: None,
        };
        assert_eq!(task.status_symbol(), "✓");
    }
} 