# Notion CLI

A command-line interface for managing tasks in Notion, written in Rust.

## Features

- Add new tasks to your Notion database
- List all tasks with their status
- Mark tasks as "In Progress"
- Mark tasks as completed/uncompleted
- Delete tasks
- Colored output in terminal
- Status indicators (✓ for done, ⏳ for in progress)

## Installation

1. Clone the repository:
```bash
git clone [your-repo-url]
cd notion-cli
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

2. Create a Notion database with:
   - A "Name" column (title type)
   - A "Status" column (status type with options: "Not started", "In progress", "Done")

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
```

## Usage

```bash
# Add a new task
cargo run -- add "Your task title"

# List all tasks
cargo run -- list

# Mark a task as in progress
cargo run -- progress <task-id>

# Mark a task as completed
cargo run -- check <task-id>

# Mark a task as not started
cargo run -- uncheck <task-id>

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

The binary will be available at `target/release/notion-cli`

## License

This project is licensed under the MIT License - see the LICENSE file for details.

