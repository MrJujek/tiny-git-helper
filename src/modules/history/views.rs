use std::thread;

use crossterm::{
    cursor::{MoveLeft, MoveUp},
    terminal::{self, disable_raw_mode, enable_raw_mode},
};

use crate::utils::out::{format_color, format_dim};

use super::CommitHistoryOptions;

#[derive(Debug)]
struct Commit {
    hash: String,
    message: String,
    date: String,
    author: String,
}

pub fn commit_history(options: CommitHistoryOptions) {
    use std::process::Command;

    let limit = match options.limit {
        Some(limit) => limit.to_string(),
        None => "10".into(),
    };

    let author = match options.author {
        Some(author) => author,
        None => "".into(),
    };

    let mut branch = match options.branch {
        Some(branch) => branch,
        None => "".into(),
    };

    let file = match options.file {
        Some(file) => file,
        None => "".into(),
    };

    let all = options.all;

    if branch.is_empty() && !all {
        branch = super::functions::get_current_branch();
    }

    print!("\nShowing commits");
    if !file.is_empty() {
        print!(
            " for {}",
            format_color(file.as_str(), crate::utils::out::Color::Green)
        );
    }
    if !author.is_empty() {
        print!(
            " made by {}",
            format_color(author.as_str(), crate::utils::out::Color::Blue)
        );
    }
    println!(
        " on {}",
        format_color(branch.as_str(), crate::utils::out::Color::Yellow)
    );

    let mut binding = Command::new("git");
    let command = binding
        .arg("log")
        .arg("--oneline")
        .arg("--decorate")
        .arg("--color")
        .arg("--pretty=format:%h-_-%s-_-%cr-_-%an")
        .arg("--full-history")
        .arg(format!("-{}", limit))
        .arg(format!("--author={}", author))
        .arg(format!("{}", branch));

    if !file.is_empty() {
        command.arg(format!("{}", file));
    }

    let output = command.output().expect("Failed to execute git log");

    let out = String::from_utf8(output.stdout).unwrap();

    let commits: Vec<Commit> = out
        .lines()
        .map(|line| {
            let mut parts = line.split("-_-");
            Commit {
                hash: parts.next().unwrap().into(),
                message: parts.next().unwrap().into(),
                date: parts.next().unwrap().into(),
                author: parts.next().unwrap().into(),
            }
        })
        .collect();

    if commits.len() == 0 {
        crate::out::print_error("\nNo commits found\n");
        return;
    }

    let window_size = 5;

    let mut index = 0;
    let max_index = commits.len() - 1 - window_size;

    for i in index..index + window_size {
        if i < commits.len() {
            let commit = &commits[i];
            let hash = format_dim(format!("({})", commit.hash).as_str());
            let message = commit.message.as_str();
            let date = format_color(commit.date.as_str(), crate::utils::out::Color::Green);
            let author = format_color(commit.author.as_str(), crate::utils::out::Color::Blue);

            println!("{} - {} ({}) ~ {}", hash, message, date, author);
        }
    }

    enable_raw_mode().unwrap();

    loop {
        match crossterm::event::read().unwrap() {
            crossterm::event::Event::Key(event) => {
                if event.code == crossterm::event::KeyCode::Down {
                    if index < max_index {
                        index += 1;
                    }

                    render_commits(&commits, index, window_size);
                } else if event.code == crossterm::event::KeyCode::Up {
                    if index > 0 {
                        index -= 1;
                    }

                    render_commits(&commits, index, window_size);
                } else if event.code == crossterm::event::KeyCode::Char('q') {
                    break;
                }
            }
            _ => {}
        }
    }

    disable_raw_mode().unwrap();
}

fn render_commits(commits: &Vec<Commit>, index: usize, window_size: usize) {
    use crossterm::execute;
    use std::io::{stdout, Write};

    enable_raw_mode().unwrap();

    let mut stdout = stdout();

    let _ = execute!(stdout, MoveUp(window_size as u16));
    let _ = execute!(stdout, terminal::Clear(terminal::ClearType::FromCursorDown));

    for i in index..index + window_size {
        if i < commits.len() {
            let commit = &commits[i];
            let hash = format_dim(format!("({})", commit.hash).as_str());
            let message = commit.message.as_str();
            let date = format_color(commit.date.as_str(), crate::utils::out::Color::Green);
            let author = format_color(commit.author.as_str(), crate::utils::out::Color::Blue);

            execute!(stdout, MoveLeft(1000)).unwrap();
            writeln!(stdout, "{} - {} ({}) ~ {}", hash, message, date, author).unwrap();
        }
    }
    execute!(stdout, MoveLeft(1000)).unwrap();
    stdout.flush().unwrap();
}
