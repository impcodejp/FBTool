use crate::domain::fb_record::RecordBuilder;
use crate::error::AppError;

/// エンドレコード（200 バイト）を構築する。FB 種別に依存しない共通レコード。
///
/// | 番号 | 項目名         | 属性   | 桁  |
/// |------|----------------|--------|-----|
/// | 1    | データ区分     | N(1)   | 1   |
/// | 2    | 総レコード件数 | N(10)  | 10  |
/// | 3    | 伝送口座数     | N(5)   | 5   |
/// | 4    | ダミー         | C(184) | 184 |
pub(crate) fn build_end_record(total_records: u64) -> Result<Vec<u8>, AppError> {
    let mut b = RecordBuilder::new(200);
    b.literal("9")
        .numeric_u64(10, total_records)
        .numeric_u64(5, 1) // 伝送口座数（ヘッダー件数）
        .spaces(184);
    Ok(b.build())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_end_record_is_200_bytes() {
        let record = build_end_record(4).unwrap();
        assert_eq!(record.len(), 200);
        assert_eq!(record[0], b'9');
        assert_eq!(&record[1..11], b"0000000004");
        assert_eq!(&record[11..16], b"00001");
    }
}
