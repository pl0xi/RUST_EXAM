#[derive(Debug)]
pub enum TextAnalysisError {
    FileReadError,
    WordCountError,
    CommonWordError,
    ConcordanceError,
    QuitCommand,
}

type TextAnalysisResult<T> = Result<T, TextAnalysisError>;