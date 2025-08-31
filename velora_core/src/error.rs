//! Error types for the Velora web engine

use thiserror::Error;

/// Main error type for the Velora web engine
#[derive(Error, Debug)]
pub enum VeloraError {
    #[error("DOM error: {0}")]
    Dom(#[from] DomError),
    
    #[error("Parser error: {0}")]
    Parser(#[from] ParserError),
    
    #[error("Layout error: {0}")]
    Layout(#[from] LayoutError),
    
    #[error("Paint error: {0}")]
    Paint(#[from] PaintError),
    
    #[error("Network error: {0}")]
    Network(#[from] NetworkError),
    
    #[error("JavaScript runtime error: {0}")]
    JsRuntime(#[from] JsRuntimeError),
    
    #[error("Platform error: {0}")]
    Platform(#[from] PlatformError),
    
    #[error("Invalid URL: {0}")]
    InvalidUrl(String),
    
    #[error("Resource not found: {0}")]
    ResourceNotFound(String),
    
    #[error("Invalid state: {0}")]
    InvalidState(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("Unknown error: {0}")]
    Unknown(String),
}

/// DOM-related errors
#[derive(Error, Debug)]
pub enum DomError {
    #[error("Invalid node type")]
    InvalidNodeType,
    
    #[error("Node not found: {0}")]
    NodeNotFound(String),
    
    #[error("Invalid attribute: {0}")]
    InvalidAttribute(String),
    
    #[error("DOM tree corruption: {0}")]
    TreeCorruption(String),
}

/// Parser-related errors
#[derive(Error, Debug)]
pub enum ParserError {
    #[error("HTML parsing failed: {0}")]
    HtmlParsing(String),
    
    #[error("CSS parsing failed: {0}")]
    CssParsing(String),
    
    #[error("JavaScript parsing failed: {0}")]
    JsParsing(String),
    
    #[error("Invalid encoding: {0}")]
    InvalidEncoding(String),
    
    #[error("Unexpected end of input")]
    UnexpectedEof,
}

/// Layout-related errors
#[derive(Error, Debug)]
pub enum LayoutError {
    #[error("Layout calculation failed: {0}")]
    CalculationFailed(String),
    
    #[error("Invalid layout constraints: {0}")]
    InvalidConstraints(String),
    
    #[error("Layout overflow: {0}")]
    Overflow(String),
    
    #[error("Circular dependency detected")]
    CircularDependency,
}

/// Paint-related errors
#[derive(Error, Debug)]
pub enum PaintError {
    #[error("Rendering failed: {0}")]
    RenderingFailed(String),
    
    #[error("Graphics context error: {0}")]
    GraphicsContext(String),
    
    #[error("Invalid paint operation: {0}")]
    InvalidOperation(String),
    
    #[error("Resource allocation failed: {0}")]
    ResourceAllocation(String),
}

/// Network-related errors
#[derive(Error, Debug)]
pub enum NetworkError {
    #[error("Request failed: {0}")]
    RequestFailed(String),
    
    #[error("Connection timeout")]
    Timeout,
    
    #[error("Invalid response: {0}")]
    InvalidResponse(String),
    
    #[error("SSL/TLS error: {0}")]
    SslError(String),
    
    #[error("Redirect limit exceeded")]
    RedirectLimitExceeded,
}

/// JavaScript runtime errors
#[derive(Error, Debug)]
pub enum JsRuntimeError {
    #[error("Script execution failed: {0}")]
    ExecutionFailed(String),
    
    #[error("Module loading failed: {0}")]
    ModuleLoading(String),
    
    #[error("Memory allocation failed")]
    MemoryAllocation,
    
    #[error("Invalid script: {0}")]
    InvalidScript(String),
}

/// Platform-specific errors
#[derive(Error, Debug)]
pub enum PlatformError {
    #[error("Window creation failed: {0}")]
    WindowCreation(String),
    
    #[error("Graphics initialization failed: {0}")]
    GraphicsInit(String),
    
    #[error("Input handling failed: {0}")]
    InputHandling(String),
    
    #[error("Platform not supported: {0}")]
    NotSupported(String),
}

/// Result type for Velora operations
pub type VeloraResult<T> = Result<T, VeloraError>;

impl From<String> for VeloraError {
    fn from(s: String) -> Self {
        VeloraError::Unknown(s)
    }
}

impl From<&str> for VeloraError {
    fn from(s: &str) -> Self {
        VeloraError::Unknown(s.to_string())
    }
}
