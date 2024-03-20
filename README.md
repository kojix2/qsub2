# qsub2

qsub2 is a tool for easily submitting PBS jobs with a script template. It allows users to quickly generate PBS job scripts based on customizable templates and submit them using the `qsub` command.

## Installation

To use qsub2, you can download the source code and compile it using Rust's package manager Cargo. Make sure you have Rust and Cargo installed on your system.

```bash
git clone https://github.com/your-username/qsub2.git
cd qsub2
cargo build --release
```

## Usage

```
qsub2 [OPTIONS] <command> [files]...

Options:
    --version        Show version information
    --help           Show help information
    -n, --name <name>             Job name
    -@, --ncpus <ncpus>           CPU number [logical cpu number]
    -m, --mem <mem>               Memory [5gb]
    -q, --queue <queue>           Queue [batch]
    --walltime <walltime>         Walltime [30:00:00:00]
    --template <template>         Script template
    --outfile <outfile>           Output script

Arguments:
    <command>        Command to submit
    <files>...       Input files

Example:
    qsub2 -n my_job -@ 4 -m 10gb --template template.pbs --outfile job_script.sh echo "Hello, world!"
```

## Features

- Customizable job script template with placeholders for job name, CPU number, memory, queue, and more.
- Ability to specify input files and customizable options for job submission.
- Interactive command-line interface for generating and submitting PBS job scripts.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
