use notion_cli_rs::{Config, NotionClient, TaskStatus, TaskPriority};
use anyhow::Result;
use mockito;
use std::sync::Once;
use tokio::runtime::Runtime;

static INIT: Once = Once::new();

fn setup_test_client(mock_server: &mockito::Server) -> Result<(NotionClient, Runtime)> {
    // Initialize environment only once
    INIT.call_once(|| {
        std::env::set_var("RUST_LOG", "debug");
        env_logger::init();
    });

    let config = Config {
        notion_token: "test-token".to_string(),
        database_id: "database-id".to_string(),
    };

    // Create a new Runtime for each test
    let rt = Runtime::new()?;
    
    // Create a new client with the mock server URL
    let client = NotionClient::new_with_base_url(config, mock_server.url())?;
    
    Ok((client, rt))
}

#[test]
fn test_add_task() -> Result<()> {
    let mut mock_server = mockito::Server::new();
    let mock_url = mock_server.url();
    println!("Mock server URL: {}", mock_url);

    let expected_body = serde_json::json!({
        "parent": { "database_id": "database-id" },
        "properties": {
            "Name": {
                "title": [
                    {
                        "type": "text",
                        "text": {
                            "content": "Test task",
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

    println!("Setting up mock...");
    let _mock = mock_server.mock("POST", "/v1/pages")
        .match_header("authorization", "Bearer test-token")
        .match_header("notion-version", "2022-06-28")
        .match_header("content-type", "application/json")
        .match_body(mockito::Matcher::Json(expected_body))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(serde_json::json!({
            "object": "page",
            "id": "task-id",
            "created_time": "2024-01-20T12:00:00.000Z",
            "last_edited_time": "2024-01-20T12:00:00.000Z",
            "url": "https://notion.so/task-id",
            "parent": {
                "type": "database_id",
                "database_id": "database-id"
            },
            "properties": {
                "Name": {
                    "id": "title",
                    "type": "title",
                    "title": [
                        {
                            "type": "text",
                            "text": {
                                "content": "Test task",
                                "link": null
                            },
                            "plain_text": "Test task"
                        }
                    ]
                },
                "Status": {
                    "id": "status",
                    "type": "status",
                    "status": {
                        "id": "not-started",
                        "name": "Not started",
                        "color": "default"
                    }
                }
            }
        }).to_string())
        .create();

    println!("Mock created");
    let (client, rt) = setup_test_client(&mock_server)?;
    println!("Client created");

    println!("Making request...");
    let task = rt.block_on(client.add_task("Test task"))?;
    println!("Request completed successfully");

    assert_eq!(task.id, "task-id");
    assert_eq!(task.title, "Test task");
    assert_eq!(task.status, TaskStatus::NotStarted);
    assert_eq!(task.url, Some("https://notion.so/task-id".to_string()));

    Ok(())
}

#[test]
fn test_list_tasks() -> Result<()> {
    let mut mock_server = mockito::Server::new();
    let mock_url = mock_server.url();
    println!("Mock server URL: {}", mock_url);

    println!("Setting up list_tasks mock...");
    let _mock = mock_server.mock("POST", "/v1/databases/database-id/query")
        .match_header("authorization", "Bearer test-token")
        .match_header("notion-version", "2022-06-28")
        .match_header("content-type", "application/json")
        .match_body(mockito::Matcher::Json(serde_json::json!({})))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(serde_json::json!({
            "object": "list",
            "results": [
                {
                    "object": "page",
                    "id": "task-id-1",
                    "created_time": "2024-01-20T12:00:00.000Z",
                    "last_edited_time": "2024-01-20T12:00:00.000Z",
                    "url": "https://notion.so/task-id-1",
                    "parent": {
                        "type": "database_id",
                        "database_id": "database-id"
                    },
                    "properties": {
                        "Name": {
                            "id": "title",
                            "type": "title",
                            "title": [
                                {
                                    "type": "text",
                                    "text": {
                                        "content": "Task 1",
                                        "link": null
                                    },
                                    "plain_text": "Task 1"
                                }
                            ]
                        },
                        "Status": {
                            "id": "status",
                            "type": "status",
                            "status": {
                                "id": "not-started",
                                "name": "Not started",
                                "color": "default"
                            }
                        }
                    }
                }
            ],
            "next_cursor": null,
            "has_more": false
        }).to_string())
        .create();

    println!("Mock created");
    let (client, rt) = setup_test_client(&mock_server)?;
    println!("Client created");

    println!("Making list_tasks request...");
    let tasks = rt.block_on(client.list_tasks())?;
    println!("Request completed successfully");

    assert_eq!(tasks.len(), 1);
    assert_eq!(tasks[0].id, "task-id-1");
    assert_eq!(tasks[0].title, "Task 1");
    assert_eq!(tasks[0].status, TaskStatus::NotStarted);
    assert_eq!(tasks[0].url, Some("https://notion.so/task-id-1".to_string()));

    Ok(())
}

#[test]
fn test_update_task_status() -> Result<()> {
    let mut mock_server = mockito::Server::new();
    let mock_url = mock_server.url();
    println!("Mock server URL: {}", mock_url);

    let expected_body = serde_json::json!({
        "properties": {
            "Status": {
                "status": {
                    "name": "In progress"
                }
            }
        }
    });

    println!("Setting up update_task_status mock...");
    let _mock = mock_server.mock("PATCH", "/v1/pages/task-id")
        .match_header("authorization", "Bearer test-token")
        .match_header("notion-version", "2022-06-28")
        .match_header("content-type", "application/json")
        .match_body(mockito::Matcher::Json(expected_body))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(serde_json::json!({
            "object": "page",
            "id": "task-id",
            "created_time": "2024-01-20T12:00:00.000Z",
            "last_edited_time": "2024-01-20T12:00:00.000Z",
            "properties": {
                "Status": {
                    "id": "status",
                    "type": "status",
                    "status": {
                        "id": "in-progress",
                        "name": "In progress",
                        "color": "blue"
                    }
                }
            }
        }).to_string())
        .create();

    println!("Mock created");
    let (client, rt) = setup_test_client(&mock_server)?;
    println!("Client created");

    println!("Making update_task_status request...");
    rt.block_on(client.update_task_status("task-id", TaskStatus::InProgress))?;
    println!("Request completed successfully");

    Ok(())
}

#[test]
fn test_add_task_tags() -> Result<()> {
    let mut mock_server = mockito::Server::new();
    let mock_url = mock_server.url();
    println!("Mock server URL: {}", mock_url);

    let expected_body = serde_json::json!({
        "properties": {
            "Tags": {
                "multi_select": [
                    { "name": "work" },
                    { "name": "urgent" }
                ]
            }
        }
    });

    println!("Setting up add_task_tags mock...");
    let _mock = mock_server.mock("PATCH", "/v1/pages/task-id")
        .match_header("authorization", "Bearer test-token")
        .match_header("notion-version", "2022-06-28")
        .match_header("content-type", "application/json")
        .match_body(mockito::Matcher::Json(expected_body))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(serde_json::json!({
            "object": "page",
            "id": "task-id",
            "created_time": "2024-01-20T12:00:00.000Z",
            "last_edited_time": "2024-01-20T12:00:00.000Z",
            "properties": {
                "Tags": {
                    "id": "tags",
                    "type": "multi_select",
                    "multi_select": [
                        {
                            "id": "work-id",
                            "name": "work",
                            "color": "blue"
                        },
                        {
                            "id": "urgent-id",
                            "name": "urgent",
                            "color": "red"
                        }
                    ]
                }
            }
        }).to_string())
        .create();

    println!("Mock created");
    let (client, rt) = setup_test_client(&mock_server)?;
    println!("Client created");

    println!("Making add_task_tags request...");
    rt.block_on(client.add_task_tags("task-id", "work, urgent"))?;
    println!("Request completed successfully");

    Ok(())
}

#[test]
fn test_set_task_priority() -> Result<()> {
    let mut mock_server = mockito::Server::new();
    let mock_url = mock_server.url();
    println!("Mock server URL: {}", mock_url);

    let expected_body = serde_json::json!({
        "properties": {
            "Priority": {
                "select": {
                    "name": "High"
                }
            }
        }
    });

    println!("Setting up set_task_priority mock...");
    let _mock = mock_server.mock("PATCH", "/v1/pages/task-id")
        .match_header("authorization", "Bearer test-token")
        .match_header("notion-version", "2022-06-28")
        .match_header("content-type", "application/json")
        .match_body(mockito::Matcher::Json(expected_body))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(serde_json::json!({
            "object": "page",
            "id": "task-id",
            "created_time": "2024-01-20T12:00:00.000Z",
            "last_edited_time": "2024-01-20T12:00:00.000Z",
            "properties": {
                "Priority": {
                    "id": "priority",
                    "type": "select",
                    "select": {
                        "id": "high-id",
                        "name": "High",
                        "color": "red"
                    }
                },
                "Name": {
                    "id": "title",
                    "type": "title",
                    "title": [
                        {
                            "type": "text",
                            "text": {
                                "content": "Test task",
                                "link": null
                            },
                            "plain_text": "Test task"
                        }
                    ]
                }
            }
        }).to_string())
        .create();

    println!("Mock created");
    let (client, rt) = setup_test_client(&mock_server)?;
    println!("Client created");

    println!("Making set_task_priority request...");
    let task = rt.block_on(client.set_task_priority("task-id", TaskPriority::High))?;
    println!("Request completed successfully");

    assert_eq!(task.priority, Some(TaskPriority::High));
    Ok(())
}

#[test]
fn test_set_task_due_date() -> Result<()> {
    let mut mock_server = mockito::Server::new();
    let mock_url = mock_server.url();
    println!("Mock server URL: {}", mock_url);

    let expected_body = serde_json::json!({
        "properties": {
            "Due Date": {
                "date": {
                    "start": "2024-01-20"
                }
            }
        }
    });

    println!("Setting up set_task_due_date mock...");
    let _mock = mock_server.mock("PATCH", "/v1/pages/task-id")
        .match_header("authorization", "Bearer test-token")
        .match_header("notion-version", "2022-06-28")
        .match_header("content-type", "application/json")
        .match_body(mockito::Matcher::Json(expected_body))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(serde_json::json!({
            "object": "page",
            "id": "task-id",
            "created_time": "2024-01-20T12:00:00.000Z",
            "last_edited_time": "2024-01-20T12:00:00.000Z",
            "properties": {
                "Due Date": {
                    "id": "due_date",
                    "type": "date",
                    "date": {
                        "start": "2024-01-20",
                        "end": null,
                        "time_zone": null
                    }
                }
            }
        }).to_string())
        .create();

    println!("Mock created");
    let (client, rt) = setup_test_client(&mock_server)?;
    println!("Client created");

    println!("Making set_task_due_date request...");
    rt.block_on(client.set_task_due_date("task-id", "2024-01-20"))?;
    println!("Request completed successfully");

    Ok(())
}

#[test]
fn test_set_task_description() -> Result<()> {
    let mut mock_server = mockito::Server::new();
    let mock_url = mock_server.url();
    println!("Mock server URL: {}", mock_url);

    let expected_body = serde_json::json!({
        "properties": {
            "Description": {
                "rich_text": [{
                    "type": "text",
                    "text": {
                        "content": "Test description",
                        "link": null
                    }
                }]
            }
        }
    });

    println!("Setting up set_task_description mock...");
    let _mock = mock_server.mock("PATCH", "/v1/pages/task-id")
        .match_header("authorization", "Bearer test-token")
        .match_header("notion-version", "2022-06-28")
        .match_header("content-type", "application/json")
        .match_body(mockito::Matcher::Json(expected_body))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(serde_json::json!({
            "object": "page",
            "id": "task-id",
            "created_time": "2024-01-20T12:00:00.000Z",
            "last_edited_time": "2024-01-20T12:00:00.000Z",
            "properties": {
                "Description": {
                    "id": "description",
                    "type": "rich_text",
                    "rich_text": [{
                        "type": "text",
                        "text": {
                            "content": "Test description",
                            "link": null
                        },
                        "plain_text": "Test description",
                        "annotations": {
                            "bold": false,
                            "italic": false,
                            "strikethrough": false,
                            "underline": false,
                            "code": false,
                            "color": "default"
                        }
                    }]
                }
            }
        }).to_string())
        .create();

    println!("Mock created");
    let (client, rt) = setup_test_client(&mock_server)?;
    println!("Client created");

    println!("Making set_task_description request...");
    rt.block_on(client.set_task_description("task-id", "Test description"))?;
    println!("Request completed successfully");

    Ok(())
} 