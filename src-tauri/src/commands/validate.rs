use serde::Serialize;

use crate::domain::csv_record::CsvValidationError;
use crate::infrastructure::csv_reader;

/// CSV ファイルの読み込み結果（フロントエンドに返す DTO）
#[derive(Debug, Serialize)]
pub struct CsvReadResult {
    pub preview: Vec<CsvPreviewRow>,
    pub total_count: usize,
    pub errors: Vec<CsvValidationError>,
}

/// プレビュー表示用の行データ
#[derive(Debug, Serialize)]
pub struct CsvPreviewRow {
    pub row: usize,
    pub amount: String,
    pub bank_name: String,
    pub branch_name: String,
    pub description: String,
    pub edi: String,
}

/// CSV ファイルを読み込み、バリデーション結果とプレビューデータを返す
#[tauri::command]
pub fn read_csv_records(csv_path: String) -> Result<CsvReadResult, String> {
    let (records, errors) = csv_reader::read_csv(&csv_path).map_err(|e| e.to_string())?;

    let preview: Vec<CsvPreviewRow> = records
        .iter()
        .enumerate()
        .map(|(i, r)| CsvPreviewRow {
            row: i + 2,
            amount: r.amount.to_string(),
            bank_name: r.bank_name.clone(),
            branch_name: r.branch_name.clone(),
            description: r.description.clone(),
            edi: r.edi.clone(),
        })
        .collect();

    let total_count = records.len();

    Ok(CsvReadResult {
        preview,
        total_count,
        errors,
    })
}
