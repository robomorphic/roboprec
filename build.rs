use std::path::Path;
use std::process::{Command};

fn log(message: &str) {
    println!("cargo::warning={message}");
}

fn main() {
    // 1. Define paths
    // TODO: add support for different project folders
    let scala_project_dir = Path::new("daisy");

    // 2. Tell Cargo when to re-run this script.
    // Re-run if any Scala source files or the build.sbt changes.
    println!("cargo:rerun-if-changed=daisy/");

    // 3. Check if sbt is installed
    if Command::new("sbt").arg("--version").output().is_err() {
        log("cargo::warning=sbt not found, installing sbt and dependencies...");
        
        // Update package list
        log("Running: sudo apt-get update");
        Command::new("sudo")
            .args(&["apt-get", "update"])
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit())
            .status()
            .expect("Failed to run apt-get update");
        
        // Install curl if not present
        log("Running: sudo apt-get install -y curl");
        Command::new("sudo")
            .args(&["apt-get", "install", "-y", "curl"])
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit())
            .status()
            .expect("Failed to install curl");
        
        // Add SBT repository
        log("Running: Adding SBT repository to apt sources");
        Command::new("sh")
            .arg("-c")
            .arg("echo 'deb https://repo.scala-sbt.org/scalasbt/debian all main' | sudo tee /etc/apt/sources.list.d/sbt.list")
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit())
            .status()
            .expect("Failed to add sbt repository");
        
        log("Running: Adding SBT old repository to apt sources");
        Command::new("sh")
            .arg("-c")
            .arg("echo 'deb https://repo.scala-sbt.org/scalasbt/debian /' | sudo tee /etc/apt/sources.list.d/sbt_old.list")
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit())
            .status()
            .expect("Failed to add sbt old repository");
        
        // Add SBT GPG key
        log("Running: Adding SBT GPG key");
        Command::new("sh")
            .arg("-c")
            .arg("curl -sL 'https://keyserver.ubuntu.com/pks/lookup?op=get&search=0x2EE0EA64E40A89B84B2DF73499E82A75642AC823' | sudo tee /etc/apt/trusted.gpg.d/sbt.asc")
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit())
            .status()
            .expect("Failed to add sbt GPG key");
        
        // Update package list again
        log("Running: sudo apt-get update");
        Command::new("sudo")
            .args(&["apt-get", "update"])
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit())
            .status()
            .expect("Failed to run apt-get update");
        
        // Install Java 8
        log("Running: sudo apt-get install -y openjdk-8-jdk");
        Command::new("sudo")
            .args(&["apt-get", "install", "-y", "openjdk-8-jdk"])
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit())
            .status()
            .expect("Failed to install Java 8");
        
        // Install SBT
        log("Running: sudo apt-get install -y sbt");
        Command::new("sudo")
            .args(&["apt-get", "install", "-y", "sbt"])
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit())
            .status()
            .expect("Failed to install sbt");
        
        log("cargo::warning=sbt and dependencies installed successfully");
    }

    // 4. Build Daisy project
    log("cargo::warning=Building Daisy Scala project...");
    
    log("Running: sbt update (in daisy directory)");
    let sbt_update = Command::new("sbt")
        .current_dir(scala_project_dir)
        .arg("update")
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .status()
        .expect("Failed to run 'sbt update' in daisy directory");
    
    if !sbt_update.success() {
        panic!("'sbt update' failed in daisy directory");
    }

    log("Running: sbt compile (in daisy directory)");
    let sbt_compile = Command::new("sbt")
        .current_dir(scala_project_dir)
        .arg("compile")
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .status()
        .expect("Failed to run 'sbt compile' in daisy directory");
    
    if !sbt_compile.success() {
        panic!("'sbt compile' failed in daisy directory");
    }

    log("Running: sbt script (in daisy directory)");
    let sbt_script = Command::new("sbt")
        .current_dir(scala_project_dir)
        .arg("script")
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .status()
        .expect("Failed to run 'sbt script' in daisy directory");
    
    if !sbt_script.success() {
        panic!("'sbt script' failed in daisy directory");
    }

    log("cargo::warning=Daisy build completed successfully");

}


