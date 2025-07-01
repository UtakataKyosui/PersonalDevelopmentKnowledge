# Basic Usage Examples

## Creating Your First Project

```bash
# Interactive mode (recommended for beginners)
opj new --interactive
```

This will guide you through:
1. Project name input
2. Type selection (Web-App, Tool, Content, API)
3. Priority setting (Low, Medium, High)
4. Optional description

## Quick Project Creation

```bash
# Create a web application
opj new "E-commerce Store" --type web-app --priority high --description "Modern e-commerce platform with React and Node.js"

# Create a CLI tool
opj new "File Organizer" --type tool --priority medium --description "Automated file organization utility"

# Create content project
opj new "React Tutorial Series" --type content --priority high --description "Comprehensive React tutorial for beginners"
```

## Managing Project Status

```bash
# View current status
opj status "E-commerce Store"

# Update status with note
opj status "E-commerce Store" --set development --note "Started implementing user authentication"

# View status history
opj status "E-commerce Store" --log
```

## Viewing and Filtering Projects

```bash
# List all projects
opj list

# Filter by status
opj list --status development

# Filter by type and priority
opj list --type web-app --priority high

# Sort by different fields
opj list --sort updated
opj list --sort priority

# Different output formats
opj list --format json
opj list --format yaml
```

## Project Information

```bash
# Detailed project information
opj info "E-commerce Store"

# Quick statistics
opj stats
```

## Working with Templates

```bash
# List available templates
opj template list

# (Coming soon) Add custom template
opj template add "My Custom Template" ./path/to/template

# (Coming soon) Edit existing template
opj template edit "Web-App-Project"
```
