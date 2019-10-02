pub mod cache;
pub mod environment;
pub mod init_db;
pub mod repository;
pub mod setup;
pub mod sha1_file;
pub mod usage;

/// Helper function that takes anything that can be converted to a String, prints it out, and
/// then exits the process with the error code 2 (an arbitrary choice).
pub fn fatal(msg: String) {
    println!("FATAL: {}", msg);
    std::process::exit(2);
}
