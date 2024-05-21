#[derive(Debug)]
pub enum TextAnalysisError {
    FileReadError,
    WordCountError,
    CommonWordError,
    ConcordanceError,
    FailedToJoinThreadError,
    QuitCommand,
}