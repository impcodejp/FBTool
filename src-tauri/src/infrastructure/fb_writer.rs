use std::io::Write;

use crate::error::AppError;

/// FB データファイルを Shift-JIS で書き込む（各レコード 200 バイト + CRLF）
pub fn write_fb_file(path: &str, records: &[Vec<u8>]) -> Result<(), AppError> {
    let mut file = std::fs::File::create(path)?;
    for record in records {
        file.write_all(record)?;
        file.write_all(b"\r\n")?;
    }
    Ok(())
}
