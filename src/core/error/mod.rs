#[derive(Debug)]
pub enum RutesError {
    ExecutorError,
    UserInitializationError,
    TaskError,
    ConfigurationError,
    PipelineError,
}
