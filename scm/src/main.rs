use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

const SCM_DIR: &str = ".scm";
const COMMITS: &str = ".scm/commits";
const HEAD_FILE: &str = ".scm/HEAD";

fn ensure_repo() {
    if !Path::new(SCM_DIR).exists() {
        fs::create_dir(SCM_DIR).unwrap();
        fs::create_dir(COMMITS).unwrap();
        fs::write(HEAD_FILE, "0").unwrap();
    }
}

fn next_commit_id() -> u64 {
    let head = fs::read_to_string(HEAD_FILE).unwrap();
    head.trim().parse::<u64>().unwrap() + 1
}

fn update_head(id: u64) {
    fs::write(HEAD_FILE, id.to_string()).unwrap();
}

fn list_working_files() -> Vec<PathBuf> {
    let mut files = Vec::new();
    for entry in WalkDir::new(".") {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_dir() {
            if path.ends_with(SCM_DIR) {
                continue;
            }
            continue;
        }

        if path.starts_with("./target") {
            continue;
        }

        files.push(path.to_path_buf());
    }
    files
}

fn commit() {
    ensure_repo();

    let id = next_commit_id();
    let commit_dir = format!("{}/{}", COMMITS, format!("{:05}", id));

    fs::create_dir(&commit_dir).unwrap();

    for file in list_working_files() {
        let rel = file.strip_prefix(".").unwrap();
        let dest = Path::new(&commit_dir).join(rel);

        if let Some(parent) = dest.parent() {
            fs::create_dir_all(parent).unwrap();
        }

        fs::copy(&file, &dest).unwrap();
    }

    update_head(id);
    println!("Committed as {}", id);
}

fn revert() {
    ensure_repo();

    let head = fs::read_to_string(HEAD_FILE).unwrap().trim().parse::<u64>().unwrap();
    if head == 0 {
        println!("No commits to revert to.");
        return;
    }

    let prev = head - 1;
    let commit_dir = format!("{}/{}", COMMITS, format!("{:05}", prev));

    if !Path::new(&commit_dir).exists() {
        println!("No earlier commit exists.");
        return;
    }

    for entry in WalkDir::new(&commit_dir) {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_dir() { continue; }

        let rel = path.strip_prefix(&commit_dir).unwrap();
        let dest = Path::new(".").join(rel);

        if let Some(parent) = dest.parent() {
            fs::create_dir_all(parent).unwrap();
        }

        fs::copy(path, dest).unwrap();
    }

    update_head(prev);
    println!("Reverted to {}", prev);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("Usage: scm <commit|revert>");
        return;
    }

    match args[1].as_str() {
        "commit" => commit(),
        "revert" => revert(),
        _ => println!("Unknown command."),
    }
}

