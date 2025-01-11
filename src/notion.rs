use crate::config::Config;
use anyhow::Result;
use reqwest::Client;
use serde_json;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub status: TaskStatus,
    pub url: Option<String>,
    pub priority: Option<TaskPriority>,
    pub due_date: Option<String>,
    pub tags: Vec<String>,
    pub description: Option<String>,
}

#[allow(dead_code)]
impl Task {
    pub fn status_symbol(&self) -> &str {
        self.status.symbol()
    }

    pub fn priority_symbol(&self) -> &str {
        match &self.priority {
            Some(priority) => priority.symbol(),
            None => " ",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
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

impl TaskStatus {
    pub fn symbol(&self) -> &str {
        match self {
            TaskStatus::NotStarted => "⭕",
            TaskStatus::InProgress => "🔄",
            TaskStatus::Done => "✅",
        }
    }
}

impl std::str::FromStr for TaskStatus {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "not started" => Ok(TaskStatus::NotStarted),
            "in progress" => Ok(TaskStatus::InProgress),
            "done" => Ok(TaskStatus::Done),
            _ => Err(anyhow::anyhow!("Invalid task status")),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TaskPriority {
    High,
    Medium,
    Low,
}

impl fmt::Display for TaskPriority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TaskPriority::High => write!(f, "High"),
            TaskPriority::Medium => write!(f, "Medium"),
            TaskPriority::Low => write!(f, "Low"),
        }
    }
}

impl TaskPriority {
    pub fn symbol(&self) -> &str {
        match self {
            TaskPriority::High => "🔴",
            TaskPriority::Medium => "🟡",
            TaskPriority::Low => "🟢",
        }
    }
}

impl std::str::FromStr for TaskPriority {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "high" => Ok(TaskPriority::High),
            "medium" => Ok(TaskPriority::Medium),
            "low" => Ok(TaskPriority::Low),
            _ => Err(anyhow::anyhow!("Invalid task priority")),
        }
    }
}

pub struct NotionClient {
    client: Client,
    config: Config,
    api_url: String,
}

impl NotionClient {
    pub fn new(config: Config) -> Result<Self> {
        Ok(Self {
            config,
            client: reqwest::Client::new(),
            api_url: "https://api.notion.com".to_string(),
        })
    }

    pub fn new_with_base_url(config: Config, base_url: String) -> Result<Self> {
        Ok(Self {
            config,
            client: reqwest::Client::new(),
            api_url: base_url,
        })
    }

    pub async fn add_task(&self, title: &str) -> Result<Task> {
        let url = format!("{}/v1/pages", self.api_url);
        let body = serde_json::json!({
            "parent": { "database_id": self.config.database_id },
            "properties": {
                "Name": {
                    "title": [
                        {
                            "type": "text",
                            "text": {
                                "content": title,
                                "link": null
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

        let response = self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.config.notion_token))
            .header("Notion-Version", "2022-06-28")
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        let task = Task {
            id: response["id"].as_str().unwrap_or_default().to_string(),
            title: response["properties"]["Name"]["title"][0]["text"]["content"]
                .as_str()
                .unwrap_or_default()
                .to_string(),
            status: TaskStatus::NotStarted,
            url: response["url"].as_str().map(|s| s.to_string()),
            priority: None,
            due_date: None,
            tags: Vec::new(),
            description: None,
        };

        Ok(task)
    }

    pub async fn list_tasks(&self) -> Result<Vec<Task>> {
        let url = format!("{}/v1/databases/{}/query", self.api_url, self.config.database_id);
        let response = self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.config.notion_token))
            .header("Notion-Version", "2022-06-28")
            .header("Content-Type", "application/json")
            .json(&serde_json::json!({}))
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        let mut tasks = Vec::new();
        if let Some(results) = response["results"].as_array() {
            for result in results {
                let status = result["properties"]["Status"]["status"]["name"]
                    .as_str()
                    .unwrap_or("Not started")
                    .parse::<TaskStatus>()?;

                let priority = result["properties"]["Priority"]["select"]["name"]
                    .as_str()
                    .map(|p| p.parse::<TaskPriority>().ok())
                    .flatten();

                let due_date = result["properties"]["Due Date"]["date"]["start"]
                    .as_str()
                    .map(|s| s.to_string());

                let tags = result["properties"]["Tags"]["multi_select"]
                    .as_array()
                    .map(|tags| {
                        tags.iter()
                            .filter_map(|tag| tag["name"].as_str())
                            .map(|s| s.to_string())
                            .collect()
                    })
                    .unwrap_or_default();

                let description = result["properties"]["Description"]["rich_text"]
                    .as_array()
                    .and_then(|texts| texts.first())
                    .and_then(|text| text["text"]["content"].as_str())
                    .map(|s| s.to_string());

                let task = Task {
                    id: result["id"].as_str().unwrap_or_default().to_string(),
                    title: result["properties"]["Name"]["title"][0]["text"]["content"]
                        .as_str()
                        .unwrap_or_default()
                        .to_string(),
                    status,
                    url: result["url"].as_str().map(|s| s.to_string()),
                    priority,
                    due_date,
                    tags,
                    description,
                };
                tasks.push(task);
            }
        }

        Ok(tasks)
    }

    pub async fn update_task_status(&self, task_id: &str, status: TaskStatus) -> Result<Task> {
        let url = format!("{}/v1/pages/{}", self.api_url, task_id);
        let body = serde_json::json!({
            "properties": {
                "Status": {
                    "status": {
                        "name": status.to_string()
                    }
                }
            }
        });

        let response = self.client
            .patch(&url)
            .header("Authorization", format!("Bearer {}", self.config.notion_token))
            .header("Notion-Version", "2022-06-28")
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        let task = Task {
            id: response["id"].as_str().unwrap_or_default().to_string(),
            title: response["properties"]["Name"]["title"][0]["text"]["content"]
                .as_str()
                .unwrap_or_default()
                .to_string(),
            status,
            url: response["url"].as_str().map(|s| s.to_string()),
            priority: None,
            due_date: None,
            tags: Vec::new(),
            description: None,
        };

        Ok(task)
    }

    pub async fn set_task_priority(&self, task_id: &str, priority: TaskPriority) -> Result<Task> {
        let url = format!("{}/v1/pages/{}", self.api_url, task_id);
        let body = serde_json::json!({
            "properties": {
                "Priority": {
                    "select": {
                        "name": priority.to_string()
                    }
                }
            }
        });

        let response = self.client
            .patch(&url)
            .header("Authorization", format!("Bearer {}", self.config.notion_token))
            .header("Notion-Version", "2022-06-28")
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        let task = Task {
            id: response["id"].as_str().unwrap_or_default().to_string(),
            title: response["properties"]["Name"]["title"][0]["text"]["content"]
                .as_str()
                .unwrap_or_default()
                .to_string(),
            status: TaskStatus::NotStarted,
            url: response["url"].as_str().map(|s| s.to_string()),
            priority: Some(priority),
            due_date: None,
            tags: Vec::new(),
            description: None,
        };

        Ok(task)
    }

    pub async fn set_task_due_date(&self, task_id: &str, due_date: &str) -> Result<Task> {
        let url = format!("{}/v1/pages/{}", self.api_url, task_id);
        let body = serde_json::json!({
            "properties": {
                "Due Date": {
                    "date": {
                        "start": due_date
                    }
                }
            }
        });

        let response = self.client
            .patch(&url)
            .header("Authorization", format!("Bearer {}", self.config.notion_token))
            .header("Notion-Version", "2022-06-28")
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        let task = Task {
            id: response["id"].as_str().unwrap_or_default().to_string(),
            title: response["properties"]["Name"]["title"][0]["text"]["content"]
                .as_str()
                .unwrap_or_default()
                .to_string(),
            status: TaskStatus::NotStarted,
            url: response["url"].as_str().map(|s| s.to_string()),
            priority: None,
            due_date: Some(due_date.to_string()),
            tags: Vec::new(),
            description: None,
        };

        Ok(task)
    }

    pub async fn set_task_description(&self, task_id: &str, description: &str) -> Result<Task> {
        let url = format!("{}/v1/pages/{}", self.api_url, task_id);
        let body = serde_json::json!({
            "properties": {
                "Description": {
                    "rich_text": [
                        {
                            "type": "text",
                            "text": {
                                "content": description,
                                "link": null
                            }
                        }
                    ]
                }
            }
        });

        let response = self.client
            .patch(&url)
            .header("Authorization", format!("Bearer {}", self.config.notion_token))
            .header("Notion-Version", "2022-06-28")
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        let task = Task {
            id: response["id"].as_str().unwrap_or_default().to_string(),
            title: response["properties"]["Name"]["title"][0]["text"]["content"]
                .as_str()
                .unwrap_or_default()
                .to_string(),
            status: TaskStatus::NotStarted,
            url: response["url"].as_str().map(|s| s.to_string()),
            priority: None,
            due_date: None,
            tags: Vec::new(),
            description: Some(description.to_string()),
        };

        Ok(task)
    }

    pub async fn add_task_tags(&self, task_id: &str, tags: &str) -> Result<Task> {
        let url = format!("{}/v1/pages/{}", self.api_url, task_id);
        let tag_list: Vec<String> = tags.split(',')
            .map(|s| s.trim().to_string())
            .collect();

        let tag_objects: Vec<serde_json::Value> = tag_list.iter()
            .map(|tag| serde_json::json!({"name": tag}))
            .collect();

        let body = serde_json::json!({
            "properties": {
                "Tags": {
                    "multi_select": tag_objects
                }
            }
        });

        let response = self.client
            .patch(&url)
            .header("Authorization", format!("Bearer {}", self.config.notion_token))
            .header("Notion-Version", "2022-06-28")
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        let task = Task {
            id: response["id"].as_str().unwrap_or_default().to_string(),
            title: response["properties"]["Name"]["title"][0]["text"]["content"]
                .as_str()
                .unwrap_or_default()
                .to_string(),
            status: TaskStatus::NotStarted,
            url: response["url"].as_str().map(|s| s.to_string()),
            priority: None,
            due_date: None,
            tags: tag_list,
            description: None,
        };

        Ok(task)
    }

    pub async fn delete_task(&self, task_id: &str) -> Result<()> {
        let url = format!("{}/v1/pages/{}", self.api_url, task_id);
        let body = serde_json::json!({
            "archived": true
        });

        self.client
            .patch(&url)
            .header("Authorization", format!("Bearer {}", self.config.notion_token))
            .header("Notion-Version", "2022-06-28")
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_creation() {
        let task = Task {
            id: "123".to_string(),
            title: "Test task".to_string(),
            status: TaskStatus::NotStarted,
            url: Some("https://notion.so/123".to_string()),
            priority: None,
            due_date: None,
            tags: Vec::new(),
            description: None,
        };

        assert_eq!(task.id, "123");
        assert_eq!(task.title, "Test task");
        assert_eq!(task.status, TaskStatus::NotStarted);
        assert_eq!(task.url, Some("https://notion.so/123".to_string()));
    }

    #[test]
    fn test_task_status_display() {
        assert_eq!(TaskStatus::NotStarted.to_string(), "Not started");
        assert_eq!(TaskStatus::InProgress.to_string(), "In progress");
        assert_eq!(TaskStatus::Done.to_string(), "Done");
    }

    #[test]
    fn test_task_status_symbol() {
        assert_eq!(TaskStatus::NotStarted.symbol(), "⭕");
        assert_eq!(TaskStatus::InProgress.symbol(), "🔄");
        assert_eq!(TaskStatus::Done.symbol(), "✅");
    }

    #[test]
    fn test_task_status_from_str() {
        assert_eq!("not started".parse::<TaskStatus>().unwrap(), TaskStatus::NotStarted);
        assert_eq!("in progress".parse::<TaskStatus>().unwrap(), TaskStatus::InProgress);
        assert_eq!("done".parse::<TaskStatus>().unwrap(), TaskStatus::Done);
        assert!("invalid".parse::<TaskStatus>().is_err());
    }

    #[test]
    fn test_priority_display() {
        assert_eq!(TaskPriority::High.to_string(), "High");
        assert_eq!(TaskPriority::Medium.to_string(), "Medium");
        assert_eq!(TaskPriority::Low.to_string(), "Low");
    }

    #[test]
    fn test_priority_from_str() {
        assert_eq!("high".parse::<TaskPriority>().unwrap(), TaskPriority::High);
        assert_eq!("medium".parse::<TaskPriority>().unwrap(), TaskPriority::Medium);
        assert_eq!("low".parse::<TaskPriority>().unwrap(), TaskPriority::Low);
        assert!("invalid".parse::<TaskPriority>().is_err());
    }

    #[test]
    fn test_task_priority_symbol() {
        assert_eq!(TaskPriority::High.symbol(), "🔴");
        assert_eq!(TaskPriority::Medium.symbol(), "🟡");
        assert_eq!(TaskPriority::Low.symbol(), "🟢");
    }
} 