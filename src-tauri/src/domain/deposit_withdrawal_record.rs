use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepositWithdrawalRecord {
    pub transaction_flag: String,
    pub transaction_category: String,
    pub amount: u64,
    pub description: String,
    pub bank_name: String,
    pub branch_name: String,
    pub summary_content: String,
    pub edi: String,
}
