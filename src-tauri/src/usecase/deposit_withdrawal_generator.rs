use crate::domain::deposit_withdrawal_record::DepositWithdrawalRecord;
use crate::domain::fb_record::RecordBuilder;
use crate::domain::header_info::{day_part, to_wareki_yymmdd, HeaderInfo};
use crate::error::AppError;
use crate::usecase::common;

/// ヘッダー情報と入出金明細から全銀協フォーマットの FB データを生成する
///
/// 返り値は各レコードを 200 バイトのバイト列として返す
pub fn generate_fb_data(
    header: &HeaderInfo,
    records: &[DepositWithdrawalRecord],
) -> Result<Vec<Vec<u8>>, AppError> {
    let date_yymmdd = to_wareki_yymmdd(&header.payment_date)?;

    let mut fb_records: Vec<Vec<u8>> = Vec::new();

    fb_records.push(build_header_record(header, &date_yymmdd)?);

    for (i, record) in records.iter().enumerate() {
        let seq = (i as u64) + 1;
        fb_records.push(build_data_record(record, seq, &date_yymmdd)?);
    }

    fb_records.push(build_trailer_record(records)?);

    // エンドレコード時点での総件数（自身を含む）
    let total_records = fb_records.len() as u64 + 1;
    fb_records.push(common::build_end_record(total_records)?);

    Ok(fb_records)
}

/// ヘッダーレコード（200 バイト）を構築する
///
/// | 番号 | 項目名           | 属性   | 桁  |
/// |------|------------------|--------|-----|
/// | 1    | データ区分       | N(1)   | 1   |
/// | 2    | 種別コード       | N(2)   | 2   |
/// | 3    | コード区分       | N(1)   | 1   |
/// | 4    | 作成日           | N(6)   | 6   |
/// | 5    | 勘定日(自)       | N(6)   | 6   |
/// | 6    | 勘定日(至)       | N(6)   | 6   |
/// | 7    | 銀行コード       | N(4)   | 4   |
/// | 8    | 銀行名           | C(15)  | 15  |
/// | 9    | 支店コード       | N(3)   | 3   |
/// | 10   | 支店名           | C(15)  | 15  |
/// | 11   | ダミー           | N(3)   | 3   |
/// | 12   | 預金種目         | N(1)   | 1   |
/// | 13   | 口座番号         | N(10)  | 10  |
/// | 14   | 口座名           | C(40)  | 40  |
/// | 15   | 貸越区分         | N(1)   | 1   |
/// | 16   | 通帳・証書区分   | N(1)   | 1   |
/// | 17   | 取引前残高       | N(14)  | 14  |
/// | 18   | ダミー           | C(71)  | 71  |
fn build_header_record(header: &HeaderInfo, date_yymmdd: &str) -> Result<Vec<u8>, AppError> {
    let mut b = RecordBuilder::new(200);
    b.literal("1")
        .literal("03")
        .literal("0")
        .literal(date_yymmdd) // 作成日
        .literal(date_yymmdd) // 勘定日(自)
        .literal(date_yymmdd) // 勘定日(至)
        .numeric_str(4, &header.bank_code);
    b.char_field(15, &header.bank_name)?;
    b.numeric_str(3, &header.branch_code);
    b.char_field(15, &header.branch_name)?;
    b.literal("000") // ダミー
        .numeric_u64(1, header.deposit_type as u64)
        .numeric_str(10, &header.account_number);
    b.char_field(40, "")?; // 口座名（デモ用固定値: スペース埋め）
    b.literal("1") // 貸越区分（デモ用固定値: プラス）
        .literal("1") // 通帳・証書区分（通帳）
        .numeric_u64(14, 0); // 取引前残高（デモ用固定値: ゼロ）
    b.spaces(71); // ダミー
    Ok(b.build())
}

/// データレコード（200 バイト）を構築する
///
/// | 番号 | 項目名             | 属性   | 桁  |
/// |------|--------------------|--------|-----|
/// | 1    | データ区分         | N(1)   | 1   |
/// | 2    | 照会番号           | N(8)   | 8   |
/// | 3    | 勘定日             | N(6)   | 6   |
/// | 4    | 預入・払出日       | N(6)   | 6   |
/// | 5    | 入払区分           | N(1)   | 1   |
/// | 6    | 取引区分           | N(2)   | 2   |
/// | 7    | 取引金額           | N(12)  | 12  |
/// | 8    | うち他店券金額     | N(12)  | 12  |
/// | 9    | 交換呈示日         | N(6)   | 6   |
/// | 10   | 不渡返還日         | N(6)   | 6   |
/// | 11   | 手形・小切手区分   | N(1)   | 1   |
/// | 12   | 手形・小切手番号   | N(7)   | 7   |
/// | 13   | 僚店番号           | N(3)   | 3   |
/// | 14   | 振込依頼人番号     | C(10)  | 10  |
/// | 15   | 振込依頼人名       | C(48)  | 48  |
/// | 16   | 仕向銀行名         | C(15)  | 15  |
/// | 17   | 仕向店名           | C(15)  | 15  |
/// | 18   | 摘要内容           | C(20)  | 20  |
/// | 19   | EDI情報            | C(20)  | 20  |
/// | 20   | ダミー             | C(1)   | 1   |
fn build_data_record(
    record: &DepositWithdrawalRecord,
    seq: u64,
    date_yymmdd: &str,
) -> Result<Vec<u8>, AppError> {
    let day = day_part(date_yymmdd);

    let mut b = RecordBuilder::new(200);
    b.literal("2")
        .literal(day) // 照会番号（前半）: 勘定日の日
        .numeric_u64(6, seq) // 照会番号（後半）: 異動明細番号
        .literal(date_yymmdd) // 勘定日
        .literal(date_yymmdd) // 預入・払出日
        .literal(&record.transaction_flag)
        .literal(&record.transaction_category)
        .numeric_u64(12, record.amount)
        .numeric_u64(12, 0) // うち他店券金額（デモ用固定値）
        .numeric_u64(6, 0) // 交換呈示日（デモ用固定値）
        .numeric_u64(6, 0) // 不渡返還日（デモ用固定値）
        .numeric_u64(1, 0) // 手形・小切手区分（デモ用固定値）
        .numeric_u64(7, 0) // 手形・小切手番号（デモ用固定値）
        .numeric_u64(3, 0) // 僚店番号（デモ用固定値）
        .literal("1234567890"); // 振込依頼人番号（固定値）
    b.char_field(48, &record.description)?; // 振込依頼人名
    b.char_field(15, &record.bank_name)?; // 仕向銀行名
    b.char_field(15, &record.branch_name)?; // 仕向店名
    b.char_field(20, &record.summary_content)?; // 摘要内容
    b.char_field(20, &record.edi)?; // EDI情報
    b.spaces(1); // ダミー
    Ok(b.build())
}

/// トレーラーレコード（200 バイト）を構築する
///
/// | 番号 | 項目名             | 属性   | 桁  |
/// |------|--------------------|--------|-----|
/// | 1    | データ区分         | N(1)   | 1   |
/// | 2    | 入金件数           | N(6)   | 6   |
/// | 3    | 入金額合計         | N(13)  | 13  |
/// | 4    | 出金件数           | N(6)   | 6   |
/// | 5    | 出金額合計         | N(13)  | 13  |
/// | 6    | 貸越区分           | N(1)   | 1   |
/// | 7    | 取引後残高         | N(14)  | 14  |
/// | 8    | データレコード件数 | N(7)   | 7   |
/// | 9    | ダミー             | C(139) | 139 |
///
/// 貸越区分・取引後残高は取引前残高（デモ用固定値: ゼロ）を起点に、
/// 入金合計・出金合計から自動計算する（入金合計 >= 出金合計 なら「1」プラス、
/// それ以外は「2」マイナスとし残高は差額の絶対値をセットする）
fn build_trailer_record(records: &[DepositWithdrawalRecord]) -> Result<Vec<u8>, AppError> {
    let deposit_count = records.iter().filter(|r| r.transaction_flag == "1").count() as u64;
    let deposit_total: u64 = records
        .iter()
        .filter(|r| r.transaction_flag == "1")
        .map(|r| r.amount)
        .sum();
    let withdrawal_count = records.iter().filter(|r| r.transaction_flag == "2").count() as u64;
    let withdrawal_total: u64 = records
        .iter()
        .filter(|r| r.transaction_flag == "2")
        .map(|r| r.amount)
        .sum();

    let (overdraft_flag, balance) = if deposit_total >= withdrawal_total {
        ("1", deposit_total - withdrawal_total)
    } else {
        ("2", withdrawal_total - deposit_total)
    };

    let mut b = RecordBuilder::new(200);
    b.literal("8")
        .numeric_u64(6, deposit_count)
        .numeric_u64(13, deposit_total)
        .numeric_u64(6, withdrawal_count)
        .numeric_u64(13, withdrawal_total)
        .literal(overdraft_flag)
        .numeric_u64(14, balance)
        .numeric_u64(7, records.len() as u64)
        .spaces(139);
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

    fn sample_record(flag: &str, amount: u64) -> DepositWithdrawalRecord {
        DepositWithdrawalRecord {
            transaction_flag: flag.to_string(),
            transaction_category: "11".to_string(),
            amount,
            description: "ｶﾌﾞｼｷｶﾞｲｼｬｻﾝﾌﾟﾙ".to_string(),
            bank_name: "ﾄｳｷﾖｳｷﾞﾝｺｳ".to_string(),
            branch_name: "ｼﾝｼﾞｭｸｼﾃﾝ".to_string(),
            summary_content: "ﾆｭｳｷﾝ".to_string(),
            edi: String::new(),
        }
    }

    #[test]
    fn test_all_records_are_200_bytes() {
        let header = sample_header();
        let records = vec![sample_record("1", 100000)];

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
        assert_eq!(&hr[1..3], b"03"); // 種別コード
        assert_eq!(hr[3], b'0'); // コード区分
        assert_eq!(&hr[4..10], b"080606"); // 作成日（令和8年6月6日）
        assert_eq!(&hr[59..62], b"000"); // ダミー（項番11）
        assert_eq!(&hr[63..73], b"0001234567"); // 口座番号（"000"+7桁）
        assert_eq!(hr[113], b'1'); // 貸越区分
        assert_eq!(hr[114], b'1'); // 通帳・証書区分
        assert_eq!(&hr[115..129], b"00000000000000"); // 取引前残高
    }

    #[test]
    fn test_data_record_inquiry_number_increments() {
        let header = sample_header();
        let records = vec![sample_record("1", 100000), sample_record("2", 50000)];
        let fb_data = generate_fb_data(&header, &records).unwrap();

        let d1 = &fb_data[1];
        assert_eq!(&d1[1..9], b"06000001"); // 06日 + 連番1
        let d2 = &fb_data[2];
        assert_eq!(&d2[1..9], b"06000002"); // 06日 + 連番2
    }

    #[test]
    fn test_data_record_dummy_fields_are_zero() {
        let header = sample_header();
        let records = vec![sample_record("1", 100000)];
        let fb_data = generate_fb_data(&header, &records).unwrap();
        let d = &fb_data[1];

        // うち他店券金額(12) 交換呈示日(6) 不渡返還日(6) 手形小切手区分(1) 手形小切手番号(7) 僚店番号(3)
        // これらは項番7(取引金額 N12、[24,36))の直後から始まる
        assert_eq!(&d[36..48], b"000000000000"); // うち他店券金額
        assert_eq!(&d[48..54], b"000000"); // 交換呈示日
        assert_eq!(&d[54..60], b"000000"); // 不渡返還日
        assert_eq!(d[60], b'0'); // 手形・小切手区分
        assert_eq!(&d[61..68], b"0000000"); // 手形・小切手番号
        assert_eq!(&d[68..71], b"000"); // 僚店番号
    }

    #[test]
    fn test_trailer_totals_by_flag() {
        let header = sample_header();
        let records = vec![
            sample_record("1", 100000),
            sample_record("1", 200000),
            sample_record("2", 50000),
        ];
        let fb_data = generate_fb_data(&header, &records).unwrap();
        let trailer = &fb_data[fb_data.len() - 2];

        assert_eq!(trailer[0], b'8');
        assert_eq!(&trailer[1..7], b"000002"); // 入金件数
        assert_eq!(&trailer[7..20], b"0000000300000"); // 入金額合計
        assert_eq!(&trailer[20..26], b"000001"); // 出金件数
        assert_eq!(&trailer[26..39], b"0000000050000"); // 出金額合計
        assert_eq!(trailer[39], b'1'); // 貸越区分（プラス: 入金超過）
        assert_eq!(&trailer[40..54], b"00000000250000"); // 取引後残高（300000-50000）
        assert_eq!(&trailer[54..61], b"0000003"); // データレコード件数
    }

    #[test]
    fn test_trailer_overdraft_flag_when_withdrawal_exceeds() {
        let header = sample_header();
        let records = vec![sample_record("1", 10000), sample_record("2", 50000)];
        let fb_data = generate_fb_data(&header, &records).unwrap();
        let trailer = &fb_data[fb_data.len() - 2];

        assert_eq!(trailer[39], b'2'); // 貸越区分（マイナス: 出金超過）
        assert_eq!(&trailer[40..54], b"00000000040000"); // 取引後残高（|10000-50000|）
    }

    #[test]
    fn test_end_record_via_common() {
        let header = sample_header();
        let records = vec![sample_record("1", 100000)];
        let fb_data = generate_fb_data(&header, &records).unwrap();
        let end = &fb_data[fb_data.len() - 1];

        assert_eq!(end[0], b'9');
        assert_eq!(&end[1..11], b"0000000004"); // 総レコード件数
        assert_eq!(&end[11..16], b"00001"); // 伝送口座数
    }
}
