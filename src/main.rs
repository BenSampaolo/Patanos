use clap::Parser;
use std::env;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Seek, SeekFrom, Write};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, num_args = 0..=1, default_missing_value = "? ")]
    note: Option<String>,

    #[arg(short, long, num_args = 0..=1, default_missing_value = "Todo! ")]
    task: Option<String>,

    #[arg(long)]
    path: Option<String>,

    #[arg(short, long, num_args = 0..=1, default_missing_value = "All")]
    list: Option<String>,

    #[arg(short, long)]
    remove: Option<String>,
}

fn main() {
    let args = Args::parse();

    let remembered_path = match args.path {
        Some(path_string) => PathBuf::from(path_string),
        None => env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
    };

    let remembered_path_text = remembered_path.to_string_lossy();
    let mut save_path = dirs::data_dir().expect("Could not find save directory! ");
    save_path.push("patanos");

    std::fs::create_dir_all(&save_path).expect("Could not create directory structure");

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
    let remove_text = args.remove.as_deref().unwrap_or("");

    if !note_text.is_empty() || !task_text.is_empty() {
        write_reminder_to_file(note_text, task_text, &remembered_path_text, &mut save_file);
    }
    if !remove_text.is_empty() {
        if let Ok(num) = remove_text.parse::<usize>() {
            remove_reminder_from_file(&mut save_file, num);
        } else {
            println!("Invalid input for --remove");
        }
    }
    if !list_text.is_empty() {
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

    let entry = format!("\"{path}\": {note}{task}");
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
        println!("{}. Reminder at {}", i + 1, line);
    }
}

fn remove_reminder_from_file(file: &mut File, idx_to_remove: usize) {
    let reader = BufReader::new(file.try_clone().expect("Failed to clone file handle"));
    let mut lines: Vec<String> = reader.lines().map_while(Result::ok).collect();

    lines.sort();

    if idx_to_remove > lines.len() || idx_to_remove == 0 {
        println!("Error: Index {} out of range!", idx_to_remove);
        return;
    }

    lines.remove(idx_to_remove - 1);

    file.set_len(0).expect("Failed to truncate file");
    file.seek(SeekFrom::Start(0))
        .expect("Failed to seek to start");

    for line in lines {
        if let Err(e) = writeln!(file, "{}", line) {
            eprintln!("Failed to write back to file: {e}");
        }
    }
}
