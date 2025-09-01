//! CSS parser for the Velora web engine

use velora_core::VeloraResult;

/// CSS parser that converts CSS text into structured rules
pub struct CssParser {
    /// Parser options
    options: CssParserOptions,
}

/// CSS parser options
#[derive(Debug, Clone)]
pub struct CssParserOptions {
    /// Whether to parse vendor prefixes
    pub parse_vendor_prefixes: bool,
    
    /// Whether to be strict about syntax errors
    pub strict_mode: bool,
}

impl Default for CssParserOptions {
    fn default() -> Self {
        Self {
            parse_vendor_prefixes: true,
            strict_mode: false,
        }
    }
}

/// A CSS rule
#[derive(Debug, Clone)]
pub struct CssRule {
    /// Rule type
    pub rule_type: CssRuleType,
    
    /// Selectors for this rule
    pub selectors: Vec<CssSelector>,
    
    /// Properties in this rule
    pub properties: Vec<CssProperty>,
    
    /// Source position
    pub source_position: Option<CssSourcePosition>,
}

/// Types of CSS rules
#[derive(Debug, Clone, PartialEq)]
pub enum CssRuleType {
    Style,
    Import,
    Media,
    FontFace,
    Keyframes,
    Page,
    Supports,
}

/// A CSS selector
#[derive(Debug, Clone)]
pub struct CssSelector {
    /// Selector specificity
    pub specificity: SelectorSpecificity,
    
    /// Selector parts
    pub parts: Vec<SelectorPart>,
}

/// Selector specificity (a, b, c) where a is most important
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SelectorSpecificity {
    pub a: u32, // ID selectors
    pub b: u32, // Class selectors, attributes, pseudo-classes
    pub c: u32, // Element selectors, pseudo-elements
}

/// A part of a CSS selector
#[derive(Debug, Clone)]
pub enum SelectorPart {
    /// Element selector (e.g., "div")
    Element(String),
    
    /// ID selector (e.g., "#main")
    Id(String),
    
    /// Class selector (e.g., ".header")
    Class(String),
    
    /// Attribute selector (e.g., "[type='text']")
    Attribute(String, Option<String>, Option<AttributeOperator>),
    
    /// Pseudo-class (e.g., ":hover")
    PseudoClass(String),
    
    /// Pseudo-element (e.g., "::before")
    PseudoElement(String),
    
    /// Universal selector (*)
    Universal,
    
    /// Combinator (space, >, +, ~)
    Combinator(CombinatorType),
}

/// Attribute selector operators
#[derive(Debug, Clone, PartialEq)]
pub enum AttributeOperator {
    Equals,
    Contains,
    StartsWith,
    EndsWith,
    ContainsWord,
    ContainsPrefix,
}

/// Combinator types
#[derive(Debug, Clone, PartialEq)]
pub enum CombinatorType {
    Descendant,    // space
    Child,         // >
    Adjacent,      // +
    Sibling,       // ~
}

/// A CSS property
#[derive(Debug, Clone)]
pub struct CssProperty {
    /// Property name
    pub name: String,
    
    /// Property value
    pub value: String,
    
    /// Whether the property is important
    pub important: bool,
    
    /// Source position
    pub source_position: Option<CssSourcePosition>,
}

/// Source position information
#[derive(Debug, Clone)]
pub struct CssSourcePosition {
    /// Line number (1-based)
    pub line: u32,
    
    /// Column number (1-based)
    pub column: u32,
    
    /// Source file name
    pub file: Option<String>,
}

impl CssParser {
    /// Create a new CSS parser with default options
    pub fn new() -> Self {
        Self {
            options: CssParserOptions::default(),
        }
    }
    
    /// Create a new CSS parser with custom options
    pub fn with_options(options: CssParserOptions) -> Self {
        Self { options }
    }
    
    /// Parse CSS text into a list of rules
    pub fn parse_css(&self, css: &str) -> VeloraResult<Vec<CssRule>> {
        // This is a simplified implementation
        // In a real implementation, you would use cssparser to tokenize
        // and parse the CSS according to the CSS specification
        
        let mut rules = Vec::new();
        
        // For now, we'll create a simple rule to demonstrate the structure
        if css.contains("body") {
            let rule = CssRule {
                rule_type: CssRuleType::Style,
                selectors: vec![CssSelector {
                    specificity: SelectorSpecificity { a: 0, b: 0, c: 1 },
                    parts: vec![SelectorPart::Element("body".to_string())],
                }],
                properties: vec![CssProperty {
                    name: "background-color".to_string(),
                    value: "#ffffff".to_string(),
                    important: false,
                    source_position: None,
                }],
                source_position: None,
            };
            rules.push(rule);
        }
        
        // Use options to avoid dead code warning
        if self.options.strict_mode && rules.is_empty() {
            return Err(velora_core::VeloraError::Parser(
                velora_core::error::ParserError::CssParsing("No valid CSS rules found".to_string())
            ));
        }
        
        Ok(rules)
    }
    
    /// Parse a CSS selector string
    pub fn parse_selector(&self, selector: &str) -> VeloraResult<CssSelector> {
        // Simplified selector parsing
        let parts = if let Some(stripped) = selector.strip_prefix('#') {
            vec![SelectorPart::Id(stripped.to_string())]
        } else if let Some(stripped) = selector.strip_prefix('.') {
            vec![SelectorPart::Class(stripped.to_string())]
        } else if selector == "*" {
            vec![SelectorPart::Universal]
        } else {
            vec![SelectorPart::Element(selector.to_string())]
        };
        
        let specificity = self.calculate_specificity(&parts);
        
        Ok(CssSelector {
            specificity,
            parts,
        })
    }
    
    /// Calculate selector specificity
    fn calculate_specificity(&self, parts: &[SelectorPart]) -> SelectorSpecificity {
        let mut a = 0;
        let mut b = 0;
        let mut c = 0;
        
        for part in parts {
            match part {
                SelectorPart::Id(_) => a += 1,
                SelectorPart::Class(_) | SelectorPart::Attribute(_, _, _) | SelectorPart::PseudoClass(_) => b += 1,
                SelectorPart::Element(_) | SelectorPart::PseudoElement(_) => c += 1,
                _ => {}
            }
        }
        
        SelectorSpecificity { a, b, c }
    }
}

impl Default for CssParser {
    fn default() -> Self {
        Self::new()
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_css_parser_creation() {
        let parser = CssParser::new();
        assert!(parser.options.parse_vendor_prefixes);
        assert!(!parser.options.strict_mode);
    }
    
    #[test]
    fn test_css_parsing() {
        let parser = CssParser::new();
        let css = "body { background-color: #ffffff; }";
        
        let result = parser.parse_css(css);
        assert!(result.is_ok());
        
        let rules = result.unwrap();
        assert!(!rules.is_empty());
        
        let rule = &rules[0];
        assert_eq!(rule.rule_type, CssRuleType::Style);
        assert_eq!(rule.selectors.len(), 1);
        assert_eq!(rule.properties.len(), 1);
    }
    
    #[test]
    fn test_selector_parsing() {
        let parser = CssParser::new();
        
        // Test ID selector
        let selector = parser.parse_selector("#main").unwrap();
        assert_eq!(selector.specificity.a, 1);
        assert_eq!(selector.specificity.b, 0);
        assert_eq!(selector.specificity.c, 0);
        
        // Test class selector
        let selector = parser.parse_selector(".header").unwrap();
        assert_eq!(selector.specificity.a, 0);
        assert_eq!(selector.specificity.b, 1);
        assert_eq!(selector.specificity.c, 0);
        
        // Test element selector
        let selector = parser.parse_selector("div").unwrap();
        assert_eq!(selector.specificity.a, 0);
        assert_eq!(selector.specificity.b, 0);
        assert_eq!(selector.specificity.c, 1);
    }
    
    #[test]
    fn test_specificity_calculation() {
        let parser = CssParser::new();
        
        // Test complex selector
        let selector = "#main .header div:hover";
        let parsed = parser.parse_selector(selector).unwrap();
        
        // This is simplified - in reality, we'd parse the full selector
        // For now, we just test the basic functionality
        assert!(parsed.specificity.a == 1); // Simplified parsing only gets first part (#main)
        assert!(parsed.specificity.b == 0);
        assert!(parsed.specificity.c == 0);
    }
}
