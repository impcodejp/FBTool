use encoding_rs::SHIFT_JIS;

use crate::domain::csv_record::{CsvRecord, CsvValidationError};
use crate::error::AppError;

/// 半角文字（ASCII + 半角カタカナ）のみかチェック
pub fn is_valid_halfwidth(s: &str) -> bool {
    s.chars()
        .all(|c| c.is_ascii() || ('\u{FF61}'..='\u{FF9F}').contains(&c))
}

/// 半角文字列の文字数（Shift-JIS バイト数に等しい）
fn halfwidth_len(s: &str) -> usize {
    s.chars().count()
}

/// Shift-JIS でエンコードされた CSV ファイルを読み込み、バリデーションを行う
///
/// # Returns
/// `(有効レコード一覧, バリデーションエラー一覧)`
pub fn read_csv(path: &str) -> Result<(Vec<CsvRecord>, Vec<CsvValidationError>), AppError> {
    let bytes = std::fs::read(path)?;
    let (decoded, _, _) = SHIFT_JIS.decode(&bytes);

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .flexible(true)
        .from_reader(decoded.as_bytes());

    let mut records: Vec<CsvRecord> = Vec::new();
    let mut errors: Vec<CsvValidationError> = Vec::new();
    let mut row_num = 2usize; // ヘッダー行が 1 行目なのでデータは 2 行目から

    for result in rdr.records() {
        let row = match result {
            Ok(r) => r,
            Err(e) => {
                errors.push(CsvValidationError {
                    row: row_num,
                    field: "行".to_string(),
                    message: format!("CSV解析エラー: {}", e),
                });
                row_num += 1;
                continue;
            }
        };

        if row.len() < 5 {
            errors.push(CsvValidationError {
                row: row_num,
                field: "行".to_string(),
                message: format!(
                    "カラム数が不足しています（{}カラム、5カラム必要）",
                    row.len()
                ),
            });
            row_num += 1;
            continue;
        }

        let amount_str = row[0].trim();
        let bank_name = row[1].trim().to_string();
        let branch_name = row[2].trim().to_string();
        let description = row[3].trim().to_string();
        let edi = row[4].trim().to_string();

        let mut row_has_error = false;

        // 金額バリデーション
        let amount = match amount_str.parse::<u64>() {
            Ok(v) if (1..=999_999_999_999).contains(&v) => v,
            Ok(0) => {
                errors.push(CsvValidationError {
                    row: row_num,
                    field: "金額".to_string(),
                    message: "1以上の整数を入力してください".to_string(),
                });
                row_has_error = true;
                0
            }
            Ok(_) => {
                errors.push(CsvValidationError {
                    row: row_num,
                    field: "金額".to_string(),
                    message: "12桁以内の数字を入力してください".to_string(),
                });
                row_has_error = true;
                0
            }
            Err(_) => {
                errors.push(CsvValidationError {
                    row: row_num,
                    field: "金額".to_string(),
                    message: "数字を入力してください".to_string(),
                });
                row_has_error = true;
                0
            }
        };

        // 銀行名（任意、半角カタカナ・半角英数字、15文字以内）
        if !bank_name.is_empty() {
            if !is_valid_halfwidth(&bank_name) {
                errors.push(CsvValidationError {
                    row: row_num,
                    field: "銀行名(カナ)".to_string(),
                    message: "半角カタカナ・半角英数字のみ使用できます".to_string(),
                });
                row_has_error = true;
            } else if halfwidth_len(&bank_name) > 15 {
                errors.push(CsvValidationError {
                    row: row_num,
                    field: "銀行名(カナ)".to_string(),
                    message: "15文字以内で入力してください".to_string(),
                });
                row_has_error = true;
            }
        }

        // 支店名（任意、半角カタカナ・半角英数字、15文字以内）
        if !branch_name.is_empty() {
            if !is_valid_halfwidth(&branch_name) {
                errors.push(CsvValidationError {
                    row: row_num,
                    field: "支店名(カナ)".to_string(),
                    message: "半角カタカナ・半角英数字のみ使用できます".to_string(),
                });
                row_has_error = true;
            } else if halfwidth_len(&branch_name) > 15 {
                errors.push(CsvValidationError {
                    row: row_num,
                    field: "支店名(カナ)".to_string(),
                    message: "15文字以内で入力してください".to_string(),
                });
                row_has_error = true;
            }
        }

        // 摘要文字列（必須、半角カタカナ・半角英数字、48文字以内）
        if description.is_empty() {
            errors.push(CsvValidationError {
                row: row_num,
                field: "摘要文字列".to_string(),
                message: "摘要文字列は必須です".to_string(),
            });
            row_has_error = true;
        } else {
            if !is_valid_halfwidth(&description) {
                errors.push(CsvValidationError {
                    row: row_num,
                    field: "摘要文字列".to_string(),
                    message: "半角カタカナ・半角英数字のみ使用できます".to_string(),
                });
                row_has_error = true;
            }
            if halfwidth_len(&description) > 48 {
                errors.push(CsvValidationError {
                    row: row_num,
                    field: "摘要文字列".to_string(),
                    message: "48文字以内で入力してください".to_string(),
                });
                row_has_error = true;
            }
        }

        // EDI（任意、20文字以内）
        if !edi.is_empty() && halfwidth_len(&edi) > 20 {
            errors.push(CsvValidationError {
                row: row_num,
                field: "EDI".to_string(),
                message: "20文字以内で入力してください".to_string(),
            });
            row_has_error = true;
        }

        if !row_has_error {
            records.push(CsvRecord {
                amount,
                bank_name,
                branch_name,
                description,
                edi,
            });
        }

        row_num += 1;
    }

    Ok((records, errors))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_halfwidth() {
        assert!(is_valid_halfwidth("ABC123"));
        assert!(is_valid_halfwidth("ｱｲｳｴｵ"));
        assert!(is_valid_halfwidth("ABC ｱｲｳ"));
        assert!(!is_valid_halfwidth("アイウ")); // 全角カタカナは不可
        assert!(!is_valid_halfwidth("テスト")); // 全角は不可
    }
}
