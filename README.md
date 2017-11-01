# Simple Todo application in Rust

## build

`cargo build --release`

## usage
<pre>
Simple Todo 1.0.0

USAGE:
    todo [FLAGS] [OPTIONS]

FLAGS:
    -h, --help         Prints help information
    -m, --mark_done    Mark task as done
    -p, --print        Just print tasks
    -V, --version      Prints version information

OPTIONS:
    -d, --description <description>...    Description [default: ]
    -t, --task <task>                     The name of the task
</pre>

### generate changelog with clog

1. `cargo install clog-cli`
2. `clog -o CHANGELOG.md --setversion 1.0.0 -s "Initial Release"`

### changelog

See [CHANGELOG.md](CHANGELOG.md)