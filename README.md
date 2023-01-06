MITM for command line io programs. specifically created for debugging language servers

## build
`cargo build --release`

## usage

`RUN_COMMAND=/usr/bin/run.sh INPUT_FILE=/tmp/input OUTPUT_FILE=/tmp/output target/debug/bucket`

input to program is captured in `/tmp/input`

output to program is captured in `/tmp/output`

### FAQ
1. if run.sh expects few command line args, create a new run2.sh with command args set
2. if env `INPUT_FILE` and `OUTPUT_FILE` can't be set, create new run3.sh with env set in run3.sh 
