use std::{
    io::Write,
    process::{Command, Output},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Uncomment this line to build the UI in ./ui using `npm`
    // build_todo_ui()?;
    Ok(())
}

#[allow(unused)]
fn build_todo_ui() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=./ui/src");
    println!("cargo:rerun-if-changed=./ui/public");
    println!("cargo:rerun-if-changed=./ui/package.json");

    // npm install and build static assets
    std::env::set_current_dir("./ui")?;
    handle_output(
        Command::new("npm").args(["install"]).output()?,
        "npm install",
    )?;
    handle_output(
        Command::new("npm").args(["run", "build"]).output()?,
        "npm run build",
    )?;

    // Move static assets up
    let _ = std::fs::remove_dir_all("../dist");
    std::fs::rename("./dist", "../dist")?;

    Ok(())
}

fn handle_output(
    output: Output,
    command_description: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    if !output.status.success() {
        let mut stderr = std::io::stderr();
        stderr
            .write_all(b"Error when running npm install.\nStdout:\n\n")
            .unwrap();
        stderr.write_all(&output.stdout).unwrap();
        stderr.write_all(b"\n\nStderr:\n\n").unwrap();
        stderr.write_all(&output.stderr).unwrap();
        return Err(format!(
            "Unable to run {}. See stdout and stderr output above",
            command_description
        )
        .into());
    }
    Ok(())
}
