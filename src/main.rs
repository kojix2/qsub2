use chrono::Local;
use clap::Parser;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

#[derive(Parser)]
#[command(version, about = "Easily submitting PBS jobs with script template.")]
struct Cli {
    #[arg(required = false)]
    /// Command to submit
    command: String,

    #[arg(required = false)]
    /// Input files
    files: Vec<PathBuf>,

    #[arg(short, long)]
    /// Job name
    name: Option<String>,

    #[arg(short = '@', long)]
    /// CPU number [logical cpu number]
    ncpus: Option<u32>,

    #[arg(short = 'm', long)]
    /// Memory [5gb]
    mem: Option<String>,

    #[arg(short, long)]
    /// Queue [batch]
    queue: Option<String>,

    #[arg(short, long)]
    /// Walltime [30:00:00:00]
    walltime: Option<String>,

    #[arg(short, long)]
    /// Script template
    template: Option<PathBuf>,

    #[arg(short = 'o', long)]
    /// Output script
    outfile: Option<PathBuf>,

    #[arg(short, long)]
    /// Submit the job
    submit: bool,
}

fn generate_job_script(cli: &Cli) -> std::io::Result<()> {
    let template_content = if let Some(ref template_path) = cli.template {
        fs::read_to_string(template_path)?
    } else {
        include_str!("../templates/default_template.sh").into() // Use a built-in default template as a fallback
    };

    let job_script = template_content
        .replace("{name}", &cli.name.as_deref().unwrap_or("job"))
        .replace("{ncpus}", &format!(":ncpus={}", cli.ncpus.unwrap_or(1)))
        .replace(
            "{mem}",
            &cli.mem
                .as_deref()
                .map_or(String::new(), |m| format!(":mem={}", m)),
        )
        .replace("{queue}", cli.queue.as_deref().unwrap_or("batch"))
        .replace(
            "{walltime}",
            cli.walltime.as_deref().unwrap_or("30:00:00:00"),
        )
        .replace("{command}", &cli.command);

    let output_file_name: PathBuf = cli.outfile.clone().unwrap_or_else(|| {
        PathBuf::from(format!(
            "job_script_{}.sh",
            Local::now().format("%Y%m%d%H%M%S").to_string()
        ))
    });
    let mut file = File::create(&output_file_name)?;
    file.write_all(job_script.as_bytes())?;

    println!("Job script generated and saved to: {:?}", output_file_name);

    Ok(())
}

fn submit_job(outfile: &PathBuf) -> std::io::Result<()> {
    let status = Command::new("qsub").arg(outfile.as_path()).status()?;
    println!("Job submitted with status: {}", status);
    Ok(())
}

fn main() {
    let cli = Cli::parse();

    if let Err(e) = generate_job_script(&cli) {
        eprintln!("Error generating job script: {}", e);
        return;
    }

    if cli.submit {
        if let Some(ref outfile) = cli.outfile {
            if let Err(e) = submit_job(outfile) {
                eprintln!("Error submitting job: {}", e);
            }
        } else {
            eprintln!("Error: Output file not specified. Job submission aborted.");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_job_script_with_defaults() {
        let cli = Cli {
            command: "echo Hello, world!".into(),
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

        let result = generate_job_script(&cli);
        assert!(result.is_ok());

        let expected_content = include_str!("../test/fixtures/expected_default_script.sh"); // Assume this contains the expected default script
        let generated_content = fs::read_to_string("test_output.sh").unwrap();
        assert_eq!(generated_content, expected_content);

        fs::remove_file("test_output.sh").unwrap();
    }
}
