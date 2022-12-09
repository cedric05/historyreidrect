create proxy input file and proxy output file for stdin/stdout softwares. This is used specifically for debuggin language servers

## build
`cargo build --release`

## usage

`RUN_COMMAND=/usr/bin/run.sh INPUT_FILE=/tmp/input OUTPUT_FILE=/tmp/output target/debug/bucket`

check `/tmp/input` and `/tmp/output`

### FAQ
1. if run.sh expects few command line args, create a new run2.sh with command args set
2. if env `INPUT_FILE` and `OUTPUT_FILE` can't be set, create new run3.sh with env set in run3.sh 

