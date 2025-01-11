mod config;
mod notion;

use clap::{Parser, Subcommand};
use colored::Colorize;
use notion_cli_rs::{Config, NotionClient, TaskStatus, TaskPriority};
use anyhow::Result;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Add a new task")]
    Add {
        #[arg(help = "Task title")]
        title: String,
        #[arg(short, long, help = "Task priority (High, Medium, Low)")]
        priority: Option<String>,
        #[arg(short, long, help = "Task due date (YYYY-MM-DD)")]
        due_date: Option<String>,
        #[arg(short, long, help = "Task tags (comma-separated)")]
        tags: Option<String>,
        #[arg(short = 'D', long, help = "Task description")]
        description: Option<String>,
    },
    #[command(about = "List all tasks")]
    List {
        #[arg(short, long, help = "Filter by status (Not started, In progress, Done)")]
        status: Option<String>,
        #[arg(short, long, help = "Filter by priority (High, Medium, Low)")]
        priority: Option<String>,
        #[arg(short, long, help = "Filter by tag")]
        tag: Option<String>,
        #[arg(short = 'S', long, help = "Sort by due date")]
        sort_by_due_date: bool,
    },
    #[command(about = "Update task status")]
    Status {
        #[arg(help = "Task ID")]
        id: String,
        #[arg(help = "New status (Not started, In progress, Done)")]
        status: String,
    },
    #[command(about = "Delete a task")]
    Delete {
        #[arg(help = "Task ID")]
        id: String,
    },
    #[command(about = "Set task priority")]
    Priority {
        #[arg(help = "Task ID")]
        id: String,
        #[arg(help = "Priority (High, Medium, Low)")]
        priority: String,
    },
    #[command(about = "Set task due date")]
    DueDate {
        #[arg(help = "Task ID")]
        id: String,
        #[arg(help = "Due date (YYYY-MM-DD)")]
        date: String,
    },
    #[command(about = "Add tags to a task")]
    Tags {
        #[arg(help = "Task ID")]
        id: String,
        #[arg(help = "Tags (comma-separated)")]
        tags: String,
    },
    #[command(about = "Set task description")]
    Description {
        #[arg(help = "Task ID")]
        id: String,
        #[arg(help = "Description")]
        description: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let cli = Cli::parse();

    let config = Config {
        notion_token: std::env::var("NOTION_TOKEN")?,
        database_id: std::env::var("NOTION_DATABASE_ID")?,
    };

    let client = NotionClient::new(config)?;

    match &cli.command {
        Commands::Add { title, priority, due_date, tags, description } => {
            let mut task = client.add_task(title).await?;

            if let Some(p) = priority {
                let priority = p.parse::<TaskPriority>()?;
                task = client.set_task_priority(&task.id, priority).await?;
            }

            if let Some(d) = due_date {
                task = client.set_task_due_date(&task.id, d).await?;
            }

            if let Some(t) = tags {
                task = client.add_task_tags(&task.id, t).await?;
            }

            if let Some(d) = description {
                task = client.set_task_description(&task.id, d).await?;
            }

            println!("Task added successfully!");
            print_task(&task);
        }
        Commands::List { status, priority, tag, sort_by_due_date } => {
            let mut tasks = client.list_tasks().await?;

            if let Some(s) = status {
                let status = s.parse::<TaskStatus>()?;
                tasks.retain(|t| t.status == status);
            }

            if let Some(p) = priority {
                let priority = p.parse::<TaskPriority>()?;
                tasks.retain(|t| t.priority.as_ref().map_or(false, |tp| *tp == priority));
            }

            if let Some(tag) = tag {
                tasks.retain(|task| task.tags.iter().any(|t| t.to_lowercase().contains(&tag.to_lowercase())));
            }

            if *sort_by_due_date {
                tasks.sort_by(|a, b| a.due_date.cmp(&b.due_date));
            }

            if tasks.is_empty() {
                println!("No tasks found.");
                return Ok(());
            }

            for task in tasks {
                print_task(&task);
                println!();
            }
        }
        Commands::Status { id, status } => {
            let status = status.parse::<TaskStatus>()?;
            let task = client.update_task_status(id, status).await?;
            println!("Task status updated successfully!");
            print_task(&task);
        }
        Commands::Delete { id } => {
            client.delete_task(id).await?;
            println!("Task deleted successfully!");
        }
        Commands::Priority { id, priority } => {
            let priority = priority.parse::<TaskPriority>()?;
            let task = client.set_task_priority(id, priority).await?;
            println!("Task priority updated successfully!");
            print_task(&task);
        }
        Commands::DueDate { id, date } => {
            let task = client.set_task_due_date(id, date).await?;
            println!("Task due date updated successfully!");
            print_task(&task);
        }
        Commands::Tags { id, tags } => {
            let task = client.add_task_tags(id, tags).await?;
            println!("Task tags updated successfully!");
            print_task(&task);
        }
        Commands::Description { id, description } => {
            let task = client.set_task_description(id, description).await?;
            println!("Task description updated successfully!");
            print_task(&task);
        }
    }

    Ok(())
}

fn print_task(task: &notion_cli_rs::Task) {
    let status_color = match task.status {
        TaskStatus::NotStarted => "yellow",
        TaskStatus::InProgress => "blue",
        TaskStatus::Done => "green",
    };

    println!("ID: {}", task.id.bright_black());
    println!("Title: {}", task.title.bold());
    println!("Status: {} {}", 
        task.status_symbol().color(status_color),
        task.status.to_string().color(status_color)
    );
    println!("Priority: {}", task.priority_symbol());

    if let Some(url) = &task.url {
        println!("    URL: {}", url.bright_blue().underline());
    }

    if let Some(due) = &task.due_date {
        println!("    Due: {}", due.bright_yellow());
    }

    if !task.tags.is_empty() {
        println!("    Tags: {}", task.tags.join(", ").blue());
    }

    if let Some(desc) = &task.description {
        println!("    Description: {}", desc);
    }
}
