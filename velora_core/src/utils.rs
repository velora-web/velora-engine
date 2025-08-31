//! Utility functions and helpers for the Velora web engine

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::LazyLock;

/// Thread-safe ID generator for creating unique identifiers
#[derive(Debug)]
pub struct IdGenerator {
    counter: AtomicU64,
}

impl IdGenerator {
    /// Create a new ID generator starting from 1
    pub fn new() -> Self {
        Self {
            counter: AtomicU64::new(1),
        }
    }
    
    /// Create a new ID generator starting from a specific value
    pub fn with_start(start: u64) -> Self {
        Self {
            counter: AtomicU64::new(start),
        }
    }
    
    /// Generate the next unique ID
    pub fn next(&self) -> u64 {
        self.counter.fetch_add(1, Ordering::Relaxed)
    }
    
    /// Get the current counter value without incrementing
    pub fn current(&self) -> u64 {
        self.counter.load(Ordering::Relaxed)
    }
    
    /// Reset the counter to a specific value
    pub fn reset(&self, value: u64) {
        self.counter.store(value, Ordering::Relaxed);
    }
}

impl Default for IdGenerator {
    fn default() -> Self {
        Self::new()
    }
}

/// Global ID generator instance
pub static GLOBAL_ID_GENERATOR: LazyLock<IdGenerator> = LazyLock::new(|| IdGenerator::new());

/// Generate a new global unique ID
pub fn next_id() -> u64 {
    GLOBAL_ID_GENERATOR.next()
}

/// Utility for measuring time durations
pub struct Timer {
    start: std::time::Instant,
}

impl Timer {
    /// Create a new timer
    pub fn new() -> Self {
        Self {
            start: std::time::Instant::now(),
        }
    }
    
    /// Get the elapsed time since creation
    pub fn elapsed(&self) -> std::time::Duration {
        self.start.elapsed()
    }
    
    /// Get elapsed time in milliseconds
    pub fn elapsed_ms(&self) -> u64 {
        self.start.elapsed().as_millis() as u64
    }
    
    /// Get elapsed time in microseconds
    pub fn elapsed_us(&self) -> u64 {
        self.start.elapsed().as_micros() as u64
    }
    
    /// Reset the timer
    pub fn reset(&mut self) {
        self.start = std::time::Instant::now();
    }
}

impl Default for Timer {
    fn default() -> Self {
        Self::new()
    }
}

/// Utility for building strings efficiently
pub struct StringBuilder {
    parts: Vec<String>,
}

impl StringBuilder {
    /// Create a new string builder
    pub fn new() -> Self {
        Self { parts: Vec::new() }
    }
    
    /// Add a string part
    pub fn push(&mut self, s: impl Into<String>) {
        self.parts.push(s.into());
    }
    
    /// Add a string part with a separator
    pub fn push_with_separator(&mut self, separator: &str, s: impl Into<String>) {
        if !self.parts.is_empty() {
            self.parts.push(separator.to_string());
        }
        self.parts.push(s.into());
    }
    
    /// Build the final string
    pub fn build(self) -> String {
        self.parts.join("")
    }
    
    /// Build the final string with a separator between parts
    pub fn build_with_separator(self, separator: &str) -> String {
        self.parts.join(separator)
    }
}

impl Default for StringBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Utility for working with CSS values
pub mod css {
    use crate::types::{CssValue, CssUnit, Color};
    
    /// Parse a CSS length value from a string
    pub fn parse_length(s: &str) -> Option<CssValue> {
        let s = s.trim();
        
        if s == "auto" {
            return Some(CssValue::Keyword("auto".to_string()));
        }
        
        if s == "none" {
            return Some(CssValue::Keyword("none".to_string()));
        }
        
        if s.ends_with("px") {
            if let Ok(value) = s[..s.len() - 2].parse::<f32>() {
                return Some(CssValue::Length(value, CssUnit::Px));
            }
        }
        
        if s.ends_with("em") {
            if let Ok(value) = s[..s.len() - 2].parse::<f32>() {
                return Some(CssValue::Length(value, CssUnit::Em));
            }
        }
        
        if s.ends_with("rem") {
            if let Ok(value) = s[..s.len() - 3].parse::<f32>() {
                return Some(CssValue::Length(value, CssUnit::Rem));
            }
        }
        
        if s.ends_with("%") {
            if let Ok(value) = s[..s.len() - 1].parse::<f32>() {
                return Some(CssValue::Percentage(value));
            }
        }
        
        if let Ok(value) = s.parse::<f32>() {
            return Some(CssValue::Number(value));
        }
        
        None
    }
    
    /// Parse a CSS color value from a string
    pub fn parse_color(s: &str) -> Option<CssValue> {
        let s = s.trim();
        
        // Handle named colors
        match s.to_lowercase().as_str() {
            "black" => Some(CssValue::Color(Color::black())),
            "white" => Some(CssValue::Color(Color::white())),
            "red" => Some(CssValue::Color(Color::rgb(255, 0, 0))),
            "green" => Some(CssValue::Color(Color::rgb(0, 255, 0))),
            "blue" => Some(CssValue::Color(Color::rgb(0, 0, 255))),
            "transparent" => Some(CssValue::Color(Color::transparent())),
            _ => {
                // Handle hex colors
                if s.starts_with('#') {
                    if let Some(color) = parse_hex_color(s) {
                        return Some(CssValue::Color(color));
                    }
                }
                
                // Handle rgb/rgba functions
                if s.starts_with("rgb(") || s.starts_with("rgba(") {
                    if let Some(color) = parse_rgb_color(s) {
                        return Some(CssValue::Color(color));
                    }
                }
                
                None
            }
        }
    }
    
    /// Parse a hex color value
    fn parse_hex_color(s: &str) -> Option<Color> {
        let s = &s[1..]; // Remove #
        
        match s.len() {
            3 => {
                // #RGB format
                let r = u8::from_str_radix(&s[0..1], 16).ok()? * 17;
                let g = u8::from_str_radix(&s[1..2], 16).ok()? * 17;
                let b = u8::from_str_radix(&s[2..3], 16).ok()? * 17;
                Some(Color::rgb(r, g, b))
            }
            6 => {
                // #RRGGBB format
                let r = u8::from_str_radix(&s[0..2], 16).ok()?;
                let g = u8::from_str_radix(&s[2..4], 16).ok()?;
                let b = u8::from_str_radix(&s[4..6], 16).ok()?;
                Some(Color::rgb(r, g, b))
            }
            8 => {
                // #RRGGBBAA format
                let r = u8::from_str_radix(&s[0..2], 16).ok()?;
                let g = u8::from_str_radix(&s[2..4], 16).ok()?;
                let b = u8::from_str_radix(&s[4..6], 16).ok()?;
                let a = u8::from_str_radix(&s[6..8], 16).ok()?;
                Some(Color::rgba(r, g, b, a))
            }
            _ => None,
        }
    }
    
    /// Parse an rgb/rgba color value
    fn parse_rgb_color(s: &str) -> Option<Color> {
        // Simple parsing for rgb(r,g,b) and rgba(r,g,b,a)
        let start = if s.starts_with("rgba(") { 5 } else { 4 };
        let end = s.len() - 1; // Remove closing )
        
        let values: Vec<f32> = s[start..end]
            .split(',')
            .filter_map(|v| v.trim().parse::<f32>().ok())
            .collect();
        
        match values.len() {
            3 => Some(Color::rgb(
                values[0].clamp(0.0, 255.0) as u8,
                values[1].clamp(0.0, 255.0) as u8,
                values[2].clamp(0.0, 255.0) as u8,
            )),
            4 => Some(Color::rgba(
                values[0].clamp(0.0, 255.0) as u8,
                values[1].clamp(0.0, 255.0) as u8,
                values[2].clamp(0.0, 255.0) as u8,
                (values[3].clamp(0.0, 1.0) * 255.0) as u8,
            )),
            _ => None,
        }
    }
}

/// Utility for working with URLs
pub mod url {
    use std::collections::HashMap;
    
    /// Parse query parameters from a URL
    pub fn parse_query_params(query: &str) -> HashMap<String, String> {
        let mut params = HashMap::new();
        
        for pair in query.split('&') {
            if let Some((key, value)) = pair.split_once('=') {
                let key = urlencoding::decode(key).map(|cow| cow.into_owned()).unwrap_or_else(|_| key.to_string());
                let value = urlencoding::decode(value).map(|cow| cow.into_owned()).unwrap_or_else(|_| value.to_string());
                params.insert(key, value);
            }
        }
        
        params
    }
    
    /// Build query parameters from a HashMap
    pub fn build_query_params(params: &HashMap<String, String>) -> String {
        if params.is_empty() {
            return String::new();
        }
        
        let mut pairs: Vec<String> = params
            .iter()
            .map(|(k, v)| format!("{}={}", urlencoding::encode(k), urlencoding::encode(v)))
            .collect();
        
        pairs.sort(); // Consistent ordering
        pairs.join("&")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{CssValue, CssUnit, Color};
    
    #[test]
    fn test_id_generator() {
        let gen = IdGenerator::new();
        let id1 = gen.next();
        let id2 = gen.next();
        assert_eq!(id1, 1);
        assert_eq!(id2, 2);
    }
    
    #[test]
    fn test_timer() {
        let timer = Timer::new();
        std::thread::sleep(std::time::Duration::from_millis(10));
        assert!(timer.elapsed_ms() >= 10);
    }
    
    #[test]
    fn test_string_builder() {
        let mut builder = StringBuilder::new();
        builder.push("Hello");
        builder.push_with_separator(" ", "World");
        assert_eq!(builder.build(), "Hello World");
    }
    
    #[test]
    fn test_css_parsing() {
        assert_eq!(
            css::parse_length("10px"),
            Some(CssValue::Length(10.0, CssUnit::Px))
        );
        
        assert_eq!(
            css::parse_color("#ff0000"),
            Some(CssValue::Color(Color::rgb(255, 0, 0)))
        );
    }
}
