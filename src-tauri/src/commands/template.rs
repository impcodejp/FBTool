use encoding_rs::SHIFT_JIS;
use std::io::Write;

/// CSVテンプレートファイルを Shift-JIS で出力する
#[tauri::command]
pub fn export_csv_template(output_path: String) -> Result<(), String> {
    // ヘッダー行 + サンプルデータ 2 件
    let content = "金額,銀行名(カナ),支店名(カナ),摘要文字列,EDI\r\n\
                   100000,ﾄｳｷﾖｳｷﾞﾝｺｳ,ｼﾝｼﾞｭｸｼﾃﾝ,ｶﾌﾞｼｷｶﾞｲｼｬｻﾝﾌﾟﾙ,\r\n\
                   200000,ﾐｽﾞﾎｷﾞﾝｺｳ,ｼﾌﾞﾔｼﾃﾝ,ｶﾌﾞｼｷｶﾞｲｼｬﾃｽﾄ,EDI12345\r\n";

    let (encoded, _, had_errors) = SHIFT_JIS.encode(content);
    if had_errors {
        return Err("テンプレートの Shift-JIS エンコードに失敗しました".to_string());
    }

    let mut file = std::fs::File::create(&output_path)
        .map_err(|e| format!("ファイルの作成に失敗しました: {}", e))?;
    file.write_all(&encoded)
        .map_err(|e| format!("ファイルの書き込みに失敗しました: {}", e))?;

    Ok(())
}
