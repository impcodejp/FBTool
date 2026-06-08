use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CsvRecord {
    pub amount: u64,
    pub bank_name: String,
    pub branch_name: String,
    pub description: String,
    pub edi: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct CsvValidationError {
    pub row: usize,
    pub field: String,
    pub message: String,
}
