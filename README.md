# 📋 Notion CLI (Rust)

<div align="center">

[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Notion](https://img.shields.io/badge/Notion-%23000000.svg?style=for-the-badge&logo=notion&logoColor=white)](https://www.notion.so/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=for-the-badge)](https://opensource.org/licenses/MIT)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg?style=for-the-badge)](http://makeapullrequest.com)
[![Minimum Rust Version](https://img.shields.io/badge/rust-1.70%2B-orange.svg?style=for-the-badge)](https://www.rust-lang.org/)
[![Crates.io](https://img.shields.io/crates/v/notion-cli-rs?style=for-the-badge)](https://crates.io/crates/notion-cli-rs)

A powerful command-line interface for managing Notion tasks, written in Rust. 
Streamline your task management workflow directly from your terminal.

[Installation](#installation) •
[Features](#features) •
[Setup Guide](#notion-setup-detailed-guide) •
[Usage](#usage-guide) •
[Contributing](#contributing)

---

</div>

## 📦 Dependencies

This project relies on the following major dependencies:

- **reqwest** (0.11): HTTP client for making API requests
- **tokio** (1.0): Async runtime for handling concurrent operations
- **serde** (1.0): Serialization/deserialization of JSON data
- **clap** (4.0): Command-line argument parsing
- **anyhow** (1.0): Error handling
- **dotenv** (0.15): Environment variable management

For development:
- **mockito** (1.2): HTTP mocking for tests
- **env_logger** (0.10): Logging during development

## 🔧 Requirements

- Rust 1.70 or higher
- Notion API Version: 2022-06-28
- Unix-like OS (Linux/macOS) or Windows
- Notion account with integration capabilities

## ✨ Features

<div align="center">
<table>
<tr>
<td>

### 📝 Task Creation
- Create tasks with rich metadata
- Set priority levels 🔴 🟡 🟢
- Add due dates 📅
- Assign tags 🏷️
- Include descriptions 📝

</td>
<td>

### 📊 Task Management
- Update task status ⭕ 🔄 ✅
- Modify priorities
- Change due dates
- Edit descriptions
- Manage tags

</td>
</tr>
<tr>
<td>

### 🔍 Advanced Filtering
- Filter by status
- Filter by priority
- Filter by tags
- Sort by due date
- Colored terminal output

</td>
<td>

### ⚡ Quick Actions
- Mark tasks complete
- Set task priorities
- Update task status
- Delete tasks
- Bulk operations

</td>
</tr>
</table>
</div>

## 🚀 Quick Start

### Prerequisites

<details>
<summary>Click to expand</summary>

- [Rust and Cargo](https://www.rust-lang.org/tools/install)
- A Notion account
- Notion integration token
- Configured Notion database

</details>

### Installation

#### Option 1: Install from crates.io (Recommended)
```bash
cargo install notion-cli-rs
```

#### Option 2: Install from source
```bash
git clone https://github.com/CharlonTank/notion-cli-rs.git
cd notion-cli-rs
cargo install --path .
```

#### Option 3: One-Line Installation Script (Unix-like Systems)
```bash
curl -sSL https://raw.githubusercontent.com/CharlonTank/notion-cli-rs/master/install.sh | bash
```

<details>
<summary>Additional Installation Options</summary>

### Unix-like Systems (Linux/macOS)

#### Local User Installation
```bash
# Create local bin directory if it doesn't exist
mkdir -p ~/.local/bin

# Copy the binary
cp target/release/notion-cli-rs ~/.local/bin/

# Add to PATH in your shell's configuration file:

# For Bash (add to ~/.bashrc or ~/.bash_profile)
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc

# For Zsh (add to ~/.zshrc)
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.zshrc
source ~/.zshrc

# For Fish (recommended way)
fish_add_path ~/.local/bin
```

#### System-wide Installation
```bash
# Requires sudo privileges
sudo cp target/release/notion-cli-rs /usr/local/bin/
```

### Windows Installation

#### Manual Installation
```powershell
# Create a directory for the binary
mkdir -p "$env:USERPROFILE\bin"

# Copy the binary
copy "target\release\notion-cli-rs.exe" "$env:USERPROFILE\bin"

# Add to PATH in PowerShell (current session)
$env:PATH += ";$env:USERPROFILE\bin"

# Add to PATH permanently (run in PowerShell as Administrator)
[Environment]::SetEnvironmentVariable(
    "Path",
    [Environment]::GetEnvironmentVariable("Path", "User") + ";$env:USERPROFILE\bin",
    "User"
)
```

#### Using Scoop
```powershell
# Install Scoop if not already installed
Set-ExecutionPolicy RemoteSigned -Scope CurrentUser
irm get.scoop.sh | iex

# Create and switch to a new directory for the package
mkdir ~/.scoop/apps/notion-cli-rs
cd ~/.scoop/apps/notion-cli-rs

# Copy the binary
copy "target\release\notion-cli-rs.exe" "notion-cli-rs.exe"

# Add to Scoop's shims
scoop shim add notion-cli-rs
```

</details>

### Verify Installation

After installation, verify the CLI is properly installed:
```bash
notion-cli-rs --help
```

## 🔧 Configuration

<details>
<summary>1. Create Notion Integration</summary>

1. Visit [Notion Integrations](https://www.notion.so/my-integrations)
2. Click "New integration"
3. Configure capabilities:
   - ✅ Read content
   - ✅ Update content
   - ✅ Insert content
   - ✅ Delete content
4. Copy the Integration Token

</details>

<details>
<summary>2. Setup Notion Database</summary>

Create a database with these properties:
| Property | Type | Options |
|----------|------|---------|
| Name | Title | - |
| Status | Select | Not started, In progress, Done |
| Priority | Select | High, Medium, Low |
| Due Date | Date | - |
| Tags | Multi-select | Custom tags |
| Description | Text | - |

</details>

<details>
<summary>3. Configure Environment</summary>

```bash
cp .env.example .env
```

```env
NOTION_TOKEN=secret_your_integration_token_here
NOTION_DATABASE_ID=your_database_id_here
LOCAL_TIMEZONE=America/New_York
```

</details>

## 📖 Usage Examples

### Basic Operations

```bash
# Create a task
notion-cli-rs add "Deploy new feature" \
  --priority high \
  --due "2024-01-20" \
  --tags "dev,feature" \
  --description "Deploy the new authentication system"

# List tasks
notion-cli-rs list

# Update status
notion-cli-rs status <task-id> "in progress"
```

<details>
<summary>View More Examples</summary>

## Usage Guide

### Basic Task Management

```bash
# Add a simple task
notion-cli-rs add "Buy groceries"

# Add a detailed task
notion-cli-rs add "Quarterly report" \
  --priority high \
  --due "2024-01-20" \
  --tags "work,reports,q4" \
  --description "Prepare Q4 2023 financial report for stakeholders"

# List all tasks
notion-cli-rs list

# List tasks with filters
notion-cli-rs list --status "in progress" --priority high

# Update task status
notion-cli-rs status <task-id> "in progress"
notion-cli-rs status <task-id> "done"

# Delete a task
notion-cli-rs delete <task-id>
```

### Advanced Task Management

```bash
# Set/update task priority
notion-cli-rs priority <task-id> high

# Set/update due date
notion-cli-rs due-date <task-id> "2024-01-20"

# Add/update tags
notion-cli-rs tags <task-id> "urgent,priority,q4"

# Set/update description
notion-cli-rs description <task-id> "Detailed task description here"
```

### Filtering and Sorting

```bash
# List high priority tasks
notion-cli-rs list --priority high

# List in-progress tasks
notion-cli-rs list --status "in progress"

# List tasks with specific tag
notion-cli-rs list --tag work

# List tasks sorted by due date
notion-cli-rs list --sort-by-due
```

### Getting Help

```bash
# Show general help
notion-cli-rs --help

# Show help for specific command
notion-cli-rs add --help
notion-cli-rs list --help
# etc.
```

</details>

## 🎯 Task Properties

<div align="center">

| Status | Symbol | | Priority | Symbol |
|--------|--------||-|----------|--------|
| Not Started | ⭕ | | High | 🔴 |
| In Progress | 🔄 | | Medium | 🟡 |
| Done | ✅ | | Low | 🟢 |

</div>

## 🔍 Troubleshooting

<details>
<summary>Common Issues and Solutions</summary>

### Common Issues:

1. **Authentication Error:**
   - Verify your `NOTION_TOKEN` in `.env`
   - Ensure the token starts with `secret_`
   - Check if the integration has access to the database

2. **Database Not Found:**
   - Verify your `NOTION_DATABASE_ID` in `.env`
   - Ensure the integration is connected to the database
   - Check if the database structure matches the requirements

3. **Command Not Found:**
   - If you haven't added the binary to PATH, use `cargo run -- <command>` instead of direct binary calls
   - Ensure you're in the project directory when using `cargo run`

4. **Invalid Property Values:**
   - Status must be exactly: "Not started", "In progress", or "Done"
   - Priority must be exactly: "High", "Medium", or "Low"
   - Due dates must be in YYYY-MM-DD format

### Getting Help

```bash
# Show general help
notion-cli-rs --help

# Show help for specific command
notion-cli-rs add --help
notion-cli-rs list --help
# etc.
```

</details>

## 🛠️ Development

<details>
<summary>Development Guidelines</summary>

### Running Tests
```bash
cargo test         # Run all tests
cargo test -- --nocapture  # With output
```

### Building
```bash
cargo build --release
```

</details>

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

<div align="center">

Made with ❤️ by [CharlonTank](https://github.com/CharlonTank)

</div>

## 📐 Project Structure

```
notion-cli-rs/
├── src/
│   ├── main.rs      # CLI entry point and command handling
│   ├── lib.rs       # Library interface
│   ├── notion.rs    # Notion API client implementation
│   └── config.rs    # Configuration management
├── tests/
│   └── integration_tests.rs  # Integration tests
├── .env.example     # Environment variables template
├── Cargo.toml       # Project dependencies
└── install.sh       # Installation script
```

## 🔌 Notion API

This CLI uses the Notion API v2022-06-28. For detailed API documentation, visit:
- [Notion API Reference](https://developers.notion.com/reference)
- [Notion API Guides](https://developers.notion.com/docs)

Key API endpoints used:
- `POST /v1/pages`: Create new tasks
- `PATCH /v1/pages/{id}`: Update task properties
- `POST /v1/databases/{id}/query`: List and filter tasks

## 🛠️ Development Setup

1. **Clone and Setup:**
```bash
git clone https://github.com/CharlonTank/notion-cli-rs.git
cd notion-cli-rs
cargo build
```

2. **Environment Setup:**
```bash
cp .env.example .env
# Edit .env with your test credentials
```

3. **Running Tests:**
```bash
# Unit tests
cargo test

# Integration tests (requires .env setup)
cargo test -- --test integration_tests

# With logging
RUST_LOG=debug cargo test -- --nocapture
```

4. **Code Style:**
- Follow Rust standard naming conventions
- Use `cargo fmt` before committing
- Run `cargo clippy` for linting
- Ensure all public items are documented

## ⚠️ Error Handling

### API Errors

| Error Code | Description | Solution |
|------------|-------------|----------|
| 401 | Unauthorized | Check your NOTION_TOKEN |
| 404 | Not Found | Verify database/page IDs |
| 409 | Conflict | Check for duplicate operations |
| 429 | Rate Limited | Implement backoff strategy |

### Common Issues

1. **Authentication Failures:**
```
Error: NOTION_TOKEN environment variable not set
Solution: Ensure .env file exists and contains valid token
```

2. **Database Access:**
```
Error: Could not access database
Solution: Check integration permissions and connection
```

3. **Invalid Properties:**
```
Error: Invalid task status
Solution: Use exact status values: "Not started", "In progress", "Done"
```

## 🔍 Debugging

Enable debug logging:
```bash
RUST_LOG=debug notion-cli-rs list
```

Common debug flags:
- `RUST_LOG=debug`: Detailed logging
- `RUST_BACKTRACE=1`: Full error backtraces
- `NOTION_API_URL`: Override API endpoint (testing)

## 📊 Performance

- Concurrent API requests where possible
- Connection pooling for multiple operations
- Efficient JSON parsing
- Minimal memory footprint

## 🔒 Security

- Tokens are never logged
- Environment variables for sensitive data
- HTTPS for all API communication
- No data caching by default

