#[derive(Debug)]
pub enum TextAnalysisError {
    FileReadError,
    WordCountError,
    CommonWordError,
    ConcordanceError,
    FailedToJoinThreadError,
    QuitCommand,
}

pub type TextAnalysisResult<T> = Result<T, TextAnalysisError>;