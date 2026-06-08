use encoding_rs::SHIFT_JIS;

use crate::error::AppError;

/// 固定長バイトバッファを構築するビルダー（初期値はスペース埋め）
pub struct RecordBuilder {
    buf: Vec<u8>,
    pos: usize,
}

impl RecordBuilder {
    pub fn new(size: usize) -> Self {
        Self {
            buf: vec![0x20u8; size],
            pos: 0,
        }
    }

    /// ASCII リテラルを書き込む（文字列長分だけ進む）
    pub fn literal(&mut self, s: &str) -> &mut Self {
        let bytes = s.as_bytes();
        self.buf[self.pos..self.pos + bytes.len()].copy_from_slice(bytes);
        self.pos += bytes.len();
        self
    }

    /// 数値を右詰めゼロパディングで書き込む
    pub fn numeric_u64(&mut self, width: usize, value: u64) -> &mut Self {
        let s = format!("{}", value);
        self.write_numeric_bytes(width, s.as_bytes());
        self
    }

    /// 数字文字列を右詰めゼロパディングで書き込む
    pub fn numeric_str(&mut self, width: usize, value: &str) -> &mut Self {
        let s = value.trim_start_matches('0');
        let s = if s.is_empty() { "0" } else { s };
        self.write_numeric_bytes(width, s.as_bytes());
        self
    }

    fn write_numeric_bytes(&mut self, width: usize, bytes: &[u8]) {
        if bytes.len() >= width {
            // 桁数が width 以上の場合、末尾 width バイトを採用
            let start = bytes.len() - width;
            self.buf[self.pos..self.pos + width].copy_from_slice(&bytes[start..]);
        } else {
            let pad = width - bytes.len();
            // ゼロパディング
            for b in &mut self.buf[self.pos..self.pos + pad] {
                *b = b'0';
            }
            self.buf[self.pos + pad..self.pos + width].copy_from_slice(bytes);
        }
        self.pos += width;
    }

    /// 文字フィールドを Shift-JIS でエンコードして左詰め書き込み（残りスペース）
    pub fn char_field(&mut self, width: usize, value: &str) -> Result<&mut Self, AppError> {
        if !value.is_empty() {
            let (encoded, _, had_errors) = SHIFT_JIS.encode(value);
            if had_errors {
                return Err(AppError::Encoding(format!(
                    "Shift-JIS変換不可の文字が含まれています: {}",
                    value
                )));
            }
            let encoded_bytes = encoded.as_ref();
            let copy_len = encoded_bytes.len().min(width);
            self.buf[self.pos..self.pos + copy_len].copy_from_slice(&encoded_bytes[..copy_len]);
        }
        self.pos += width;
        Ok(self)
    }

    /// スペース埋めフィールド（既に初期値がスペースなので位置を進めるだけ）
    pub fn spaces(&mut self, width: usize) -> &mut Self {
        self.pos += width;
        self
    }

    pub fn build(self) -> Vec<u8> {
        self.buf
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_numeric_u64_padding() {
        let mut b = RecordBuilder::new(10);
        b.numeric_u64(6, 42);
        assert_eq!(&b.buf[..6], b"000042");
    }

    #[test]
    fn test_char_field_left_aligned() {
        let mut b = RecordBuilder::new(20);
        b.char_field(10, "ABC").unwrap();
        assert_eq!(&b.buf[..3], b"ABC");
        assert_eq!(&b.buf[3..10], b"       ");
    }

    #[test]
    fn test_record_size() {
        let mut b = RecordBuilder::new(200);
        b.literal("1")
            .literal("01")
            .literal("0")
            .literal("080606")
            .literal("080606")
            .literal("080606")
            .numeric_str(4, "0005");
        b.char_field(15, "TEST").unwrap();
        b.numeric_str(3, "001");
        b.char_field(15, "").unwrap();
        b.numeric_u64(1, 1).numeric_str(7, "1234567");
        b.char_field(40, "").unwrap();
        b.spaces(93);
        assert_eq!(b.pos, 200);
    }
}
