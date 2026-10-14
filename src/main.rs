use clap::{Parser, Subcommand};
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

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
            find_exercise(&exercise_name);
        }
    }
}

// fn find_exercise(exercise_name: String) -> Exercise {}
fn find_exercise(exercise_name: &str) {
    match find_line("exercises.txt", exercise_name) {
        Ok(Some(line_number)) => {
            println!("The exercise you are looking for is on line {line_number}");
        }
        Ok(None) => {
            println!("Exercise not found.");
        }
        Err(error) => {
            eprintln!("Error finding exercise: {error}");
        }
    }
}

fn find_line(path: &str, needle: &str) -> io::Result<Option<usize>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let line = line?;

        let (before, _) = line.split_once('|').ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "{path}: line {:?} is missing the required '|' delimiter",
                    index.checked_add(1)
                ),
            )
        })?;

        if before.contains(needle) {
            return Ok(index.checked_add(1));
        }
    }

    Ok(None)
}
