use serde::{Deserialize, Serialize};
use std::io::Write;

/// ヘッダーフォームの保存・読み込み用 DTO（deposit_type は文字列 "1"/"2"）
#[derive(Debug, Serialize, Deserialize)]
pub struct HeaderFormData {
    pub payment_date: String,
    pub bank_code: String,
    pub bank_name: String,
    pub branch_code: String,
    pub branch_name: String,
    pub deposit_type: String,
    pub account_number: String,
}

/// ヘッダ情報を JSON ファイルに出力する
#[tauri::command]
pub fn export_header_info(data: HeaderFormData, output_path: String) -> Result<(), String> {
    let json = serde_json::to_string_pretty(&data).map_err(|e| format!("JSON変換エラー: {}", e))?;

    let mut file = std::fs::File::create(&output_path)
        .map_err(|e| format!("ファイルの作成に失敗しました: {}", e))?;
    file.write_all(json.as_bytes())
        .map_err(|e| format!("ファイルの書き込みに失敗しました: {}", e))?;

    Ok(())
}

/// ヘッダ情報 JSON ファイルを読み込んでフォームデータを返す
#[tauri::command]
pub fn import_header_info(file_path: String) -> Result<HeaderFormData, String> {
    let content = std::fs::read_to_string(&file_path)
        .map_err(|e| format!("ファイルの読み込みに失敗しました: {}", e))?;

    let data: HeaderFormData =
        serde_json::from_str(&content).map_err(|e| format!("ファイルの形式が不正です: {}", e))?;

    Ok(data)
}
