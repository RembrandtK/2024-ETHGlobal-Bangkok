use tracing::{subscriber::SetGlobalDefaultError, Level};

/// Initialize tracing with the given filter level.
pub fn start_tracing(filter: Level) -> Result<(), SetGlobalDefaultError> {
    tracing::subscriber::set_global_default(
        tracing_subscriber::fmt()
            .with_max_level(filter)
            .with_file(true)
            .with_line_number(true)
            .finish(),
    )
}
