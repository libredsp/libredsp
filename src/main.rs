use clap::{CommandFactory, Parser, Subcommand, ValueEnum};
use clap_complete::{Shell, generate};
use std::io;

#[derive(Parser, Debug)]
#[command(name = "freedsp", version, about = "Digital signal processing toolkit")]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Design a digital filter
    FilterDesign {
        /// Filter type: lowpass, highpass, bandpass, or bandstop
        #[arg(short = 't', long)]
        r#type: FilterType,

        /// Filter design method: fir or iir
        #[arg(short = 'm', long)]
        method: FilterMethod,

        /// Cutoff frequency in Hz
        #[arg(short = 'c', long)]
        cutoff: f64,

        /// Sampling frequency in Hz
        #[arg(short = 's', long)]
        sample_rate: f64,

        /// Filter order
        #[arg(short = 'n', long)]
        order: usize,

        /// Output file
        #[arg(short = 'o', long)]
        output: String,
    },

    /// Remove background noise from an audio file
    NoiseRemove {
        /// Input WAV file
        input: String,

        /// Output WAV file
        #[arg(short = 'o', long)]
        output: String,

        /// Start of the noise-only interval in seconds
        #[arg(long, default_value_t = 0.0)]
        noise_start: f64,

        /// End of the noise-only interval in seconds
        #[arg(long)]
        noise_end: f64,
    },

    /// Generate shell completion script
    Completion {
        /// Shell to generate completions for
        #[arg(value_enum)]
        shell: Shell,
    },
}

#[derive(ValueEnum, Debug, Clone)]
enum FilterType {
    Lowpass,
    Highpass,
    Bandpass,
    Bandstop,
}

#[derive(ValueEnum, Debug, Clone)]
enum FilterMethod {
    Fir,
    Iir,
}

fn main() {
    let args = Args::parse();

    match args.command {
        Command::FilterDesign {
            r#type,
            method,
            cutoff,
            sample_rate,
            order,
            output,
        } => {
            println!("Type: {type:?}");
            println!("Method: {method:?}");
            println!("Cutoff: {cutoff} Hz");
            println!("Sample rate: {sample_rate} Hz");
            println!("Order: {order}");
            println!("Output: {output}");
        }

        Command::NoiseRemove {
            input,
            output,
            noise_start,
            noise_end,
        } => {
            println!("Input: {input}");
            println!("Output: {output}");
            println!("Noise interval: {noise_start} - {noise_end} seconds");
        }

        Command::Completion { shell } => {
            let mut command = Args::command();
            let mut stdout = io::stdout();

            generate(shell, &mut command, "freedsp", &mut stdout);
        }
    }
}
