#!/bin/bash

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Print with color
info() {
    echo -e "${BLUE}INFO:${NC} $1"
}

success() {
    echo -e "${GREEN}SUCCESS:${NC} $1"
}

error() {
    echo -e "${RED}ERROR:${NC} $1"
    exit 1
}

# Check if required tools are installed
check_requirements() {
    info "Checking requirements..."
    
    if ! command -v cargo &> /dev/null; then
        error "Rust and Cargo are required but not installed. Please visit https://www.rust-lang.org/tools/install"
    fi

    if ! command -v git &> /dev/null; then
        error "Git is required but not installed. Please install git first."
    fi
}

# Clone the repository
clone_repo() {
    info "Cloning notion-cli-rs repository..."
    
    TEMP_DIR=$(mktemp -d)
    git clone https://github.com/CharlonTank/notion-cli-rs.git "$TEMP_DIR" || error "Failed to clone repository"
    cd "$TEMP_DIR"
}

# Build the project
build_project() {
    info "Building project..."
    
    cargo build --release || error "Failed to build project"
}

# Install the binary
install_binary() {
    info "Installing notion-cli-rs..."
    
    # Determine installation directory
    if [ -w "/usr/local/bin" ]; then
        INSTALL_DIR="/usr/local/bin"
    else
        INSTALL_DIR="$HOME/.local/bin"
        mkdir -p "$INSTALL_DIR"
    fi

    # Copy binary
    cp "target/release/notion-cli-rs" "$INSTALL_DIR/" || error "Failed to copy binary"

    # Make it executable
    chmod +x "$INSTALL_DIR/notion-cli-rs" || error "Failed to make binary executable"

    # Add to PATH if necessary
    if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
        case "$SHELL" in
            */fish)
                fish_add_path "$INSTALL_DIR"
                ;;
            */zsh)
                echo "export PATH=\"$INSTALL_DIR:\$PATH\"" >> "$HOME/.zshrc"
                ;;
            */bash)
                echo "export PATH=\"$INSTALL_DIR:\$PATH\"" >> "$HOME/.bashrc"
                ;;
            *)
                info "Please add $INSTALL_DIR to your PATH manually"
                ;;
        esac
    fi
}

# Cleanup temporary files
cleanup() {
    if [ -n "$TEMP_DIR" ]; then
        info "Cleaning up..."
        rm -rf "$TEMP_DIR"
    fi
}

# Set up environment file
setup_env() {
    info "Setting up environment..."
    
    if [ ! -f "$HOME/.notion-cli-rs/.env" ]; then
        mkdir -p "$HOME/.notion-cli-rs"
        cat > "$HOME/.notion-cli-rs/.env" << EOL
# Notion CLI Configuration
# Visit https://www.notion.so/my-integrations to get your token
NOTION_TOKEN=secret_your_integration_token_here
NOTION_DATABASE_ID=your_database_id_here
LOCAL_TIMEZONE=America/New_York
EOL
        info "Created .env file at $HOME/.notion-cli-rs/.env"
        info "Please edit this file with your Notion credentials"
    fi
}

# Main installation process
main() {
    info "Starting notion-cli-rs installation..."
    
    check_requirements
    clone_repo
    build_project
    install_binary
    setup_env
    cleanup

    success "notion-cli-rs has been installed successfully!"
    echo
    info "Next steps:"
    echo "1. Edit $HOME/.notion-cli-rs/.env with your Notion credentials"
    echo "2. Run 'notion-cli-rs --help' to get started"
    echo
    info "If the command is not found, you may need to restart your shell or run:"
    echo "export PATH=\"$INSTALL_DIR:\$PATH\""
}

# Run main function
main 