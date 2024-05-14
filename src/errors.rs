#[derive(Debug)]
pub enum TextAnalysisError {
    FileReadError,
    WordCountError,
    CommonWordError,
    ConcordanceError,
    QuitCommand,
}

pub type TextAnalysisResult<T> = Result<T, TextAnalysisError>;