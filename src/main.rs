mod config;
mod notion;

use clap::{Parser, Subcommand};
use anyhow::Result;
use colored::*;
use config::Config;
use notion::{NotionClient, TaskStatus};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new task to Notion
    Add {
        /// Task title
        #[arg(help = "Task title")]
        title: String,
    },
    /// List all tasks
    List,
    /// Mark a task as in progress
    Progress {
        /// Task ID
        #[arg(help = "Task ID")]
        id: String,
    },
    /// Mark a task as completed
    Check {
        /// Task ID
        #[arg(help = "Task ID")]
        id: String,
    },
    /// Mark a task as not started
    Uncheck {
        /// Task ID
        #[arg(help = "Task ID")]
        id: String,
    },
    /// Delete a task
    Delete {
        /// Task ID
        #[arg(help = "Task ID")]
        id: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    let config = Config::load()?;
    let client = NotionClient::new(config)?;
    let cli = Cli::parse();

    match cli.command {
        Commands::Add { title } => {
            let task = client.add_task(&title).await?;
            println!("Added task: {} (ID: {})", task.title.green(), task.id);
            if let Some(url) = task.url {
                println!("View in Notion: {}", url.blue().underline());
            }
        }
        Commands::List => {
            let tasks = client.list_tasks().await?;
            if tasks.is_empty() {
                println!("No tasks found");
                return Ok(());
            }

            for task in tasks {
                let status_color = match task.status {
                    TaskStatus::NotStarted => "red",
                    TaskStatus::InProgress => "yellow",
                    TaskStatus::Done => "green",
                };
                println!(
                    "[{}] {} (ID: {})",
                    task.status_symbol().color(status_color),
                    task.title,
                    task.id.dimmed()
                );
                if let Some(url) = task.url {
                    println!("    {}", url.blue().underline());
                }
            }
        }
        Commands::Progress { id } => {
            client.update_task_status(&id, TaskStatus::InProgress).await?;
            println!("Marked task as in progress");
        }
        Commands::Check { id } => {
            client.update_task_status(&id, TaskStatus::Done).await?;
            println!("Marked task as completed");
        }
        Commands::Uncheck { id } => {
            client.update_task_status(&id, TaskStatus::NotStarted).await?;
            println!("Marked task as not started");
        }
        Commands::Delete { id } => {
            client.delete_task(&id).await?;
            println!("Deleted task");
        }
    }

    Ok(())
}
