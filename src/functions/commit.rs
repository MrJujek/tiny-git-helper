pub fn is_valid_commit() {
    use std::process;

    if !super::is_git_repo() {
        crate::out::print_error("Not a git repository");
        process::exit(0);
    }

    if !are_files_to_commit() {
        crate::out::print_error("No files to commit");
        process::exit(0);
    }
}

fn are_files_to_commit() -> bool {
    use std::process::Command;

    let output = Command::new("git")
        .arg("status")
        .arg("-s")
        .output()
        .unwrap();

    if output.stdout.len() == 0 {
        return false;
    }

    true
}

pub fn is_top_level() -> bool {
    use std::process::Command;

    let output = Command::new("git")
        .arg("rev-parse")
        .arg("--show-toplevel")
        .output()
        .unwrap();

    if !output.status.success() {
        return false;
    }

    let out = String::from_utf8(output.stdout).unwrap();
    let out = out.trim();
    let current_dir = std::env::current_dir().unwrap();

    if out != current_dir.to_str().unwrap() {
        return false;
    }

    true
}
