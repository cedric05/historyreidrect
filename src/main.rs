use std::error::Error;
use std::process::Stdio;

use tokio::fs::OpenOptions;
use tokio::io::stdin;
use tokio::io::stdout;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::process::Command;

macro_rules! copy {
    ($input:expr, $output:expr, $output2:expr) => {
        async {
            let mut buf = [0; 4096];
            loop {
                let count = $input.read(&mut buf).await?;
                let res = tokio::join!(
                    $output.write_all(&buf[0..count]),
                    $output2.write_all(&buf[0..count])
                );
                res.0?;
                res.1?;
            }
        }
    };
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    let command = std::env::var("RUN_COMMAND").expect("set command to run in `RUN_COMMAND`");
    let input_file = std::env::var("INPUT_FILE").unwrap_or("input".to_string());
    let output_file = std::env::var("OUTPUT_FILE").unwrap_or("output".to_string());

    let child = Command::new(command)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;
    let mut child_stdout = child.stdout.expect("impossible for this to happen");
    let mut child_stdin = child.stdin.expect("impossible for this to happen");

    let mut open_options = OpenOptions::new();
    open_options.append(true).create(true);

    let mut output_file = open_options.clone().open(output_file).await?;
    let mut input_file = open_options.open(input_file).await?;

    let mut current_stdin = stdin();
    let mut current_stdout = stdout();

    tokio::select! {
        r = copy!(current_stdin, child_stdin, input_file)=> r,
        r = copy!(child_stdout, current_stdout, output_file)=> r,
    }
}
