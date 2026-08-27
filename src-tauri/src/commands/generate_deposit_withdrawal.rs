use crate::domain::header_info::HeaderInfo;
use crate::infrastructure::{deposit_withdrawal_csv_reader, fb_writer};
use crate::usecase::deposit_withdrawal_generator;

/// 入出金明細の FB データファイルを生成する
///
/// 1. CSV ファイルを読み込み・バリデーション
/// 2. FB レコードを生成
/// 3. Shift-JIS でファイルに書き込む
#[tauri::command]
pub fn generate_fb_deposit_withdrawal(
    header_info: HeaderInfo,
    csv_path: String,
    output_path: String,
) -> Result<(), String> {
    let (records, errors) =
        deposit_withdrawal_csv_reader::read_csv(&csv_path).map_err(|e| e.to_string())?;

    if !errors.is_empty() {
        let messages: Vec<String> = errors
            .iter()
            .map(|e| format!("行{} {}: {}", e.row, e.field, e.message))
            .collect();
        return Err(format!(
            "CSVにバリデーションエラーがあります:\n{}",
            messages.join("\n")
        ));
    }

    let fb_data = deposit_withdrawal_generator::generate_fb_data(&header_info, &records)
        .map_err(|e| e.to_string())?;

    fb_writer::write_fb_file(&output_path, &fb_data).map_err(|e| e.to_string())?;

    Ok(())
}
