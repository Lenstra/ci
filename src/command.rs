use std::error::Error;
use std::process::Command;

pub fn run(
    cmd: &str,
    args: Vec<String>,
    workdir: &str,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut binding = Command::new(cmd);
    let command = binding
        .current_dir(workdir)
        .args(args)
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit());

    let mut child = command.spawn()?;

    let status = child.wait()?;
    if !status.success() {
        return Err(format!("{} exited with status {}", cmd, status).into());
    }

    Ok(())
}
