use std::io;
use std::process::Command;

const GIT_HASH_ENV_VAR: &str = "GIT_SHA_SHORT";

fn main() {
    match get_git_commit_hash() {
        // If the git commit hash is retrieved successfully, set the environment variable
        Ok(git_hash) => {
            println!("cargo:rustc-env={GIT_HASH_ENV_VAR}={git_hash}");
        }
        // If there's an error retrieving the git commit hash, print a note and set the environment variable to "unknown"
        Err(e) => {
            println!("cargo:warning=Note: Failed to get git commit hash: {}", e);
            println!("cargo:rustc-env={GIT_HASH_ENV_VAR}=unknown");
        }
    }
}

fn get_git_commit_hash() -> Result<String, io::Error> {
    // Build the command to retrieve the short git commit hash
    let output = Command::new("git")
        .arg("rev-parse")
        .arg("--short")
        .arg("HEAD")
        .output()?;

    if output.status.success() {
        // Parse the raw output into a string and trim the newline character
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        // Return an error if the command failed
        Err(io::Error::new(io::ErrorKind::Other, "Git command failed"))
    }
}
