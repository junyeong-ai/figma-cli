//! Configuration structures for extraction

use regex::Regex;

#[derive(Debug, Clone, Default)]
pub struct FilterCriteria {
    pub page_names: Option<Vec<String>>,
    pub page_ids: Option<Vec<String>>,
    pub page_pattern: Option<Regex>,
    pub frame_pattern: Option<Regex>,
    pub include_hidden: bool,
}

impl FilterCriteria {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_pages(mut self, pages: Vec<String>) -> Self {
        self.page_names = Some(pages);
        self
    }

    pub fn with_page_ids(mut self, page_ids: Vec<String>) -> Self {
        self.page_ids = Some(page_ids);
        self
    }

    pub fn with_page_pattern(mut self, pattern: Regex) -> Self {
        self.page_pattern = Some(pattern);
        self
    }

    pub fn with_frame_pattern(mut self, pattern: Regex) -> Self {
        self.frame_pattern = Some(pattern);
        self
    }

    pub const fn with_include_hidden(mut self, include: bool) -> Self {
        self.include_hidden = include;
        self
    }

    pub fn matches_page(&self, page_name: &str) -> bool {
        if let Some(pages) = &self.page_names
            && !pages
                .iter()
                .any(|p| p == page_name || page_name.contains(p))
        {
            return false;
        }

        if let Some(pattern) = &self.page_pattern
            && !pattern.is_match(page_name)
        {
            return false;
        }

        true
    }

    pub fn matches_page_id(&self, page_id: &str) -> bool {
        if let Some(ids) = &self.page_ids {
            ids.iter().any(|id| id == page_id)
        } else {
            true
        }
    }

    pub fn matches_frame(&self, frame_name: &str) -> bool {
        if let Some(pattern) = &self.frame_pattern {
            pattern.is_match(frame_name)
        } else {
            true
        }
    }

    pub const fn is_empty(&self) -> bool {
        self.page_names.is_none()
            && self.page_ids.is_none()
            && self.page_pattern.is_none()
            && self.frame_pattern.is_none()
            && !self.include_hidden
    }
}

#[derive(Debug, Clone)]
pub struct ExtractionOptions {
    pub filter: FilterCriteria,
    pub include_metadata: bool,
    pub include_images: bool,
    pub image_dir: Option<String>,
    pub image_format: ImageFormat,
    pub image_scale: f64,
    pub max_concurrent_images: usize,
    pub timeout_ms: u64,
}

impl Default for ExtractionOptions {
    fn default() -> Self {
        Self {
            filter: FilterCriteria::default(),
            include_metadata: false,
            include_images: false,
            image_dir: None,
            image_format: ImageFormat::Png,
            image_scale: 2.0,
            max_concurrent_images: 50,
            timeout_ms: 30000,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageFormat {
    Png,
    Jpg,
    Svg,
    Pdf,
}

impl ImageFormat {
    pub const fn extension(&self) -> &str {
        match self {
            Self::Png => "png",
            Self::Jpg => "jpg",
            Self::Svg => "svg",
            Self::Pdf => "pdf",
        }
    }

    pub const fn mime_type(&self) -> &str {
        match self {
            Self::Png => "image/png",
            Self::Jpg => "image/jpeg",
            Self::Svg => "image/svg+xml",
            Self::Pdf => "application/pdf",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_criteria_matches_page() {
        let filter =
            FilterCriteria::new().with_pages(vec!["Page1".to_string(), "Page2".to_string()]);

        assert!(filter.matches_page("Page1"));
        assert!(filter.matches_page("Page2"));
        assert!(!filter.matches_page("Page3"));
    }

    #[test]
    fn test_filter_criteria_page_pattern() {
        let pattern = regex::Regex::new(r"^Design").unwrap();
        let filter = FilterCriteria::new().with_page_pattern(pattern);

        assert!(filter.matches_page("Design System"));
        assert!(filter.matches_page("Design Components"));
        assert!(!filter.matches_page("Other Page"));
    }

    #[test]
    fn test_filter_criteria_matches_frame() {
        let pattern = regex::Regex::new(r"Button").unwrap();
        let filter = FilterCriteria::new().with_frame_pattern(pattern);

        assert!(filter.matches_frame("Primary Button"));
        assert!(filter.matches_frame("Button Group"));
        assert!(!filter.matches_frame("Input Field"));
    }

    #[test]
    fn test_filter_criteria_is_empty() {
        let empty_filter = FilterCriteria::new();
        assert!(empty_filter.is_empty());

        let with_pages = FilterCriteria::new().with_pages(vec!["Page1".to_string()]);
        assert!(!with_pages.is_empty());

        let with_hidden = FilterCriteria::new().with_include_hidden(true);
        assert!(!with_hidden.is_empty());
    }

    #[test]
    fn test_image_format_extension() {
        assert_eq!(ImageFormat::Png.extension(), "png");
        assert_eq!(ImageFormat::Jpg.extension(), "jpg");
        assert_eq!(ImageFormat::Svg.extension(), "svg");
        assert_eq!(ImageFormat::Pdf.extension(), "pdf");
    }

    #[test]
    fn test_image_format_mime_type() {
        assert_eq!(ImageFormat::Png.mime_type(), "image/png");
        assert_eq!(ImageFormat::Jpg.mime_type(), "image/jpeg");
        assert_eq!(ImageFormat::Svg.mime_type(), "image/svg+xml");
        assert_eq!(ImageFormat::Pdf.mime_type(), "application/pdf");
    }
}
