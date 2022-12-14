use std::error::Error;
use std::process::Stdio;

use tokio::fs::OpenOptions;
use tokio::io::stdin;
use tokio::io::stdout;
use tokio::io::AsyncBufReadExt;
use tokio::io::AsyncWrite;
use tokio::io::AsyncWriteExt;
use tokio::io::BufReader;
use tokio::join;
use tokio::process::Command;

async fn write_to_both<S1: AsyncWrite + Unpin, S2: AsyncWrite + Unpin>(
    s1: &mut S1,
    s2: &mut S2,
    bytes: &[u8],
) -> Result<(), Box<dyn Error>> {
    let (res1, res2) = join!(s1.write_all(&bytes), s2.write_all(&bytes));
    res1?;
    res2?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let command = std::env::var("RUN_COMMAND").expect("set command to run in `RUN_COMMAND`");
    let input_file = std::env::var("INPUT_FILE").unwrap_or("input".to_string());
    let output_file = std::env::var("OUTPUT_FILE").unwrap_or("output".to_string());

    let child = Command::new(command)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;
    let mut child_stdout = BufReader::new(child.stdout.unwrap());
    let mut child_stdin = child.stdin.unwrap();

    let mut open_options = OpenOptions::new();
    open_options.append(true).create(true);

    let mut output_file = open_options.clone().open(output_file).await?;
    let mut input_file = open_options.open(input_file).await?;

    let mut current_stdin = BufReader::new(stdin());
    let mut current_stdout = stdout();

    let mut child_stdout_line = String::new();
    let mut stdin_line = String::new();

    loop {
        tokio::select! {
            out = child_stdout.read_line(&mut child_stdout_line)=>{
                out?;
                write_to_both(&mut current_stdout, &mut output_file, child_stdout_line.as_bytes()).await?;
                child_stdout_line.clear();
            }
            out = current_stdin.read_line(&mut stdin_line)=>{
                out?;
                write_to_both(&mut input_file, &mut child_stdin, stdin_line.as_bytes()).await?;
                stdin_line.clear();
            }

        }
    }
}
