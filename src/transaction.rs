//! Transaction implementation
//!
//! Package transaction management

use anyhow::Result;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::package::Package;

/// Transaction status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransactionStatus {
    Pending,
    Running,
    Completed,
    Failed,
    RolledBack,
}

/// Transaction operation
#[derive(Debug, Clone)]
pub enum Operation {
    Install { package: String, repo: String },
    Remove { package: String },
    Upgrade { package: String, from: String, to: String },
}

/// Transaction
pub struct Transaction {
    pub id: u64,
    pub operations: Vec<Operation>,
    pub status: TransactionStatus,
    pub created_at: u64,
    pub completed_at: Option<u64>,
    pub error: Option<String>,
}

impl Transaction {
    /// Create new transaction
    pub fn new() -> Self {
        Self {
            id: 0,
            operations: Vec::new(),
            status: TransactionStatus::Pending,
            created_at: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            completed_at: None,
            error: None,
        }
    }
    
    /// Create install transaction
    pub fn new_install(package: Package) -> Result<Self> {
        let mut transaction = Self::new();
        transaction.add_operation(Operation::Install {
            package: package.name,
            repo: package.repo,
        });
        Ok(transaction)
    }
    
    /// Create remove transaction
    pub fn new_remove(package: Package) -> Result<Self> {
        let mut transaction = Self::new();
        transaction.add_operation(Operation::Remove {
            package: package.name,
        });
        Ok(transaction)
    }
    
    /// Add operation
    pub fn add_operation(&mut self, operation: Operation) {
        self.operations.push(operation);
    }
    
    /// Set status
    pub fn set_status(&mut self, status: TransactionStatus) {
        self.status = status;
        if status == TransactionStatus::Completed || status == TransactionStatus::Failed {
            self.completed_at = Some(SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs());
        }
    }
    
    /// Set error
    pub fn set_error(&mut self, error: String) {
        self.error = Some(error);
        self.set_status(TransactionStatus::Failed);
    }
    
    /// Check if transaction is pending
    pub fn is_pending(&self) -> bool {
        self.status == TransactionStatus::Pending
    }
    
    /// Check if transaction is completed
    pub fn is_completed(&self) -> bool {
        self.status == TransactionStatus::Completed
    }
    
    /// Check if transaction is failed
    pub fn is_failed(&self) -> bool {
        self.status == TransactionStatus::Failed
    }
    
    /// Get operation count
    pub fn operation_count(&self) -> usize {
        self.operations.len()
    }
    
    /// Get duration
    pub fn duration(&self) -> Option<u64> {
        self.completed_at.map(|end| end - self.created_at)
    }
}
