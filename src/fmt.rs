use std::io::Write;
use std::process::{Command, Stdio};

pub fn rustfmt(input: String) -> anyhow::Result<String> {
    let mut command = Command::new("rustfmt")
        .arg("--emit")
        .arg("stdout")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    let stdin = command.stdin.as_mut().unwrap();
    stdin.write_all(input.as_bytes())?;
    drop(stdin);

    let output = command.wait_with_output()?;
    let stdout = output.stdout;
    let stdout = String::from_utf8(stdout)?;
    Ok(stdout)
}