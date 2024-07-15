use env_logger;

// Function to set up logging for the application.
pub fn logging_setup() {
    // Set the environment variable for the log level.
    std::env::set_var("RUST_LOG", "debug");

    // Initialize the logger.
    env_logger::init();
}
