use ferrumc_utils::constants::DEFAULT_LOG_LEVEL;

/// Sets up the logger. Needs to be run before anything else in order for logging to run end.
pub fn setup_logger() {
    let trace_level = std::env::args()
        .find(|arg| arg.starts_with("--log="))
        .map(|arg| arg.replace("--log=", ""));

    let mut trace_level: &str = trace_level.as_deref().unwrap_or("");
    if trace_level.is_empty() {
        eprintln!(
            "No log level specified, using default: {}",
            DEFAULT_LOG_LEVEL
        );
        trace_level = DEFAULT_LOG_LEVEL;
    }

    let trace_level = match trace_level.trim().parse::<tracing::Level>() {
        Ok(level) => level,
        Err(_) => {
            eprintln!("Invalid log level: {}", trace_level);
            eprintln!("Possible values: trace, debug, info, warn, error");
            std::process::exit(1);
        }
    };

    let filter = tracing_subscriber::EnvFilter::from_default_env()
        .add_directive(trace_level.into())
        .add_directive("hyper=error".parse().unwrap());

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .init();
}
