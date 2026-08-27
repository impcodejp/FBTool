use serde::Serialize;

use crate::domain::csv_record::CsvValidationError;
use crate::infrastructure::deposit_withdrawal_csv_reader;

/// 入出金明細 CSV ファイルの読み込み結果（フロントエンドに返す DTO）
#[derive(Debug, Serialize)]
pub struct DepositWithdrawalReadResult {
    pub preview: Vec<DepositWithdrawalPreviewRow>,
    pub total_count: usize,
    pub errors: Vec<CsvValidationError>,
}

/// プレビュー表示用の行データ
#[derive(Debug, Serialize)]
pub struct DepositWithdrawalPreviewRow {
    pub row: usize,
    pub transaction_flag: String,
    pub transaction_category: String,
    pub amount: String,
    pub description: String,
    pub bank_name: String,
    pub branch_name: String,
    pub summary_content: String,
    pub edi: String,
}

/// 入出金明細 CSV ファイルを読み込み、バリデーション結果とプレビューデータを返す
#[tauri::command]
pub fn read_deposit_withdrawal_csv_records(
    csv_path: String,
) -> Result<DepositWithdrawalReadResult, String> {
    let (records, errors) =
        deposit_withdrawal_csv_reader::read_csv(&csv_path).map_err(|e| e.to_string())?;

    let preview: Vec<DepositWithdrawalPreviewRow> = records
        .iter()
        .enumerate()
        .map(|(i, r)| DepositWithdrawalPreviewRow {
            row: i + 2,
            transaction_flag: r.transaction_flag.clone(),
            transaction_category: r.transaction_category.clone(),
            amount: r.amount.to_string(),
            description: r.description.clone(),
            bank_name: r.bank_name.clone(),
            branch_name: r.branch_name.clone(),
            summary_content: r.summary_content.clone(),
            edi: r.edi.clone(),
        })
        .collect();

    let total_count = records.len();

    Ok(DepositWithdrawalReadResult {
        preview,
        total_count,
        errors,
    })
}
