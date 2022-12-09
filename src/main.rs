use std::error::Error;

use tokio::fs::File;
use tokio::io::AsyncBufReadExt;
use tokio::io::AsyncWriteExt;

async fn input_half(
    mut child_stdin: tokio::process::ChildStdin,
    mut input_file: File,
) -> Result<(), Box<dyn Error>> {
    let stdin = tokio::io::stdin();
    let mut stdin = tokio::io::BufReader::new(stdin);
    loop {
        let mut line = String::new();
        stdin.read_line(&mut line).await?;
        input_file.write_all(&line.as_bytes()).await?;
        child_stdin.write_all(&line.as_bytes()).await?;
    }
}

async fn output_half(
    child_stdout: tokio::process::ChildStdout,
    mut output_file: File,
) -> Result<(), Box<dyn Error>> {
    let mut stdout = tokio::io::stdout();
    let mut child_stdout = tokio::io::BufReader::new(child_stdout);
    loop {
        let mut line = String::new();
        stdout.write_all(&line.as_bytes()).await?;
        child_stdout.read_line(&mut line).await?;
        output_file.write_all(&line.as_bytes()).await?;
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let command = std::env::var("RUN_COMMAND")?;
    let input_file = std::env::var("INPUT_FILE")?;
    let output_file = std::env::var("OUTPUT_FILE")?;

    let command = tokio::process::Command::new(command)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()?;
    let output_file = tokio::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(output_file)
        .await?;
    let input_file = tokio::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(input_file)
        .await?;
    let child_stdout = command.stdout.unwrap();
    let child_stdin = command.stdin.unwrap();
    let _ = tokio::join!(
        tokio::spawn(async move { input_half(child_stdin, input_file).await.unwrap_or(()) }),
        tokio::spawn(async move { output_half(child_stdout, output_file).await.unwrap_or(()) }),
    );
    Ok(())
}
