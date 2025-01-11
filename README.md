# Notion CLI (Rust)

A powerful command-line interface for managing tasks in Notion, written in Rust.

## Features

- Add new tasks with rich metadata:
  - Priority levels (High, Medium, Low)
  - Due dates
  - Tags
  - Descriptions
- List tasks with advanced filtering and sorting:
  - Filter by status, priority, or tags
  - Sort by due date
  - Colored output in terminal
  - Status indicators (✓ for done, ⏳ for in progress, ○ for not started)
  - Priority indicators (🔴 high, 🟡 medium, 🟢 low)
- Update task properties:
  - Mark tasks as "In Progress"
  - Mark tasks as completed/uncompleted
  - Set priority levels
  - Set due dates
  - Add tags
  - Set descriptions
- Delete tasks

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

## Configuration

1. Create a Notion integration:
   - Go to https://www.notion.so/my-integrations
   - Click "New integration"
   - Give it a name (e.g., "Task Manager CLI")
   - Copy the "Internal Integration Token"

2. Create a Notion database with the following properties:
   - "Name" column (title type)
   - "Status" column (status type with options: "Not started", "In progress", "Done")
   - "Priority" column (select type with options: "High", "Medium", "Low")
   - "Due Date" column (date type)
   - "Tags" column (multi-select type)
   - "Description" column (text type)

3. Share the database with your integration:
   - Open your database in Notion
   - Click "..." in the top right
   - Click "Add connections"
   - Select your integration

4. Copy the database ID from the URL:
   - Open your database in Notion
   - The URL will look like: `https://www.notion.so/workspace/DATABASE_ID?v=...`
   - Copy the DATABASE_ID part

5. Create a `.env` file:
```bash
cp .env.example .env
```
Then add your credentials:
```
NOTION_TOKEN=your_integration_token
NOTION_DATABASE_ID=your_database_id
LOCAL_TIMEZONE='Your/Timezone'
```

## Usage

```bash
# Add a new task (basic)
cargo run -- add "Your task title"

# Add a task with metadata
cargo run -- add "Important task" --priority high --due "2024-01-20" --tags "work,urgent" --description "This needs to be done ASAP"

# List all tasks
cargo run -- list

# List tasks with filters
cargo run -- list --status in-progress --priority high --tag work --sort-by-due

# Mark a task as in progress
cargo run -- progress <task-id>

# Mark a task as completed
cargo run -- check <task-id>

# Mark a task as not started
cargo run -- uncheck <task-id>

# Set task priority
cargo run -- set-priority <task-id> high

# Set due date
cargo run -- set-due <task-id> "2024-01-20"

# Add tags
cargo run -- add-tags <task-id> "work,urgent,important"

# Set description
cargo run -- set-description <task-id> "Detailed task description"

# Delete a task
cargo run -- delete <task-id>

# Show help
cargo run -- --help
```

## Development

### Running Tests

```bash
cargo test
```

### Building for Release

```bash
cargo build --release
```

The binary will be available at `target/release/notion-cli-rs`

## License

This project is licensed under the MIT License - see the LICENSE file for details.

