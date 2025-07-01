# obsidian-project-cli (opj)

A powerful CLI tool for managing Obsidian project workflows efficiently. Create, organize, and track your projects with ease.

## ✨ Features

- 🚀 **Quick Project Creation** - Create structured projects from templates in seconds
- 📊 **Smart Project Tracking** - Monitor status, progress, and timeline of all projects
- 🎯 **Type-Specific Templates** - Optimized templates for Web Apps, Tools, Content, and APIs
- 📈 **Progress Visualization** - Beautiful progress tracking and statistics
- 🔄 **Status Management** - Intuitive project lifecycle management
- 🎨 **Beautiful CLI** - Colorful, informative output with emojis and progress bars

## 🚀 Quick Start

### Installation

```bash
# Install from source
git clone https://github.com/yourusername/obsidian-project-cli
cd obsidian-project-cli
cargo install --path .
```

### Basic Usage

```bash
# Create a new project interactively
opj new --interactive

# Create a specific type of project
opj new "My Web App" --type web-app --priority high

# List all projects
opj list

# Update project status
opj status "My Web App" --set development --note "Started implementation"

# View project details
opj info "My Web App"

# Show project statistics
opj stats
```

## 📋 Commands

### Project Management

```bash
opj new [NAME]              # Create a new project
  --type TYPE               # Project type: web-app, tool, content, api
  --priority LEVEL          # Priority: low, medium, high
  --description TEXT        # Project description
  --interactive            # Interactive creation mode

opj list                    # List all projects
  --status STATUS          # Filter by status
  --type TYPE              # Filter by type
  --priority LEVEL         # Filter by priority
  --sort FIELD             # Sort by: name, date, updated, status, priority
  --format FORMAT          # Output format: table, json, yaml

opj status PROJECT         # Manage project status
  --set STATUS            # Set new status: idea, planning, development, testing, completed
  --note TEXT             # Add note to status change
  --log                   # Show status history

opj info PROJECT           # Show detailed project information
```

### Utility Commands

```bash
opj template list           # List available templates
opj stats                   # Show project statistics
opj config                  # Manage configuration (coming soon)
opj migrate SOURCE          # Migrate existing projects (coming soon)
```

## 🏗️ Project Structure

Each project follows a consistent structure:

```
2025-06-18_Web-App_Project-Name/
├── 00_PROJECT-OVERVIEW.md      # Project overview and metadata
├── 01_Requirements.md          # Requirements and specifications
├── 02_Architecture.md          # Architecture and design
├── 03_Development-Log.md       # Development progress log
├── 04_Testing.md              # Testing plans and results
├── 05_Deployment.md           # Deployment and release info
├── assets/                    # Project assets
│   ├── images/               # Screenshots, diagrams
│   ├── diagrams/             # Architecture diagrams
│   └── mockups/              # UI/UX mockups
├── .project-meta.yaml         # Project metadata (auto-generated)
└── [project-specific]/        # Additional directories
```

## 🎯 Project Types

### 🌐 Web Application
Perfect for React, Vue, Next.js, and other web development projects.
- Frontend/backend architecture planning
- UI/UX design workflow
- Database design templates
- Deployment strategies

### 🛠️ Tool/CLI
Ideal for command-line tools, libraries, and development utilities.
- CLI design and user experience
- API documentation
- Package distribution strategy
- Cross-platform compatibility

### 📝 Content/Article
Optimized for blog posts, tutorials, and documentation.
- Content strategy and planning
- Writing workflow management
- Publishing and promotion
- SEO and analytics tracking

### 🔌 API/Backend
Designed for REST APIs, GraphQL services, and backend systems.
- API design and documentation
- Database schema design
- Security and authentication
- Performance and scalability

## ⚙️ Configuration

The CLI automatically creates a configuration file at `~/.config/opj/config.yaml`:

```yaml
obsidian_path: "/Users/username/Documents/Obsidian-Vault"
projects_dir: "Projects"
templates_dir: "Projects/_TEMPLATES"
default_type: "web-app"
default_priority: "medium"
auto_git_init: true
editor: "code"
date_format: "%Y-%m-%d"
```

## 🎨 Examples

### Creating a Web Application Project

```bash
$ opj new "Task Manager" --type web-app --priority high --description "A modern task management app"

✨ Create a new project

🚀 Creating project "Task Manager"...
📁 Creating directory: Projects/2025-06-18_Web-App_Task-Manager/
📄 Copying template: Web-App-Project -> Task Manager
✏️  Updating project metadata...
📦 Initializing Git repository...

✅ Project "Task Manager" created successfully!

📍 Location: Projects/2025-06-18_Web-App_Task-Manager/
📋 Next steps:
  1. Review project overview: opj info "Task Manager"
  2. Edit requirements: edit 01_Requirements.md
  3. Start development: opj status "Task Manager" --set development
```

### Viewing Project Status

```bash
$ opj status "Task Manager" --set development --note "Started implementing user authentication"

🔄 Updating project status...

Project: Task Manager
Status: Planning → Development
Note: Started implementing user authentication

✅ Status updated successfully!

📈 Progress:
  ✅ Idea (2025-06-18)
  ✅ Planning (2025-06-19)
  🚧 Development (2025-06-20) ← Current
  ⏳ Testing
  ⏳ Completed

💡 Next: opj status "Task Manager" --set testing when ready
```

### Project Statistics

```bash
$ opj stats

📊 Project Statistics

📈 Overall
  Total Projects: 12
  Active Projects: 5
  Completed Projects: 3
  Completion Rate: 25.0%

📋 Status Breakdown
  💡 Idea: 2
  📋 Planning: 1
  🚧 Development: 4
  🧪 Testing: 2
  ✅ Completed: 3

🏷️ Type Breakdown
  🌐 Web-App: 6
  🛠️ Tool: 3
  📝 Content: 2
  🔌 API: 1

⏰ Activity Stats
  Most Recent Activity: Task Manager (1 days ago)
  Oldest Project: Portfolio Website (45 days old)
  Average Project Age: 18.2 days
```

## 🔧 Development

### Building from Source

```bash
# Clone the repository
git clone https://github.com/yourusername/obsidian-project-cli
cd obsidian-project-cli

# Build the project
cargo build --release

# Run tests
cargo test

# Install locally
cargo install --path .
```

### Project Structure

```
src/
├── main.rs                 # Application entry point
├── cli/                    # CLI argument parsing and commands
│   ├── mod.rs
│   └── commands/           # Individual command implementations
├── core/                   # Core business logic
│   ├── project.rs          # Project data structures
│   ├── template.rs         # Template management
│   └── status.rs           # Status management
├── config/                 # Configuration management
├── fs/                     # File system operations
│   ├── operations.rs       # File/directory operations
│   └── scanner.rs          # Project discovery
└── utils/                  # Utility modules
    ├── template.rs         # Template processing
    ├── date.rs             # Date formatting
    └── output.rs           # Output formatting
```

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_project_creation

# Run tests in release mode
cargo test --release
```

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

### Development Guidelines

1. **Code Style**: Follow Rust conventions and run `cargo fmt`
2. **Testing**: Add tests for new features and ensure all tests pass
3. **Documentation**: Update README and code comments for new features
4. **Commits**: Use conventional commit messages

### Feature Requests

Priority features for future releases:

- [ ] **Template Management** - Custom template creation and editing
- [ ] **Project Migration** - Import existing projects from other systems
- [ ] **Configuration Management** - Full CLI configuration interface
- [ ] **Web Interface** - Optional web UI for project management
- [ ] **Team Collaboration** - Shared project workflows
- [ ] **Plugin System** - Extensible architecture for custom functionality

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [clap](https://github.com/clap-rs/clap) - Command line argument parsing
- [serde](https://github.com/serde-rs/serde) - Serialization framework
- [tokio](https://github.com/tokio-rs/tokio) - Async runtime
- [colored](https://github.com/mackwic/colored) - Terminal colors
- [dialoguer](https://github.com/mitsuhiko/dialoguer) - Interactive CLI prompts

## 🔗 Related Projects

- [Obsidian](https://obsidian.md/) - The knowledge management app this tool is designed for
- [obsidian-clipper](https://github.com/obsidianmd/obsidian-clipper) - Web clipper for Obsidian
- [templater-obsidian](https://github.com/SilentVoid13/Templater) - Template plugin for Obsidian

---

**Built with ❤️ for the Obsidian community**
