use clap::{Parser, Subcommand, builder::Str};
use std::{fs::read_to_string, io};

enum Modality {
    Barbell,
    Dumbbell,
    Cable,
    Machine,
    Kettlebell,
    Bodyweight,
}

enum LengthTensionEmphasis {
    Midrange,
    Shortened,
    Lengthened,
}

enum LimbUsage {
    Bilateral,
    Unilateral,
    Alternating,
    IndependentBilateral,
}

struct Exercise {
    name: String,
    target_muscles: Vec<String>,
    modality: Modality,
    length_tension_emphasis: LengthTensionEmphasis,
    limb_usage: LimbUsage,
    joint_actions: Vec<String>,
}

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Find { exercise_name: String },
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Find { exercise_name } => {
            println!("You are looking for this exercise: {exercise_name}");
        }
    }
}

fn find_exercise(exercise_name: String) -> Exercise {}
