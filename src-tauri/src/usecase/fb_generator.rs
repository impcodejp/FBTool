use crate::domain::csv_record::CsvRecord;
use crate::domain::fb_record::RecordBuilder;
use crate::domain::header_info::{to_wareki_yymmdd, HeaderInfo};
use crate::error::AppError;
use crate::usecase::common;

/// ヘッダー情報と入金明細から全銀協フォーマットの FB データを生成する
///
/// 返り値は各レコードを 200 バイトのバイト列として返す
pub fn generate_fb_data(
    header: &HeaderInfo,
    records: &[CsvRecord],
) -> Result<Vec<Vec<u8>>, AppError> {
    let date_yymmdd = to_wareki_yymmdd(&header.payment_date)?;

    let mut fb_records: Vec<Vec<u8>> = Vec::new();

    fb_records.push(build_header_record(header, &date_yymmdd)?);

    for (i, record) in records.iter().enumerate() {
        let inquiry_num = (i as u64) + 1;
        fb_records.push(build_data_record(record, inquiry_num, &date_yymmdd)?);
    }

    let transfer_count = records.len() as u64;
    let transfer_sum: u64 = records.iter().map(|r| r.amount).sum();
    fb_records.push(build_trailer_record(transfer_count, transfer_sum)?);

    // エンドレコード時点での総件数（自身を含む）
    let total_records = fb_records.len() as u64 + 1;
    fb_records.push(common::build_end_record(total_records)?);

    Ok(fb_records)
}

/// ヘッダーレコード（200 バイト）を構築する
///
/// | 番号 | 項目名       | 属性   | 桁  |
/// |------|-------------|--------|-----|
/// | 1    | データ区分   | N(1)   | 1   |
/// | 2    | 種別コード   | N(2)   | 2   |
/// | 3    | コード区分   | N(1)   | 1   |
/// | 4    | 作成日       | N(6)   | 6   |
/// | 5    | 勘定日(自)   | N(6)   | 6   |
/// | 6    | 勘定日(至)   | N(6)   | 6   |
/// | 7    | 銀行コード   | N(4)   | 4   |
/// | 8    | 銀行名       | C(15)  | 15  |
/// | 9    | 支店コード   | N(3)   | 3   |
/// | 10   | 支店名       | C(15)  | 15  |
/// | 11   | 預金種目     | N(1)   | 1   |
/// | 12   | 口座番号     | N(7)   | 7   |
/// | 13   | 口座名       | C(40)  | 40  |
/// | 14   | ダミー       | C(93)  | 93  |
fn build_header_record(header: &HeaderInfo, date_yymmdd: &str) -> Result<Vec<u8>, AppError> {
    let mut b = RecordBuilder::new(200);
    b.literal("1")
        .literal("01")
        .literal("0")
        .literal(date_yymmdd) // 作成日
        .literal(date_yymmdd) // 勘定日(自)
        .literal(date_yymmdd) // 勘定日(至)
        .numeric_str(4, &header.bank_code);
    b.char_field(15, &header.bank_name)?;
    b.numeric_str(3, &header.branch_code);
    b.char_field(15, &header.branch_name)?;
    b.numeric_u64(1, header.deposit_type as u64)
        .numeric_str(7, &header.account_number);
    b.char_field(40, "")?; // 口座名（スペース埋め）
    b.spaces(93); // ダミー
    Ok(b.build())
}

/// データレコード（200 バイト）を構築する
///
/// | 番号 | 項目名             | 属性   | 桁  |
/// |------|--------------------|--------|-----|
/// | 1    | データ区分         | N(1)   | 1   |
/// | 2    | 照会番号           | N(6)   | 6   |
/// | 3    | 勘定日             | N(6)   | 6   |
/// | 4    | 起算日             | N(6)   | 6   |
/// | 5    | 金額(1)            | N(10)  | 10  |
/// | 6    | 他店券金額(1)      | N(10)  | 10  |
/// | 7    | 振込依頼人番号     | C(10)  | 10  |
/// | 8    | 振込依頼人名       | C(48)  | 48  |
/// | 9    | 仕向銀行名         | C(15)  | 15  |
/// | 10   | 仕向店名           | C(15)  | 15  |
/// | 11   | 取消区分           | N(1)   | 1   |
/// | 12   | 金額(2)            | N(12)  | 12  |
/// | 13   | 他店券金額(2)      | N(12)  | 12  |
/// | 14   | EDI情報            | C(20)  | 20  |
/// | 15   | ダミー(1)          | C(8)   | 8   |
/// | 16   | カナコメントリスト | C(6)   | 6   |
/// | 17   | ダミー(2)          | C(14)  | 14  |
fn build_data_record(
    record: &CsvRecord,
    inquiry_num: u64,
    date_yymmdd: &str,
) -> Result<Vec<u8>, AppError> {
    // 金額が 10 桁以内 → 金額(1)、10 桁超 → 金額(2)
    let (amount_1, amount_2) = if record.amount <= 9_999_999_999 {
        (record.amount, 0u64)
    } else {
        (0u64, record.amount)
    };

    let mut b = RecordBuilder::new(200);
    b.literal("2")
        .numeric_u64(6, inquiry_num)
        .literal(date_yymmdd) // 勘定日
        .literal(date_yymmdd) // 起算日
        .numeric_u64(10, amount_1)
        .numeric_u64(10, 0) // 他店券金額(1)
        .literal("1234567890"); // 振込依頼人番号（固定値）
    b.char_field(48, &record.description)?; // 振込依頼人名
    b.char_field(15, &record.bank_name)?; // 仕向銀行名
    b.char_field(15, &record.branch_name)?; // 仕向店名
    b.literal("0") // 取消区分（0:振込）
        .numeric_u64(12, amount_2)
        .numeric_u64(12, 0); // 他店券金額(2)
    b.char_field(20, &record.edi)?; // EDI情報
    b.spaces(8).spaces(6).spaces(14); // ダミー群
    Ok(b.build())
}

/// トレーラーレコード（200 バイト）を構築する
///
/// | 番号 | 項目名         | 属性   | 桁  |
/// |------|----------------|--------|-----|
/// | 1    | データ区分     | N(1)   | 1   |
/// | 2    | 振込合計件数   | N(6)   | 6   |
/// | 3    | 振込合計金額   | N(12)  | 12  |
/// | 4    | 取消合計件数   | N(6)   | 6   |
/// | 5    | 取消合計金額   | N(12)  | 12  |
/// | 6    | ダミー         | C(163) | 163 |
fn build_trailer_record(transfer_count: u64, transfer_sum: u64) -> Result<Vec<u8>, AppError> {
    let mut b = RecordBuilder::new(200);
    b.literal("8")
        .numeric_u64(6, transfer_count)
        .numeric_u64(12, transfer_sum)
        .numeric_u64(6, 0) // 取消合計件数
        .numeric_u64(12, 0) // 取消合計金額
        .spaces(163);
    Ok(b.build())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::header_info::HeaderInfo;

    fn sample_header() -> HeaderInfo {
        HeaderInfo {
            payment_date: "2026/06/06".to_string(),
            bank_code: "0005".to_string(),
            bank_name: "ﾐﾂﾋﾞｼﾕｰｴﾌｼﾞｴｲ".to_string(),
            branch_code: "001".to_string(),
            branch_name: "ﾎﾝﾃﾝ".to_string(),
            deposit_type: 1,
            account_number: "1234567".to_string(),
        }
    }

    #[test]
    fn test_all_records_are_200_bytes() {
        let header = sample_header();
        let records = vec![CsvRecord {
            amount: 100000,
            bank_name: "ﾄｳｷﾖｳｷﾞﾝｺｳ".to_string(),
            branch_name: "ｼﾝｼﾞｭｸｼﾃﾝ".to_string(),
            description: "ｶﾌﾞｼｷｶﾞｲｼｬｻﾝﾌﾟﾙ".to_string(),
            edi: String::new(),
        }];

        let fb_data = generate_fb_data(&header, &records).unwrap();
        // header + 1 data + trailer + end = 4 records
        assert_eq!(fb_data.len(), 4);
        for (i, record) in fb_data.iter().enumerate() {
            assert_eq!(record.len(), 200, "Record {} is not 200 bytes", i);
        }
    }

    #[test]
    fn test_header_record_fields() {
        let header = sample_header();
        let fb_data = generate_fb_data(&header, &[]).unwrap();
        let hr = &fb_data[0];

        assert_eq!(hr[0], b'1'); // データ区分
        assert_eq!(&hr[1..3], b"01"); // 種別コード
        assert_eq!(hr[3], b'0'); // コード区分
        assert_eq!(&hr[4..10], b"080606"); // 作成日（令和8年6月6日）
    }

    #[test]
    fn test_trailer_record_total() {
        let header = sample_header();
        let records = vec![
            CsvRecord {
                amount: 100000,
                bank_name: String::new(),
                branch_name: String::new(),
                description: "TEST".to_string(),
                edi: String::new(),
            },
            CsvRecord {
                amount: 200000,
                bank_name: String::new(),
                branch_name: String::new(),
                description: "TEST2".to_string(),
                edi: String::new(),
            },
        ];

        let fb_data = generate_fb_data(&header, &records).unwrap();
        let trailer = &fb_data[fb_data.len() - 2]; // エンドの 1 つ前
        assert_eq!(trailer[0], b'8'); // データ区分
        assert_eq!(&trailer[1..7], b"000002"); // 振込合計件数
        assert_eq!(&trailer[7..19], b"000000300000"); // 振込合計金額
    }

    #[test]
    fn test_amount_field_split() {
        let header = sample_header();
        let large_amount = 10_000_000_001u64; // 11 桁 → 金額(2)
        let records = vec![CsvRecord {
            amount: large_amount,
            bank_name: String::new(),
            branch_name: String::new(),
            description: "TEST".to_string(),
            edi: String::new(),
        }];

        let fb_data = generate_fb_data(&header, &records).unwrap();
        let data = &fb_data[1]; // データレコード
                                // 金額(1) は 0
        assert_eq!(&data[19..29], b"0000000000");
        // 金額(2) は large_amount
        assert_eq!(&data[128..140], b"010000000001");
    }
}
