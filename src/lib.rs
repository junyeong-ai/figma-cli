pub mod cli;
pub mod client;
pub mod core;
pub mod extractor;
pub mod images;
pub mod models;
pub mod service;
pub mod utils;

pub use core::{
    config::{Config, SecureString},
    errors::{Error, Result},
};

pub use client::FigmaClient;
pub use extractor::TextExtractor;
pub use models::{
    document::FigmaFile,
    extraction::{ExtractedText, ExtractionResult, FileMetadata},
};
pub use service::Orchestrator;
pub use utils::{parse_file_key_from_url, validate_file_key};
