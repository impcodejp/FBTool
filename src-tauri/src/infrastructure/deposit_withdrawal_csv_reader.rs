use encoding_rs::SHIFT_JIS;

use crate::domain::csv_record::CsvValidationError;
use crate::domain::deposit_withdrawal_record::DepositWithdrawalRecord;
use crate::error::AppError;
use crate::infrastructure::csv_reader::is_valid_halfwidth;

const VALID_TRANSACTION_CATEGORIES: [&str; 7] = ["10", "11", "12", "13", "14", "18", "19"];

fn halfwidth_len(s: &str) -> usize {
    s.chars().count()
}

/// Shift-JIS でエンコードされた入出金明細 CSV ファイルを読み込み、バリデーションを行う
///
/// # Returns
/// `(有効レコード一覧, バリデーションエラー一覧)`
pub fn read_csv(
    path: &str,
) -> Result<(Vec<DepositWithdrawalRecord>, Vec<CsvValidationError>), AppError> {
    let bytes = std::fs::read(path)?;
    let (decoded, _, _) = SHIFT_JIS.decode(&bytes);

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .flexible(true)
        .from_reader(decoded.as_bytes());

    let mut records: Vec<DepositWithdrawalRecord> = Vec::new();
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

        if row.len() < 8 {
            errors.push(CsvValidationError {
                row: row_num,
                field: "行".to_string(),
                message: format!(
                    "カラム数が不足しています（{}カラム、8カラム必要）",
                    row.len()
                ),
            });
            row_num += 1;
            continue;
        }

        let transaction_flag = row[0].trim().to_string();
        let transaction_category = row[1].trim().to_string();
        let amount_str = row[2].trim();
        let description = row[3].trim().to_string();
        let bank_name = row[4].trim().to_string();
        let branch_name = row[5].trim().to_string();
        let summary_content = row[6].trim().to_string();
        let edi = row[7].trim().to_string();

        let mut row_has_error = false;

        // 入払区分（必須、"1" または "2"）
        if transaction_flag != "1" && transaction_flag != "2" {
            errors.push(CsvValidationError {
                row: row_num,
                field: "入払区分".to_string(),
                message: "1(入金) または 2(出金) を入力してください".to_string(),
            });
            row_has_error = true;
        }

        // 取引区分（必須、許可コードのいずれか）
        if !VALID_TRANSACTION_CATEGORIES.contains(&transaction_category.as_str()) {
            errors.push(CsvValidationError {
                row: row_num,
                field: "取引区分".to_string(),
                message: "10/11/12/13/14/18/19 のいずれかを入力してください".to_string(),
            });
            row_has_error = true;
        }

        // 取引金額（必須、1以上、12桁以内）
        let amount = match amount_str.parse::<u64>() {
            Ok(v) if (1..=999_999_999_999).contains(&v) => v,
            Ok(0) => {
                errors.push(CsvValidationError {
                    row: row_num,
                    field: "取引金額".to_string(),
                    message: "1以上の整数を入力してください".to_string(),
                });
                row_has_error = true;
                0
            }
            Ok(_) => {
                errors.push(CsvValidationError {
                    row: row_num,
                    field: "取引金額".to_string(),
                    message: "12桁以内の数字を入力してください".to_string(),
                });
                row_has_error = true;
                0
            }
            Err(_) => {
                errors.push(CsvValidationError {
                    row: row_num,
                    field: "取引金額".to_string(),
                    message: "数字を入力してください".to_string(),
                });
                row_has_error = true;
                0
            }
        };

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

        // 仕向銀行名(カナ)（任意、半角カタカナ・半角英数字、15文字以内）
        if !bank_name.is_empty() {
            if !is_valid_halfwidth(&bank_name) {
                errors.push(CsvValidationError {
                    row: row_num,
                    field: "仕向銀行名(カナ)".to_string(),
                    message: "半角カタカナ・半角英数字のみ使用できます".to_string(),
                });
                row_has_error = true;
            } else if halfwidth_len(&bank_name) > 15 {
                errors.push(CsvValidationError {
                    row: row_num,
                    field: "仕向銀行名(カナ)".to_string(),
                    message: "15文字以内で入力してください".to_string(),
                });
                row_has_error = true;
            }
        }

        // 仕向店名(カナ)（任意、半角カタカナ・半角英数字、15文字以内）
        if !branch_name.is_empty() {
            if !is_valid_halfwidth(&branch_name) {
                errors.push(CsvValidationError {
                    row: row_num,
                    field: "仕向店名(カナ)".to_string(),
                    message: "半角カタカナ・半角英数字のみ使用できます".to_string(),
                });
                row_has_error = true;
            } else if halfwidth_len(&branch_name) > 15 {
                errors.push(CsvValidationError {
                    row: row_num,
                    field: "仕向店名(カナ)".to_string(),
                    message: "15文字以内で入力してください".to_string(),
                });
                row_has_error = true;
            }
        }

        // 摘要内容（任意、20文字以内）
        if !summary_content.is_empty() && halfwidth_len(&summary_content) > 20 {
            errors.push(CsvValidationError {
                row: row_num,
                field: "摘要内容".to_string(),
                message: "20文字以内で入力してください".to_string(),
            });
            row_has_error = true;
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
            records.push(DepositWithdrawalRecord {
                transaction_flag,
                transaction_category,
                amount,
                description,
                bank_name,
                branch_name,
                summary_content,
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
    use std::io::Write;
    use std::sync::atomic::{AtomicU64, Ordering};

    static COUNTER: AtomicU64 = AtomicU64::new(0);

    fn write_temp_csv(content: &str) -> String {
        let dir = std::env::temp_dir();
        let unique = COUNTER.fetch_add(1, Ordering::SeqCst);
        let path = dir.join(format!(
            "fbtool_test_dw_{}_{}.csv",
            std::process::id(),
            unique
        ));
        let (encoded, _, _) = SHIFT_JIS.encode(content);
        let mut file = std::fs::File::create(&path).unwrap();
        file.write_all(&encoded).unwrap();
        path.to_str().unwrap().to_string()
    }

    #[test]
    fn test_valid_row() {
        let path = write_temp_csv(
            "入払区分,取引区分,取引金額,摘要文字列,仕向銀行名(カナ),仕向店名(カナ),摘要内容,EDI\r\n\
             1,11,100000,ｶﾌﾞｼｷｶﾞｲｼｬｻﾝﾌﾟﾙ,ﾄｳｷﾖｳｷﾞﾝｺｳ,ｼﾝｼﾞｭｸｼﾃﾝ,ﾆｭｳｷﾝ,\r\n",
        );
        let (records, errors) = read_csv(&path).unwrap();
        assert!(errors.is_empty());
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].transaction_flag, "1");
        assert_eq!(records[0].transaction_category, "11");
        assert_eq!(records[0].amount, 100000);
        std::fs::remove_file(path).ok();
    }

    #[test]
    fn test_invalid_transaction_flag() {
        let path = write_temp_csv(
            "入払区分,取引区分,取引金額,摘要文字列,仕向銀行名(カナ),仕向店名(カナ),摘要内容,EDI\r\n\
             3,11,100000,ﾃｽﾄ,,,,\r\n",
        );
        let (records, errors) = read_csv(&path).unwrap();
        assert!(records.is_empty());
        assert!(errors.iter().any(|e| e.field == "入払区分"));
        std::fs::remove_file(path).ok();
    }

    #[test]
    fn test_invalid_transaction_category() {
        let path = write_temp_csv(
            "入払区分,取引区分,取引金額,摘要文字列,仕向銀行名(カナ),仕向店名(カナ),摘要内容,EDI\r\n\
             1,99,100000,ﾃｽﾄ,,,,\r\n",
        );
        let (records, errors) = read_csv(&path).unwrap();
        assert!(records.is_empty());
        assert!(errors.iter().any(|e| e.field == "取引区分"));
        std::fs::remove_file(path).ok();
    }

    #[test]
    fn test_optional_fields_can_be_empty() {
        let path = write_temp_csv(
            "入払区分,取引区分,取引金額,摘要文字列,仕向銀行名(カナ),仕向店名(カナ),摘要内容,EDI\r\n\
             2,10,50000,ﾃｽﾄ,,,,\r\n",
        );
        let (records, errors) = read_csv(&path).unwrap();
        assert!(errors.is_empty());
        assert_eq!(records.len(), 1);
        std::fs::remove_file(path).ok();
    }

    #[test]
    fn test_fullwidth_description_is_error() {
        let path = write_temp_csv(
            "入払区分,取引区分,取引金額,摘要文字列,仕向銀行名(カナ),仕向店名(カナ),摘要内容,EDI\r\n\
             1,11,100000,テスト,,,,\r\n",
        );
        let (records, errors) = read_csv(&path).unwrap();
        assert!(records.is_empty());
        assert!(errors.iter().any(|e| e.field == "摘要文字列"));
        std::fs::remove_file(path).ok();
    }

    #[test]
    fn test_insufficient_columns() {
        let path = write_temp_csv(
            "入払区分,取引区分,取引金額,摘要文字列,仕向銀行名(カナ),仕向店名(カナ),摘要内容,EDI\r\n\
             1,11,100000\r\n",
        );
        let (records, errors) = read_csv(&path).unwrap();
        assert!(records.is_empty());
        assert!(errors.iter().any(|e| e.field == "行"));
        std::fs::remove_file(path).ok();
    }
}
