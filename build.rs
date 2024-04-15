use std::process::Command;

const GIT_HASH_ENV_VAR: &str = "GIT_SHA_SHORT";
fn main() {
    //Build command to retrieve the short git commit hash
    let git_process_output = Command::new("git")
        .arg("rev-parse")
        .arg("--short")
        .arg("HEAD")
        .output()
        .expect("Failed to retrieve short git commit hash");

    //Parse the raw output into a string, we still need to remove the newline character
    let git_hash_raw =
        String::from_utf8(git_process_output.stdout).expect("Failed to convert git hash to string");
    //Remove the newline character from the short git commit hash
    let git_hash = git_hash_raw.trim_end_matches('\n');

    println!("cargo:rustc-env={}={}", GIT_HASH_ENV_VAR, git_hash);
}
