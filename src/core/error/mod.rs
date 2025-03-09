#[derive(Debug)]
pub enum RutesError {
    ExecutorError,
    IOError,
    SerializationError,
    UserCreationError,
    TaskError,
    ConfigurationError,
    PipelineError,
}
