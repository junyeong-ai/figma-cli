//! Command handlers

use super::context::ClientContext;
use crate::cli::args::{
    AuthCommand, CacheCommand, ConfigCommand, ExtractArgs, ImagesArgs, InspectArgs, OutputFormat,
    QueryArgs,
};
use crate::cli::output::format_output;
use crate::client::{FigmaClient, TokenManager};
use crate::core::{Cache, Config, QueryEngine};
use crate::models::config::FilterCriteria;
use crate::service::Orchestrator;
use anyhow::{Context, Result};
use std::io::{self, Write};

/// Handle extract command
pub async fn handle_extract(args: ExtractArgs) -> Result<()> {
    let file_key = args
        .parse_file_key()
        .map_err(|e| anyhow::anyhow!("Failed to parse file key: {e}"))?;

    tracing::info!("Extracting from file: {}", file_key);

    let ctx = ClientContext::new(None)?;

    // Build filter criteria
    let mut filter = FilterCriteria::new();

    if let Some(pages) = args.pages {
        filter = filter.with_pages(pages);
    }

    if let Some(page_ids) = args.page_ids {
        filter = filter.with_page_ids(page_ids);
    }

    if let Some(pattern) = args.page_pattern {
        let regex = regex::Regex::new(&pattern).context("Invalid page pattern regex")?;
        filter = filter.with_page_pattern(regex);
    }

    if let Some(pattern) = args.frame_pattern {
        let regex = regex::Regex::new(&pattern).context("Invalid frame pattern regex")?;
        filter = filter.with_frame_pattern(regex);
    }

    if args.include_hidden {
        filter = filter.with_include_hidden(true);
    }

    // Create orchestrator and extract
    let orchestrator = Orchestrator::new(ctx.client);
    let result = orchestrator
        .extract(&file_key, filter, args.depth)
        .await
        .context("Failed to extract content from Figma file")?;

    let format = args.format;

    // Print stats header (skip for Summary format which is AI-optimized)
    if !matches!(format, OutputFormat::Summary) {
        println!();
        println!("File: {}", &result.metadata.file_name);
        println!("Version: {}", result.metadata.version);
        println!();
        println!("Statistics:");
        println!("  Pages:      {}", result.stats.total_pages);
        println!("  Frames:     {}", result.stats.total_frames);
        println!("  Text nodes: {}", result.stats.total_text_nodes);
        println!("  Characters: {}", result.stats.total_characters);
        println!("  Time:       {}ms", result.stats.extraction_time_ms);
        println!("  Memory:     {:.2}MB", result.stats.memory_size_mb);
        println!();
    }

    // Format and output
    format_output(&result, format, args.output.as_deref(), args.pretty)?;

    Ok(())
}

/// Handle auth command
pub async fn handle_auth(command: AuthCommand) -> Result<()> {
    match command {
        AuthCommand::Login { token } => {
            let token = if let Some(t) = token {
                t
            } else {
                // Prompt for token
                print!("Enter your Figma personal access token: ");
                io::stdout().flush()?;

                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
                input.trim().to_string()
            };

            // Validate token format
            if !token.starts_with("figd_") {
                anyhow::bail!("Invalid token format. Token should start with 'figd_'");
            }

            // Create client and test auth

            let client = FigmaClient::new(token.clone())?;
            let user_info = client
                .validate_auth()
                .await
                .context("Authentication failed")?;

            // Store token
            TokenManager::store(&token)?;

            println!();
            println!("✓ Authentication successful!");
            println!();
            println!("  Email:  {}", &user_info.email);
            println!("  Handle: {}", &user_info.handle);
            println!();
            println!("Token stored in ~/.config/figma-cli/config.json");
        }

        AuthCommand::Test => {
            let token = TokenManager::get()?
                .context("No authentication token found. Run 'figma-cli auth login' first")?;

            let client = FigmaClient::new(token)?;
            let user_info = client
                .validate_auth()
                .await
                .context("Authentication test failed")?;

            println!();
            println!("✓ Authentication valid");
            println!();
            println!("  Email:  {}", &user_info.email);
            println!("  Handle: {}", &user_info.handle);
        }

        AuthCommand::Logout => {
            TokenManager::delete()?;
            println!();
            println!("✓ Logged out successfully");
            println!("Token removed from config file.");
        }
    }

    Ok(())
}

/// Handle config command
pub async fn handle_config(command: ConfigCommand) -> Result<()> {
    match command {
        ConfigCommand::Init { local } => handle_config_init(!local),
        ConfigCommand::Show { json } => handle_config_show(json),
        ConfigCommand::Path => handle_config_path(),
        ConfigCommand::Edit { local } => handle_config_edit(!local),
        ConfigCommand::Set { key, value } => handle_config_set(&key, &value),
        ConfigCommand::Get { key } => handle_config_get(&key),
    }
}

/// Initialize a new config file
fn handle_config_init(global: bool) -> Result<()> {
    use crate::core::config::Config;

    let config_path = get_config_path(global)?;

    if config_path.exists() && !confirm_overwrite(&config_path)? {
        println!("Aborted.");
        return Ok(());
    }

    let default_config = Config::default();
    default_config.save(&config_path)?;

    println!();
    println!("✓ Config initialized");
    println!("  Path: {}", config_path.display());
    println!();
    println!("Edit the file to add your settings:");
    println!(
        "  figma-cli config edit{}",
        if global { " --global" } else { "" }
    );

    Ok(())
}

/// Show current configuration
fn handle_config_show(json: bool) -> Result<()> {
    use crate::core::config::Config;

    let config = Config::load()?;

    if json {
        let json_output = serde_json::to_string_pretty(&config)?;
        println!("{json_output}");
        return Ok(());
    }

    println!();
    println!("Configuration:");
    println!();

    print_token_status();

    println!();
    println!("  Extract depth: {}", config.extraction.depth);
    println!();
    println!("  Image format: {}", config.images.format);
    println!("  Image scale: {}", config.images.scale);
    // Base64 is handled at request level, not in config
    println!();

    print_config_files();

    println!("Run 'figma-cli config edit' to modify settings.");

    Ok(())
}

/// Show config file paths
fn handle_config_path() -> Result<()> {
    let system_config = Config::default_config_path();
    let project_config = std::path::PathBuf::from("figma-cli.toml");

    println!();
    println!("Config file paths:");
    println!();

    if let Some(path) = system_config {
        let status = if path.exists() { "exists" } else { "not found" };
        println!("  Global: {} ({})", path.display(), status);
    }

    let status = if project_config.exists() {
        "exists"
    } else {
        "not found"
    };
    println!("  Project: {} ({})", project_config.display(), status);
    println!();

    println!("Initialize config:");
    println!("  figma-cli config init --global  # Create global config");
    println!("  figma-cli config init           # Create project config");

    Ok(())
}

/// Edit config file in editor
fn handle_config_edit(global: bool) -> Result<()> {
    let config_path = get_config_path(global)?;

    if !config_path.exists() {
        println!();
        println!("✗ Config file not found");
        println!("  Path: {}", config_path.display());
        println!();
        println!("Initialize it first:");
        println!(
            "  figma-cli config init{}",
            if global { " --global" } else { "" }
        );
        anyhow::bail!("Config file not found");
    }

    let editor = get_default_editor();

    println!();
    println!("Opening editor...");
    println!("  Editor: {editor}");
    println!("  File:   {}", config_path.display());
    println!();

    let status = std::process::Command::new(&editor)
        .arg(&config_path)
        .status()
        .context(format!("Failed to open editor: {editor}"))?;

    if !status.success() {
        anyhow::bail!("Editor exited with error");
    }

    println!();
    println!("✓ Config saved");

    Ok(())
}

/// Set a configuration value
fn handle_config_set(key: &str, value: &str) -> Result<()> {
    // Special case: token uses TokenManager
    if key == "token" {
        TokenManager::store(value)?;
        println!();
        println!("✓ Token updated");
        return Ok(());
    }

    // For other keys, update config file
    let config_path = find_config_file()?;
    let mut toml_value: toml::Value = {
        let content = std::fs::read_to_string(&config_path)?;
        toml::from_str(&content)?
    };

    // Parse key path (e.g., "extraction.depth" -> ["extraction", "depth"])
    let path: Vec<&str> = key.split('.').collect();

    // Set the value
    set_toml_value(&mut toml_value, &path, value)?;

    // Save back to file
    let toml_string = toml::to_string_pretty(&toml_value)?;
    std::fs::write(&config_path, toml_string)?;

    println!();
    println!("✓ Configuration updated");
    println!("  {} = {}", key, value);
    println!("  File: {}", config_path.display());

    Ok(())
}

/// Get a configuration value
fn handle_config_get(key: &str) -> Result<()> {
    // Special case: token uses TokenManager
    if key == "token" {
        match TokenManager::get()? {
            Some(token) => println!("{}", mask_token(&token)),
            None => println!("Not set"),
        }
        return Ok(());
    }

    // For other keys, read from config file
    let config_path = find_config_file()?;
    let toml_value: toml::Value = {
        let content = std::fs::read_to_string(&config_path)?;
        toml::from_str(&content)?
    };

    // Parse key path
    let path: Vec<&str> = key.split('.').collect();

    // Get the value
    let value = get_toml_value(&toml_value, &path)?;
    println!("{value}");

    Ok(())
}

// Helper functions

/// Get config file path based on scope
fn get_config_path(global: bool) -> Result<std::path::PathBuf> {
    if global {
        let config_path = Config::default_config_path()
            .ok_or_else(|| anyhow::anyhow!("Cannot determine config directory"))?;

        if let Some(parent) = config_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        Ok(config_path)
    } else {
        Ok(std::path::PathBuf::from("figma-cli.toml"))
    }
}

/// Confirm overwrite of existing file
fn confirm_overwrite(config_path: &std::path::Path) -> Result<bool> {
    println!();
    println!("⚠️  Config file already exists");
    println!("  Path: {}", config_path.display());
    println!();
    print!("Overwrite? [y/N]: ");
    std::io::Write::flush(&mut std::io::stdout())?;

    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    Ok(input.trim().eq_ignore_ascii_case("y"))
}

/// Print token status with masking
fn print_token_status() {
    match TokenManager::get() {
        Ok(Some(token)) => println!("  Token:  {}", mask_token(&token)),
        _ => println!("  Token:  Not set"),
    }
}

/// Mask token for display
fn mask_token(token: &str) -> String {
    if token.len() > 12 {
        format!("{}...{}", &token[..8], &token[token.len() - 4..])
    } else {
        "***".to_string()
    }
}

/// Print available config files
fn print_config_files() {
    let system_config = Config::default_config_path().filter(|p| p.exists());

    let project_config = std::path::PathBuf::from("figma-cli.toml");
    let has_project = project_config.exists();

    if system_config.is_some() || has_project {
        println!("  Config files:");
        if let Some(path) = system_config {
            println!("    • {} (global)", path.display());
        }
        if has_project {
            println!("    • {} (project)", project_config.display());
        }
        println!();
    }
}

/// Get default editor from environment or system default
fn get_default_editor() -> String {
    std::env::var("EDITOR")
        .or_else(|_| std::env::var("VISUAL"))
        .unwrap_or_else(|_| {
            #[cfg(target_os = "macos")]
            {
                "nano".to_string()
            }
            #[cfg(not(target_os = "macos"))]
            {
                "vi".to_string()
            }
        })
}

/// Find config file (project first, then global)
fn find_config_file() -> Result<std::path::PathBuf> {
    let project_config = std::path::PathBuf::from("figma-cli.toml");
    if project_config.exists() {
        return Ok(project_config);
    }

    let global_config = dirs::config_dir()
        .ok_or_else(|| anyhow::anyhow!("Cannot determine config directory"))?
        .join("figma-cli")
        .join("config.toml");

    if global_config.exists() {
        return Ok(global_config);
    }

    anyhow::bail!("No config file found. Run 'figma-cli config init' to create one.")
}

/// Get value from TOML using dot-separated path
fn get_toml_value(value: &toml::Value, path: &[&str]) -> Result<String> {
    if path.is_empty() {
        anyhow::bail!("Empty path");
    }

    let mut current = value;

    for &segment in path {
        current = current
            .get(segment)
            .ok_or_else(|| anyhow::anyhow!("Key not found: {}", path.join(".")))?;
    }

    // Format the value based on its type
    let result = match current {
        toml::Value::String(s) => s.clone(),
        toml::Value::Integer(i) => i.to_string(),
        toml::Value::Float(f) => f.to_string(),
        toml::Value::Boolean(b) => b.to_string(),
        toml::Value::Datetime(d) => d.to_string(),
        toml::Value::Array(_) | toml::Value::Table(_) => toml::to_string_pretty(current)?,
    };

    Ok(result)
}

/// Set value in TOML using dot-separated path
fn set_toml_value(value: &mut toml::Value, path: &[&str], value_str: &str) -> Result<()> {
    if path.is_empty() {
        anyhow::bail!("Empty path");
    }

    if path.len() == 1 {
        // Top-level key
        let table = value
            .as_table_mut()
            .ok_or_else(|| anyhow::anyhow!("Root is not a table"))?;

        let existing = table
            .get(path[0])
            .ok_or_else(|| anyhow::anyhow!("Key not found: {}", path[0]))?;

        let new_value = parse_toml_value(existing, value_str)?;
        table.insert(path[0].to_string(), new_value);
    } else {
        // Nested key
        let mut current = value;

        for &segment in &path[..path.len() - 1] {
            current = current
                .get_mut(segment)
                .ok_or_else(|| anyhow::anyhow!("Key not found: {}", path.join(".")))?;
        }

        let table = current
            .as_table_mut()
            .ok_or_else(|| anyhow::anyhow!("Parent is not a table"))?;

        let last_key = path[path.len() - 1];
        let existing = table
            .get(last_key)
            .ok_or_else(|| anyhow::anyhow!("Key not found: {}", path.join(".")))?;

        let new_value = parse_toml_value(existing, value_str)?;
        table.insert(last_key.to_string(), new_value);
    }

    Ok(())
}

/// Parse string value based on existing type
fn parse_toml_value(existing: &toml::Value, value_str: &str) -> Result<toml::Value> {
    let new_value = match existing {
        toml::Value::Integer(_) => {
            let v = value_str
                .parse::<i64>()
                .context("Failed to parse as integer")?;
            toml::Value::Integer(v)
        }
        toml::Value::Float(_) => {
            let v = value_str
                .parse::<f64>()
                .context("Failed to parse as float")?;
            toml::Value::Float(v)
        }
        toml::Value::Boolean(_) => {
            let v = value_str
                .parse::<bool>()
                .context("Failed to parse as boolean")?;
            toml::Value::Boolean(v)
        }
        toml::Value::String(_) => toml::Value::String(value_str.to_string()),
        _ => anyhow::bail!("Unsupported type for modification"),
    };

    Ok(new_value)
}

/// Handle inspect command
pub async fn handle_inspect(args: InspectArgs) -> Result<()> {
    // Parse file key and node IDs from URL
    let (file_key, url_node_ids) = crate::utils::parse_file_and_nodes_from_url(&args.file)
        .map_err(|e| anyhow::anyhow!("Failed to parse file/node IDs: {e}"))?;

    // Get node IDs from args or URL
    let mut node_ids = args.nodes.unwrap_or_default();

    // Convert hyphenated format to colon format if needed
    node_ids = node_ids
        .iter()
        .map(|id| {
            if let Some(converted) = crate::utils::parse_node_id_from_url(id) {
                converted
            } else {
                id.clone()
            }
        })
        .collect();

    // Add node IDs from URL
    node_ids.extend(url_node_ids);

    if node_ids.is_empty() {
        anyhow::bail!("No node IDs specified. Use --nodes or provide a URL with node-id parameter");
    }

    tracing::info!(
        "Inspecting {} nodes from file: {}",
        node_ids.len(),
        file_key
    );

    let ctx = ClientContext::new(args.config.as_deref())?;

    // Fetch nodes
    let nodes_response = ctx
        .client
        .get_nodes(&file_key, &node_ids, Some(args.depth))
        .await
        .context("Failed to fetch nodes from Figma")?;

    // Prepare output
    let output_json = serde_json::json!({
        "file": {
            "key": file_key,
            "name": nodes_response.name,
        },
        "nodes": nodes_response.nodes,
        "depth": args.depth,
    });

    // Output results
    let output_str = if args.pretty {
        serde_json::to_string_pretty(&output_json)?
    } else {
        serde_json::to_string(&output_json)?
    };

    if let Some(output_path) = args.output {
        std::fs::write(&output_path, output_str)?;
        println!();
        println!("✓ Nodes inspected");
        println!("  File: {output_path}");
        println!("  Total nodes: {}", nodes_response.nodes.len());
    } else {
        println!("{output_str}");
    }

    Ok(())
}

/// Handle images command
pub async fn handle_images(args: ImagesArgs) -> Result<()> {
    use crate::images::ImageProcessor;

    // Parse file key and node IDs from URL
    let (file_key, url_node_ids) = crate::utils::parse_file_and_nodes_from_url(&args.file)
        .map_err(|e| anyhow::anyhow!("Failed to parse file/node IDs: {e}"))?;

    tracing::info!("Processing images from file: {}", file_key);

    let mut ctx = ClientContext::new(args.config.as_deref())?;

    // Override config with CLI args
    ctx.config.images.format = args.format.clone();
    ctx.config.images.scale = args.scale as f32;

    // Determine frame IDs
    let frame_ids = if let Some(frames) = args.frames {
        // Convert hyphenated format to colon format if needed
        frames
            .iter()
            .map(|id| {
                if let Some(converted) = crate::utils::parse_node_id_from_url(id) {
                    converted
                } else {
                    id.clone()
                }
            })
            .collect()
    } else if !url_node_ids.is_empty() {
        // Use node IDs from URL
        url_node_ids
    } else {
        anyhow::bail!("No frames specified. Use --frames or provide a URL with node-id parameter");
    };

    if frame_ids.is_empty() {
        anyhow::bail!("No frames to process");
    }

    // Process images
    let processor = ImageProcessor::new(ctx.config.images.clone())?;
    let results = processor
        .process_frames(&ctx.token, &file_key, &frame_ids, args.base64)
        .await?;

    // Convert to AI format
    let ai_formatted: Vec<_> = results
        .iter()
        .map(super::super::images::ImageResult::to_ai_format)
        .collect();

    let output_json = serde_json::json!({
        "images": ai_formatted,
        "total": results.len(),
        "format": if args.base64 {
            "base64"
        } else {
            "url"
        }
    });

    // Output results
    let output_str = if args.pretty {
        serde_json::to_string_pretty(&output_json)?
    } else {
        serde_json::to_string(&output_json)?
    };

    if let Some(output_path) = args.output {
        std::fs::write(&output_path, output_str)?;
        println!();
        println!("✓ Images processed");
        println!("  File: {output_path}");
        println!("  Total images: {}", results.len());
        println!("  Format: {}", if args.base64 { "base64" } else { "url" });
    } else {
        println!("{output_str}");
    }

    Ok(())
}

pub async fn handle_query(args: QueryArgs) -> Result<()> {
    let (file_key, url_node_ids) = crate::utils::parse_file_and_nodes_from_url(&args.file)
        .map_err(|e| anyhow::anyhow!("Failed to parse file/node IDs: {e}"))?;

    let ctx = ClientContext::new(args.config.as_deref())?;

    let data = if let Some(node_ids) = args.nodes.as_ref().or(Some(&url_node_ids)) {
        if !node_ids.is_empty() {
            let response = ctx
                .client
                .get_nodes(&file_key, node_ids, args.depth)
                .await
                .context("Failed to fetch nodes")?;
            serde_json::to_value(&response)?
        } else {
            let file = ctx
                .client
                .get_file(&file_key, args.depth)
                .await
                .context("Failed to fetch file")?;
            serde_json::to_value(&file)?
        }
    } else {
        let file = ctx
            .client
            .get_file(&file_key, args.depth)
            .await
            .context("Failed to fetch file")?;
        serde_json::to_value(&file)?
    };

    let result = QueryEngine::apply(&args.query, &data)?;

    let output_str = if args.pretty {
        serde_json::to_string_pretty(&result)?
    } else {
        serde_json::to_string(&result)?
    };

    if let Some(output_path) = args.output {
        std::fs::write(&output_path, output_str)?;
        println!();
        println!("✓ Query executed");
        println!("  File: {output_path}");
    } else {
        println!("{output_str}");
    }

    Ok(())
}

pub async fn handle_cache(command: CacheCommand) -> Result<()> {
    let config = Config::load()?;
    let cache_dir = config.cache_path();
    let cache = Cache::new(cache_dir.clone(), config.cache.ttl)?;

    match command {
        CacheCommand::Stats => {
            let stats = cache.stats();
            println!();
            println!("Cache Statistics:");
            println!("  Entries: {}", stats.total_entries);
            println!("  Size: {:.2}MB", stats.total_size as f64 / 1024.0 / 1024.0);
            println!("  Expired: {}", stats.expired_entries);
            println!("  TTL: {}h", stats.ttl_hours);
            println!("  Path: {}", cache_dir.display());
            println!();
        }

        CacheCommand::List { json } => {
            let entries = cache.list();

            if json {
                println!("{}", serde_json::to_string_pretty(&entries)?);
            } else {
                if entries.is_empty() {
                    println!();
                    println!("No cache entries");
                    println!();
                    return Ok(());
                }

                println!();
                println!("Cache Entries:");
                println!();
                for (i, entry) in entries.iter().enumerate() {
                    println!("  {}. File: {}", i + 1, entry.file_key);
                    println!("     Version: {}", entry.version);
                    println!("     Depth: {:?}", entry.depth);
                    println!("     Size: {:.2}KB", entry.size as f64 / 1024.0);
                    println!("     Created: {}", entry.created_at);
                    println!("     Accessed: {}", entry.accessed_at);
                    println!();
                }
            }
        }

        CacheCommand::Clear { yes } => {
            if !yes {
                print!("Clear all cache entries? [y/N]: ");
                std::io::Write::flush(&mut std::io::stdout())?;

                let mut input = String::new();
                std::io::stdin().read_line(&mut input)?;

                if !input.trim().eq_ignore_ascii_case("y") {
                    println!("Aborted.");
                    return Ok(());
                }
            }

            cache.clear()?;
            println!();
            println!("✓ Cache cleared");
            println!();
        }
    }

    Ok(())
}
