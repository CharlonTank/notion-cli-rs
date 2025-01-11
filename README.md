# Notion CLI (Rust)

A powerful command-line interface for managing tasks in Notion, written in Rust. This CLI tool allows you to manage your Notion tasks directly from your terminal with rich features and intuitive commands.

## Features

- Add new tasks with rich metadata:
  - Priority levels (High, Medium, Low) 🔴 🟡 🟢
  - Due dates 📅
  - Tags 🏷️
  - Descriptions 📝
- List tasks with advanced filtering and sorting:
  - Filter by status (Not started ⭕, In progress 🔄, Done ✅)
  - Filter by priority
  - Filter by tags
  - Sort by due date
  - Colored output in terminal
- Update task properties:
  - Change task status
  - Set/update priority levels
  - Set/update due dates
  - Add/update tags
  - Set/update descriptions
- Delete tasks

## Prerequisites

- Rust and Cargo installed on your system ([Install Rust](https://www.rust-lang.org/tools/install))
- A Notion account
- A Notion integration token
- A properly configured Notion database

## Installation

1. Clone the repository:
```bash
git clone https://github.com/CharlonTank/notion-cli-rs.git
cd notion-cli-rs
```

2. Build the project:
```bash
cargo build --release
```

3. Install the binary:

### Unix-like Systems (Linux/macOS)

#### Option A: Local User Installation
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

# Alternative for older Fish versions (< 3.2.0)
# echo 'set -x PATH $HOME/.local/bin $PATH' >> ~/.config/fish/config.fish
# source ~/.config/fish/config.fish
```

#### Option B: System-wide Installation
```bash
# Requires sudo privileges
sudo cp target/release/notion-cli-rs /usr/local/bin/
```

### Windows

#### Option A: Manual Installation
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

#### Option B: Using Scoop
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

### Verify Installation

After installation, verify the CLI is properly installed:
```bash
# If added to PATH
notion-cli-rs --help

# Or using cargo
cargo run -- --help
```

## Notion Setup (Detailed Guide)

1. Create a Notion Integration:
   - Visit [Notion Integrations](https://www.notion.so/my-integrations)
   - Click "New integration"
   - Name: Enter a name (e.g., "Task Manager CLI")
   - Capabilities needed:
     - ✅ Read content
     - ✅ Update content
     - ✅ Insert content
     - ✅ Delete content
   - Click "Submit"
   - Copy the "Internal Integration Token" (starts with `secret_`)

2. Create a Notion Database:
   - Open Notion
   - Click "+ New page" in the sidebar
   - Click "Table" at the top
   - Add the following properties (exact names are important):
     - "Name" (already exists, title type)
     - Click "+ Add a property" for each:
       - "Status" (Select type)
         - Options: "Not started", "In progress", "Done"
         - Colors: Gray, Blue, Green (recommended)
       - "Priority" (Select type)
         - Options: "High", "Medium", "Low"
         - Colors: Red, Yellow, Green (recommended)
       - "Due Date" (Date type)
       - "Tags" (Multi-select type)
         - Add some initial tags (e.g., "work", "personal", "urgent")
       - "Description" (Text type)

3. Share Database with Integration:
   - In your database view, click "•••" (three dots) in the top-right corner
   - Click "Add connections"
   - Find and select your integration name
   - Click "Confirm"

4. Get Database ID:
   - Open your database in Notion
   - Look at the URL: `https://www.notion.so/workspace/XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX?v=...`
   - Copy the 32-character ID (marked as X's above)

5. Configure Environment:
```bash
# Copy the example environment file
cp .env.example .env

# Edit the .env file with your details
NOTION_TOKEN=secret_your_integration_token_here
NOTION_DATABASE_ID=your_database_id_here
LOCAL_TIMEZONE=America/New_York  # Use your timezone from the TZ database
```

## Usage Guide

### Basic Task Management

```bash
# Add a simple task
cargo run -- add "Buy groceries"

# Add a detailed task
cargo run -- add "Quarterly report" \
  --priority high \
  --due "2024-01-20" \
  --tags "work,reports,q4" \
  --description "Prepare Q4 2023 financial report for stakeholders"

# List all tasks
cargo run -- list

# List tasks with filters
cargo run -- list --status "in progress" --priority high

# Update task status
cargo run -- status <task-id> "in progress"
cargo run -- status <task-id> "done"

# Delete a task
cargo run -- delete <task-id>
```

### Advanced Task Management

```bash
# Set/update task priority
cargo run -- priority <task-id> high

# Set/update due date
cargo run -- due-date <task-id> "2024-01-20"

# Add/update tags
cargo run -- tags <task-id> "urgent,priority,q4"

# Set/update description
cargo run -- description <task-id> "Detailed task description here"
```

### Filtering and Sorting

```bash
# List high priority tasks
cargo run -- list --priority high

# List in-progress tasks
cargo run -- list --status "in progress"

# List tasks with specific tag
cargo run -- list --tag work

# List tasks sorted by due date
cargo run -- list --sort-by-due
```

## Task Properties

- **Status Options:**
  - "Not started" (⭕)
  - "In progress" (🔄)
  - "Done" (✅)

- **Priority Levels:**
  - "High" (🔴)
  - "Medium" (🟡)
  - "Low" (🟢)

## Troubleshooting

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
cargo run -- --help

# Show help for specific command
cargo run -- add --help
cargo run -- list --help
# etc.
```

## Development

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name -- --nocapture
```

### Building for Release

```bash
cargo build --release
```

The binary will be available at `target/release/notion-cli-rs`

## License

This project is licensed under the MIT License - see the LICENSE file for details.

