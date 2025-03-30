/// This file contains the CLI errors for Rutes.
#[derive(Debug)]
pub enum RutesError {
    ExecutorError,
    IOError,
    SerializationError,
    UserCreationError,
    TaskError,
    ConfigurationError,
    PipelineError,
    PipelineSyntaxError,
}
