//! CLI argument definitions

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "figma-cli")]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Enable verbose logging
    #[arg(short, long, global = true)]
    pub verbose: bool,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Extract content from a Figma file
    Extract(ExtractArgs),

    /// Inspect specific nodes from a Figma file
    Inspect(InspectArgs),

    /// Get images with optional base64 encoding
    Images(ImagesArgs),

    /// Query Figma data using JMESPath
    Query(QueryArgs),

    /// Manage cache
    Cache(CacheArgs),

    /// Manage authentication
    Auth(AuthArgs),

    /// Manage configuration
    Config(ConfigArgs),
}

#[derive(Parser, Debug)]
pub struct ExtractArgs {
    /// Figma file URL or key
    #[arg(value_name = "FILE")]
    pub file: String,

    /// Output file path (default: stdout)
    #[arg(short, long)]
    pub output: Option<String>,

    /// Output format
    #[arg(short = 'f', long, default_value = "json")]
    pub format: OutputFormat,

    /// Pretty print JSON output
    #[arg(long)]
    pub pretty: bool,

    /// Filter by page names (comma-separated)
    #[arg(short, long, value_delimiter = ',')]
    pub pages: Option<Vec<String>>,

    /// Filter by page name pattern (regex)
    #[arg(long)]
    pub page_pattern: Option<String>,

    /// Filter by frame name pattern (regex)
    #[arg(long)]
    pub frame_pattern: Option<String>,

    /// Include hidden nodes
    #[arg(long)]
    pub include_hidden: bool,

    /// Include design element metadata
    #[arg(long)]
    pub with_metadata: bool,

    /// Include images
    #[arg(long)]
    pub with_images: bool,

    /// Directory for downloaded images
    #[arg(long, default_value = "./images")]
    pub image_dir: String,

    /// Image format
    #[arg(long, default_value = "png")]
    pub image_format: String,

    /// Image scale (1.0, 2.0, etc.)
    #[arg(long, default_value = "2.0")]
    pub image_scale: f64,

    /// Maximum concurrent image downloads
    #[arg(long, default_value = "50")]
    pub max_concurrent: usize,

    /// Request timeout in milliseconds
    #[arg(long, default_value = "30000")]
    pub timeout: u64,

    /// Depth of tree traversal (1=pages only, 2=pages+top-level objects, etc.)
    #[arg(long)]
    pub depth: Option<u32>,
}

#[derive(Parser, Debug)]
pub struct InspectArgs {
    /// Figma file URL or key (supports node-id in URL)
    #[arg(value_name = "FILE")]
    pub file: String,

    /// Node IDs to inspect (comma-separated, in format "123:456" or "123-456")
    #[arg(long, value_delimiter = ',')]
    pub nodes: Option<Vec<String>>,

    /// Depth of tree traversal for each node
    #[arg(long, default_value = "1")]
    pub depth: u32,

    /// Output file path (default: stdout)
    #[arg(short, long)]
    pub output: Option<String>,

    /// Pretty print JSON output
    #[arg(long)]
    pub pretty: bool,

    /// Config file path
    #[arg(short = 'c', long)]
    pub config: Option<String>,
}

#[derive(Parser, Debug)]
pub struct ImagesArgs {
    /// Figma file URL or key
    #[arg(value_name = "FILE")]
    pub file: String,

    /// Frame IDs to export (comma-separated)
    #[arg(long, value_delimiter = ',')]
    pub frames: Option<Vec<String>>,

    /// Enable base64 encoding
    #[arg(long)]
    pub base64: bool,

    /// Output file path (default: stdout)
    #[arg(short, long)]
    pub output: Option<String>,

    /// Pretty print JSON output
    #[arg(long)]
    pub pretty: bool,

    /// Image format (png, jpg, svg, pdf)
    #[arg(long, default_value = "png")]
    pub format: String,

    /// Image scale
    #[arg(long, default_value = "2.0")]
    pub scale: f64,

    /// Config file path
    #[arg(short = 'c', long)]
    pub config: Option<String>,
}

#[derive(Parser, Debug)]
pub struct AuthArgs {
    #[command(subcommand)]
    pub command: AuthCommand,
}

#[derive(Subcommand, Debug)]
pub enum AuthCommand {
    /// Login with a personal access token
    Login {
        /// Figma personal access token
        #[arg(value_name = "TOKEN")]
        token: Option<String>,
    },

    /// Test authentication
    Test,

    /// Logout (remove stored token)
    Logout,
}

#[derive(Parser, Debug)]
pub struct QueryArgs {
    /// Figma file URL or key (supports node-id in URL)
    #[arg(value_name = "FILE")]
    pub file: String,

    /// JMESPath query expression
    #[arg(value_name = "QUERY")]
    pub query: String,

    /// Node IDs to query (comma-separated, applies query to specific nodes)
    #[arg(long, value_delimiter = ',')]
    pub nodes: Option<Vec<String>>,

    /// Depth of tree traversal
    #[arg(long)]
    pub depth: Option<u32>,

    /// Output file path (default: stdout)
    #[arg(short, long)]
    pub output: Option<String>,

    /// Pretty print JSON output
    #[arg(long)]
    pub pretty: bool,

    /// Config file path
    #[arg(short = 'c', long)]
    pub config: Option<String>,
}

#[derive(Parser, Debug)]
pub struct CacheArgs {
    #[command(subcommand)]
    pub command: CacheCommand,
}

#[derive(Subcommand, Debug)]
pub enum CacheCommand {
    /// Show cache statistics
    Stats,

    /// List cache entries
    List {
        /// Output in JSON format
        #[arg(long)]
        json: bool,
    },

    /// Clear all cache entries
    Clear {
        /// Skip confirmation prompt
        #[arg(short, long)]
        yes: bool,
    },
}

#[derive(Parser, Debug)]
pub struct ConfigArgs {
    #[command(subcommand)]
    pub command: ConfigCommand,
}

#[derive(Subcommand, Debug)]
pub enum ConfigCommand {
    /// Initialize configuration file
    Init {
        /// Initialize project config (./figma-cli.toml) instead of global config
        #[arg(long)]
        local: bool,
    },

    /// Show current configuration
    Show {
        /// Output in JSON format
        #[arg(long)]
        json: bool,
    },

    /// Show configuration file path
    Path,

    /// Edit configuration file with default editor
    Edit {
        /// Edit project config (./figma-cli.toml) instead of global config
        #[arg(long)]
        local: bool,
    },

    /// Set a configuration value
    Set {
        /// Configuration key
        key: String,
        /// Configuration value
        value: String,
    },

    /// Get a configuration value
    Get {
        /// Configuration key
        key: String,
    },
}

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
pub enum OutputFormat {
    Json,
    Text,
    Markdown,
}

impl ExtractArgs {
    pub fn parse_file_key(&self) -> Result<String, String> {
        crate::utils::parse_file_key_from_url(&self.file).map_err(|e| e.to_string())
    }
}
