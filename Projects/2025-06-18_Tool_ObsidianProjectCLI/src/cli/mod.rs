use clap::{Parser, Subcommand, ValueEnum};

pub mod commands;

#[derive(Parser)]
#[command(name = "opj")]
#[command(about = "A CLI tool for managing Obsidian project workflows")]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Verbose output
    #[arg(short, long, global = true)]
    pub verbose: bool,

    /// Suppress output except errors
    #[arg(short, long, global = true)]
    pub quiet: bool,

    /// Configuration file path
    #[arg(short, long, global = true)]
    pub config: Option<String>,

    /// Obsidian vault path
    #[arg(long, global = true)]
    pub obsidian_path: Option<String>,

    /// Dry run - show what would be done without executing
    #[arg(long, global = true)]
    pub dry_run: bool,

    /// Disable colored output
    #[arg(long, global = true)]
    pub no_color: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Create a new project
    New {
        /// Project name
        name: Option<String>,

        /// Project type
        #[arg(short, long)]
        project_type: Option<ProjectType>,

        /// Project priority
        #[arg(short, long)]
        priority: Option<Priority>,

        /// Project description
        #[arg(short, long)]
        description: Option<String>,

        /// Template to use
        #[arg(long)]
        template: Option<String>,

        /// Skip Git initialization
        #[arg(long)]
        no_git: bool,

        /// Interactive mode
        #[arg(short, long)]
        interactive: bool,
    },

    /// List existing projects
    List {
        /// Filter by status
        #[arg(short, long)]
        status: Option<ProjectStatus>,

        /// Filter by project type
        #[arg(short = 't', long)]
        project_type: Option<ProjectType>,

        /// Filter by priority
        #[arg(short, long)]
        priority: Option<Priority>,

        /// Sort by field
        #[arg(long, default_value = "updated")]
        sort: SortField,

        /// Output format
        #[arg(short, long, default_value = "table")]
        format: OutputFormat,
    },

    /// Manage project status
    Status {
        /// Project name or ID
        project: String,

        /// Set new status
        #[arg(long)]
        set_status: Option<ProjectStatus>,

        /// Add note to status change
        #[arg(short, long)]
        note: Option<String>,

        /// Show status history
        #[arg(short, long)]
        log: bool,
    },

    /// Show project information
    Info {
        /// Project name or ID
        project: String,
    },

    /// Manage templates
    Template {
        #[command(subcommand)]
        subcommand: TemplateCommand,
    },

    /// Migrate existing projects
    Migrate {
        /// Source directory path
        source: String,
    },

    /// Manage configuration
    Config {
        /// Configuration key
        key: Option<String>,

        /// Configuration value
        value: Option<String>,
    },

    /// Show project statistics
    Stats,
}

#[derive(Subcommand)]
pub enum TemplateCommand {
    /// List available templates
    List,

    /// Add a new template
    Add {
        /// Template name
        name: String,

        /// Template source path
        path: String,
    },

    /// Edit a template
    Edit {
        /// Template name
        name: String,
    },

    /// Remove a template
    Remove {
        /// Template name
        name: String,
    },
}

#[derive(Clone, Debug, ValueEnum)]
pub enum ProjectType {
    WebApp,
    Tool,
    Content,
    Api,
}

#[derive(Clone, Debug, ValueEnum)]
pub enum ProjectStatus {
    Idea,
    Planning,
    Development,
    Testing,
    Completed,
    Archived,
    Cancelled,
}

#[derive(Clone, Debug, ValueEnum)]
pub enum Priority {
    Low,
    Medium,
    High,
}

#[derive(Clone, Debug, ValueEnum)]
pub enum SortField {
    Name,
    Date,
    Updated,
    Status,
    Priority,
    Type,
}

#[derive(Clone, Debug, ValueEnum)]
pub enum OutputFormat {
    Table,
    Json,
    Yaml,
}
