// Import necessary modules from external crates and the standard library.
use clap::Parser; // For parsing command-line arguments.
use std::fs::{self, File}; // For file system operations like reading and writing files.
use std::io::Write; // To enable writing to files.
use std::path::PathBuf; // For handling file paths in a platform-independent way.
use std::process::Command; // To execute external commands (like `qsub`).
use chrono; // For generating timestamps.

// Define a struct to hold command-line arguments using the clap crate.
#[derive(Parser)]
#[command(version = "1.0", about = "Easily submitting PBS jobs with script template.", long_about = None)]
struct Cli {
    /// Command to submit
    command: String,

    /// Input files
    #[arg(required = false)]
    files: Vec<PathBuf>,

    /// Job name
    #[arg(short, long)]
    name: Option<String>,

    /// CPU number [logical cpu number]
    #[arg(short = '@', long)]
    ncpus: Option<u32>,

    /// Memory [5gb]
    #[arg(short = 'm', long)]
    mem: Option<String>,

    /// Queue [batch]
    #[arg(short, long)]
    queue: Option<String>,

    /// Walltime [30:00:00:00]
    #[arg(short, long)]
    walltime: Option<String>,

    /// Script template
    #[arg(short, long)]
    template: Option<PathBuf>,

    /// Output script
    #[arg(short, long)]
    outfile: Option<PathBuf>,

    /// Submit the job
    #[arg(short, long)]
    submit: bool,
}

// Function to generate the job script file based on command-line arguments and/or defaults.
fn generate_job_script(cli: &Cli) -> std::io::Result<()> {
    // If a template path is provided, read it as the template content; otherwise, use a default template.
    let template_content = if let Some(template_path) = &cli.template {
        fs::read_to_string(template_path)?
    } else {
        // Default job script template with placeholders.
r#"#!/bin/bash
#PBS -N {name}
#PBS -l select=1:ncpus={ncpus}:mem={mem}
#PBS -q {queue}
#PBS -l walltime={walltime}

cd $PBS_O_WORKDIR

{command}
"#
        .to_string()
    };

    // Replace placeholders in the template with actual values from the command-line arguments or their defaults.
    let job_script = template_content
        .replace("{name}", &cli.name.as_deref().unwrap_or("job"))
        .replace(
            "{ncpus}",
            &cli.ncpus.map_or_else(|| "1".to_string(), |n| n.to_string()),
        )
        .replace("{mem}", cli.mem.as_deref().unwrap_or("5gb"))
        .replace("{queue}", cli.queue.as_deref().unwrap_or("batch"))
        .replace(
            "{walltime}",
            cli.walltime.as_deref().unwrap_or("30:00:00:00"),
        )
        .replace("{command}", &cli.command);

    // Determine the output file path, defaulting to "job_script.sh" if not specified.
    let timestamp = chrono::Local::now().format("%Y%m%d%H%M%S").to_string();
    let default_output_path = PathBuf::from(format!("job_script_{}.sh", timestamp));
    let output_path = cli.outfile.as_ref().unwrap_or(&default_output_path);
    let mut file = File::create(output_path)?;
    file.write_all(job_script.as_bytes())?;

    Ok(())
}

// Function to submit the job script using the `qsub` command.
fn submit_job(outfile: &PathBuf) -> std::io::Result<()> {
    let status = Command::new("qsub")
        .arg(outfile.to_str().unwrap())
        .status()?;

    println!("Job submitted with status: {}", status);

    Ok(())
}

// The main function where execution starts.
fn main() {
    // Parse command-line arguments into the Cli struct.
    let cli = Cli::parse();

    // Generate the job script. If an error occurs, print it and exit.
    if let Err(e) = generate_job_script(&cli) {
        eprintln!("Error generating job script: {}", e);
        return;
    }

    // If the --submit option is set, and an output file was specified, submit the job. If an error occurs, print it.
    if cli.submit && cli.outfile.is_some() {
        if let Err(e) = submit_job(cli.outfile.as_ref().unwrap()) {
            eprintln!("Error submitting job: {}", e);
        }
    } else if cli.submit {
        eprintln!("Error: Output file not specified. Job submission aborted.");
    }
}

// Module for unit tests.
#[cfg(test)]
mod tests {
    use super::*;

    // A test function to verify the job script generation with default parameters.
    #[test]
    fn test_generate_job_script_with_defaults() {
        // Define test inputs.
        let cli = Cli {
            command: "echo Hello, world!".to_string(),
            files: vec![],
            name: None,
            ncpus: None,
            mem: None,
            queue: None,
            walltime: None,
            template: None,
            outfile: Some(PathBuf::from("test_output.sh")),
            submit: false,
        };

        // Generate a job script based on the test inputs.
        let result = generate_job_script(&cli);
        // Assert that the job script generation succeeded without errors.
        assert!(result.is_ok());

        // Define the expected content of the generated job script using default values.
        let expected_content = "#!/bin/bash\n#PBS -N job\n#PBS -l select=1:ncpus=1:mem=5gb\n#PBS -q batch\n#PBS -l walltime=30:00:00:00\n\ncd $PBS_O_WORKDIR\n\necho Hello, world!\n";
        // Read the generated job script file.
        let generated_content =
            fs::read_to_string("test_output.sh").expect("Failed to read output file");
        // Assert that the generated content matches the expected content.
        assert_eq!(generated_content, expected_content);

        // Clean up by removing the generated file after the test.
        fs::remove_file("test_output.sh").expect("Failed to clean up output file");
    }
}
