use serde::{Deserialize, Serialize};

use crate::error::AppError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeaderInfo {
    pub payment_date: String,
    pub bank_code: String,
    pub bank_name: String,
    pub branch_code: String,
    pub branch_name: String,
    pub deposit_type: u8,
    pub account_number: String,
}

/// グレゴリオ暦の "YYYY/MM/DD" を和暦 "YYMMDD" に変換する
pub fn to_wareki_yymmdd(date_str: &str) -> Result<String, AppError> {
    let parts: Vec<&str> = date_str.split('/').collect();
    if parts.len() != 3 {
        return Err(AppError::Date(format!(
            "日付フォーマットが不正です: {}",
            date_str
        )));
    }

    let year: u32 = parts[0]
        .parse()
        .map_err(|_| AppError::Date(format!("年の値が不正です: {}", parts[0])))?;
    let month: u32 = parts[1]
        .parse()
        .map_err(|_| AppError::Date(format!("月の値が不正です: {}", parts[1])))?;
    let day: u32 = parts[2]
        .parse()
        .map_err(|_| AppError::Date(format!("日の値が不正です: {}", parts[2])))?;

    if !(1..=12).contains(&month) || !(1..=31).contains(&day) {
        return Err(AppError::Date(format!(
            "有効な日付を入力してください: {}",
            date_str
        )));
    }

    let era_year = if year > 2019 || (year == 2019 && month >= 5) {
        year - 2018 // 令和
    } else if year > 1989 || (year == 1989 && (month > 1 || day >= 8)) {
        year - 1988 // 平成
    } else {
        year - 1925 // 昭和
    };

    Ok(format!("{:02}{:02}{:02}", era_year, month, day))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reiwa_conversion() {
        assert_eq!(to_wareki_yymmdd("2026/06/06").unwrap(), "080606");
        assert_eq!(to_wareki_yymmdd("2019/05/01").unwrap(), "010501");
    }

    #[test]
    fn test_heisei_conversion() {
        assert_eq!(to_wareki_yymmdd("2019/04/30").unwrap(), "310430");
        assert_eq!(to_wareki_yymmdd("1989/01/08").unwrap(), "010108");
    }

    #[test]
    fn test_invalid_format() {
        assert!(to_wareki_yymmdd("20260606").is_err());
        assert!(to_wareki_yymmdd("2026-06-06").is_err());
    }
}
