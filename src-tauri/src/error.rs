use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("ファイルエラー: {0}")]
    Io(#[from] std::io::Error),
    #[error("エンコードエラー: {0}")]
    Encoding(String),
    #[error("日付エラー: {0}")]
    Date(String),
}

impl serde::Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
