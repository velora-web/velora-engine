//! Tokenizer for the Velora web engine

use velora_core::VeloraResult;

/// Token types for HTML and CSS
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    /// HTML tag start
    TagStart(String),
    
    /// HTML tag end
    TagEnd(String),
    
    /// HTML self-closing tag
    SelfClosingTag(String),
    
    /// HTML attribute
    Attribute(String, String),
    
    /// HTML text content
    Text(String),
    
    /// HTML comment
    Comment(String),
    
    /// CSS rule start
    CssRuleStart,
    
    /// CSS rule end
    CssRuleEnd,
    
    /// CSS property
    CssProperty(String, String),
    
    /// End of file
    Eof,
}

/// Tokenizer for parsing HTML and CSS
#[derive(Debug)]
pub struct Tokenizer {
    /// Input source
    source: String,
    
    /// Current position
    position: usize,
}

impl Tokenizer {
    /// Create a new tokenizer
    pub fn new(source: String) -> Self {
        Self {
            source,
            position: 0,
        }
    }
    
    /// Get the next token
    pub fn next_token(&mut self) -> VeloraResult<Token> {
        // TODO: Implement actual tokenization
        if self.position >= self.source.len() {
            return Ok(Token::Eof);
        }
        
        // For now, return a simple text token
        let token = Token::Text(self.source[self.position..].chars().next().unwrap().to_string());
        self.position += 1;
        
        Ok(token)
    }
    
    /// Check if there are more tokens
    pub fn has_more(&self) -> bool {
        self.position < self.source.len()
    }
}
