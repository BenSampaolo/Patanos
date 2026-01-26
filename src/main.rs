use clap::Parser;
use std::env;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Seek, SeekFrom, Write};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long)]
    patanos: String,

    #[arg(short, long)]
    note: Option<String>,

    #[arg(short, long)]
    task: Option<String>,

    #[arg(long)]
    path: Option<String>,

    #[arg(long)]
    list: Option<String>,
}

fn main() {
    let args = Args::parse();

    let remembered_path = match args.path {
        Some(path_string) => PathBuf::from(path_string),
        None => env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
    };

    let remembered_path_text = remembered_path.to_string_lossy();
    let mut save_path = PathBuf::from(args.patanos);
    save_path.push("Reminders.txt");

    let mut save_file = OpenOptions::new()
        .read(true)
        .append(true)
        .create(true)
        .open(&save_path)
        .expect("Could not open or create savefile");

    let note_text = args.note.as_deref().unwrap_or("");
    let task_text = args.task.as_deref().unwrap_or("");
    let list_text = args.list.as_deref().unwrap_or("");

    if list_text.is_empty() {
        write_reminder_to_file(note_text, task_text, &remembered_path_text, &mut save_file);
    } else {
        save_file
            .seek(SeekFrom::Start(0))
            .expect("Failed to seek file");

        if list_text == "All" {
            read_reminder_of_file(&mut save_file, -1);
        } else if let Ok(num) = list_text.parse::<i32>() {
            read_reminder_of_file(&mut save_file, num);
        } else {
            println!("Invalid input for `--list` argument!");
        }
    }
}

fn write_reminder_to_file(note: &str, task: &str, path: &str, file: &mut File) {
    if note.is_empty() && task.is_empty() {
        return;
    }

    let entry = format!("Reminder at \"{path}\": {note}{task}");
    if let Err(e) = writeln!(file, "{}", entry) {
        eprintln!("Could not write to savefile: {e}");
    }
}

fn read_reminder_of_file(file: &mut File, lines_to_read: i32) {
    let reader = BufReader::new(file);
    let mut lines: Vec<String> = reader.lines().map_while(Result::ok).collect();

    lines.sort();

    let count = if lines_to_read < 0 {
        lines.len()
    } else {
        lines_to_read as usize
    };

    for (i, line) in lines.iter().take(count).enumerate() {
        println!("{} {}", i + 1, line);
    }
}
