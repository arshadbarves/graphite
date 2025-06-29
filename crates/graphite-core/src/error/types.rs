//! Core error types for the AgentGraph framework.

use thiserror::Error;

/// Comprehensive error types for the AgentGraph core framework
#[derive(Error, Debug)]
pub enum CoreError {
    /// Node-related errors
    #[error("Node error in '{node_id}': {message}")]
    NodeError {
        /// The ID of the node that caused the error
        node_id: String,
        /// Error message
        message: String,
        /// Optional source error
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// Graph structure errors
    #[error("Graph structure error: {0}")]
    GraphStructure(String),

    /// State management errors
    #[error("State error: {0}")]
    StateError(String),

    /// Execution errors
    #[error("Execution error: {0}")]
    ExecutionError(String),

    /// Checkpointing errors
    #[error("Checkpointing error: {0}")]
    CheckpointError(String),

    /// Edge routing errors
    #[error("Edge routing error: {0}")]
    EdgeError(String),

    /// Serialization/deserialization errors
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    /// I/O errors
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    /// Timeout errors
    #[error("Operation timed out after {seconds} seconds")]
    Timeout {
        /// Number of seconds before timeout
        seconds: u64
    },

    /// Concurrency errors
    #[error("Concurrency error: {0}")]
    ConcurrencyError(String),

    /// Configuration errors
    #[error("Configuration error: {0}")]
    ConfigurationError(String),

    /// Resource errors (memory, file handles, etc.)
    #[error("Resource error: {0}")]
    ResourceError(String),

    /// Validation errors
    #[error("Validation error: {0}")]
    ValidationError(String),

    /// Generic internal errors
    #[error("Internal error: {0}")]
    Internal(String),

    /// Network/external service errors
    #[error("External service error: {0}")]
    ExternalServiceError(String),
}

impl CoreError {
    /// Create a new node error
    pub fn node_error<S: Into<String>>(
        node_id: S,
        message: S,
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    ) -> Self {
        Self::NodeError {
            node_id: node_id.into(),
            message: message.into(),
            source,
        }
    }

    /// Create a new graph structure error
    pub fn graph_structure<S: Into<String>>(message: S) -> Self {
        Self::GraphStructure(message.into())
    }

    /// Create a new state error
    pub fn state_error<S: Into<String>>(message: S) -> Self {
        Self::StateError(message.into())
    }

    /// Create a new execution error
    pub fn execution_error<S: Into<String>>(message: S) -> Self {
        Self::ExecutionError(message.into())
    }

    /// Create a new checkpoint error
    pub fn checkpoint_error<S: Into<String>>(message: S) -> Self {
        Self::CheckpointError(message.into())
    }

    /// Create a new edge error
    pub fn edge_error<S: Into<String>>(message: S) -> Self {
        Self::EdgeError(message.into())
    }

    /// Create a new timeout error
    pub fn timeout(seconds: u64) -> Self {
        Self::Timeout { seconds }
    }

    /// Create a new concurrency error
    pub fn concurrency_error<S: Into<String>>(message: S) -> Self {
        Self::ConcurrencyError(message.into())
    }

    /// Create a new validation error
    pub fn validation_error<S: Into<String>>(message: S) -> Self {
        Self::ValidationError(message.into())
    }

    /// Create a new configuration error
    pub fn configuration_error<S: Into<String>>(message: S) -> Self {
        Self::ConfigurationError(message.into())
    }

    /// Create a new resource error
    pub fn resource_error<S: Into<String>>(message: S) -> Self {
        Self::ResourceError(message.into())
    }

    /// Create a new external service error
    pub fn external_service_error<S: Into<String>>(message: S) -> Self {
        Self::ExternalServiceError(message.into())
    }

    /// Create a new internal error
    pub fn internal<S: Into<String>>(message: S) -> Self {
        Self::Internal(message.into())
    }

    /// Check if this error is recoverable
    pub fn is_recoverable(&self) -> bool {
        matches!(
            self,
            CoreError::Timeout { .. }
                | CoreError::ResourceError(_)
                | CoreError::ConcurrencyError(_)
                | CoreError::ExternalServiceError(_)
        )
    }

    /// Get the error category for metrics/logging
    pub fn category(&self) -> &'static str {
        match self {
            CoreError::NodeError { .. } => "node",
            CoreError::GraphStructure(_) => "graph_structure",
            CoreError::StateError(_) => "state",
            CoreError::ExecutionError(_) => "execution",
            CoreError::CheckpointError(_) => "checkpoint",
            CoreError::EdgeError(_) => "edge",
            CoreError::SerializationError(_) => "serialization",
            CoreError::IoError(_) => "io",
            CoreError::Timeout { .. } => "timeout",
            CoreError::ConcurrencyError(_) => "concurrency",
            CoreError::ConfigurationError(_) => "configuration",
            CoreError::ResourceError(_) => "resource",
            CoreError::ValidationError(_) => "validation",
            CoreError::Internal(_) => "internal",
            CoreError::ExternalServiceError(_) => "external_service",
        }
    }
}

// Tests for CoreError types and functionality
#[cfg(test)]
mod tests {
    use super::*;
    use std::io;

    #[test]
    fn test_node_error_creation() {
        let node_error = CoreError::node_error("test_node", "test message", None);
        assert_eq!(node_error.category(), "node");
        assert!(!node_error.is_recoverable());

        // Test with source error
        let source_err = Box::new(io::Error::new(io::ErrorKind::NotFound, "file not found"));
        let node_error_with_source = CoreError::node_error("node1", "failed to read", Some(source_err));
        assert!(node_error_with_source.source().is_some());
    }

    #[test]
    fn test_timeout_error() {
        let timeout_error = CoreError::timeout(30);
        assert_eq!(timeout_error.category(), "timeout");
        assert!(timeout_error.is_recoverable());
        assert_eq!(timeout_error.to_string(), "Operation timed out after 30 seconds");
    }

    #[test]
    fn test_error_display_messages() {
        let validation_error = CoreError::validation_error("Invalid input");
        assert_eq!(validation_error.to_string(), "Validation error: Invalid input");

        let graph_error = CoreError::graph_structure("Cycle detected");
        assert_eq!(graph_error.to_string(), "Graph structure error: Cycle detected");

        let execution_error = CoreError::execution_error("Node failed");
        assert_eq!(execution_error.to_string(), "Execution error: Node failed");
    }

    #[test]
    fn test_all_error_categories() {
        let test_cases = vec![
            (CoreError::node_error("n1", "msg", None), "node"),
            (CoreError::graph_structure("msg"), "graph_structure"),
            (CoreError::state_error("msg"), "state"),
            (CoreError::execution_error("msg"), "execution"),
            (CoreError::checkpoint_error("msg"), "checkpoint"),
            (CoreError::edge_error("msg"), "edge"),
            (CoreError::timeout(10), "timeout"),
            (CoreError::concurrency_error("msg"), "concurrency"),
            (CoreError::configuration_error("msg"), "configuration"),
            (CoreError::resource_error("msg"), "resource"),
            (CoreError::validation_error("msg"), "validation"),
            (CoreError::internal("msg"), "internal"),
            (CoreError::external_service_error("msg"), "external_service"),
        ];

        for (error, expected_category) in test_cases {
            assert_eq!(error.category(), expected_category);
        }
    }

    #[test]
    fn test_recoverable_errors() {
        // Recoverable errors
        assert!(CoreError::timeout(30).is_recoverable());
        assert!(CoreError::resource_error("Out of memory").is_recoverable());
        assert!(CoreError::concurrency_error("Lock contention").is_recoverable());
        assert!(CoreError::external_service_error("Network timeout").is_recoverable());

        // Non-recoverable errors
        assert!(!CoreError::validation_error("Invalid data").is_recoverable());
        assert!(!CoreError::internal("Logic error").is_recoverable());
        assert!(!CoreError::graph_structure("Invalid graph").is_recoverable());
        assert!(!CoreError::node_error("n1", "critical failure", None).is_recoverable());
    }

    #[test]
    fn test_error_conversion_from_std_errors() {
        // Test automatic conversion from serde_json::Error
        let json_str = r#"{"invalid": json"#;
        let json_result: Result<serde_json::Value, _> = serde_json::from_str(json_str);
        let json_error = json_result.unwrap_err();
        let core_error: CoreError = json_error.into();
        assert_eq!(core_error.category(), "serialization");

        // Test automatic conversion from std::io::Error
        let io_error = io::Error::new(io::ErrorKind::PermissionDenied, "Access denied");
        let core_error: CoreError = io_error.into();
        assert_eq!(core_error.category(), "io");
    }

    #[test]
    fn test_error_chaining() {
        let root_cause = io::Error::new(io::ErrorKind::NotFound, "file missing");
        let node_error = CoreError::node_error(
            "file_reader_node",
            "Failed to read configuration",
            Some(Box::new(root_cause))
        );

        // Test that we can access the source error
        assert!(node_error.source().is_some());
        let source = node_error.source().unwrap();
        assert_eq!(source.to_string(), "file missing");
    }

    #[test]
    fn test_error_debug_formatting() {
        let error = CoreError::validation_error("Test error");
        let debug_output = format!("{:?}", error);
        assert!(debug_output.contains("ValidationError"));
        assert!(debug_output.contains("Test error"));
    }

    #[test]
    fn test_constructor_methods_consistency() {
        // Test that constructor methods produce correct variants
        let errors = vec![
            CoreError::state_error("test"),
            CoreError::execution_error("test"),
            CoreError::checkpoint_error("test"),
            CoreError::edge_error("test"),
            CoreError::concurrency_error("test"),
            CoreError::configuration_error("test"),
            CoreError::resource_error("test"),
            CoreError::validation_error("test"),
            CoreError::internal("test"),
            CoreError::external_service_error("test"),
        ];

        // Each should have correct category
        let expected_categories = vec![
            "state", "execution", "checkpoint", "edge", "concurrency",
            "configuration", "resource", "validation", "internal", "external_service"
        ];

        for (error, expected) in errors.iter().zip(expected_categories.iter()) {
            assert_eq!(error.category(), *expected);
        }
    }
}