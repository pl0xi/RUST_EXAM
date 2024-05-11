pub enum TextAnalysisError {
    FileReadError,
    WordCountError,
    CommonWordError,
    ConcordanceError,
}

type TextAnalysisResult<T> = Result<T, TextAnalysisError>;