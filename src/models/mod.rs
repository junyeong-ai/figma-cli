//! Data models

pub mod config;
pub mod document;
pub mod extraction;

pub use config::FilterCriteria;
pub use document::FigmaFile;
pub use extraction::{DocumentStructure, ExtractedText, ExtractionResult, FileMetadata, PageInfo};
