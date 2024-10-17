use std::collections::HashMap;
use std::fs::{self, DirEntry, OpenOptions};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::{env, io, io::Write};

struct GloablParams {
    map: HashMap<String, PathBuf>,
    log_file: fs::File,
}

fn walk_dir(dir: &Path, glb_params: &mut GloablParams) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let file_type = entry.file_type()?;

        let path = entry.path();
        let file_name = entry.file_name();

        // Skip dotfiles/dotfolders
        if file_name.to_str().unwrap_or("").starts_with('.') {
            continue;
        }

        if file_type.is_dir() {
            walk_dir(&path, glb_params)?;
        } else {
            process_file(entry, glb_params)?;
        }
    }
    Ok(())
}

fn process_file(entry: DirEntry, glb_params: &mut GloablParams) -> io::Result<()> {
    let output = Command::new("sha256sum").arg(entry.path()).output()?;
    writeln!(glb_params.log_file, "Processing: {:?}", entry.path())?;
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        if let Some(checksum) = stdout.split_whitespace().next() {
            let checksum = checksum.to_string();
            handle_duplicate(&checksum, entry.path(), glb_params)?;
        }
    }
    Ok(())
}

fn handle_duplicate(
    checksum: &str,
    file_path: PathBuf,
    glb_params: &mut GloablParams,
) -> io::Result<()> {
    if let Some(existing_path) = glb_params.map.get(checksum) {
        let (m1, m2) = (
            fs::metadata(existing_path)?.modified()?,
            fs::metadata(&file_path)?.modified()?,
        );

        writeln!(
            glb_params.log_file,
            "DUPLICATE DETECTED: {:?} and {:?}",
            existing_path, file_path
        )?;

        if m1 < m2 {
            writeln!(glb_params.log_file, "DELETING: {:?}", existing_path)?;
            fs::remove_file(existing_path)?;
            glb_params.map.insert(checksum.to_string(), file_path);
        } else {
            writeln!(glb_params.log_file, "DELETING: {:?}", file_path)?;
            fs::remove_file(file_path)?;
        }
    } else {
        glb_params.map.insert(checksum.to_string(), file_path);
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("ERROR: No direcotry path provided.");
        eprintln!("USAGE: {} <path>", args[0]);
        return;
    }

    let mut glb_params = GloablParams {
        map: HashMap::new(),
        log_file: OpenOptions::new()
            .create(true)
            .append(true)
            .open("dedupe.log")
            .expect("Unable to open or create a log file"),
    };

    let dir_path = Path::new(&args[1]);
    if !dir_path.is_dir() {
        eprintln!("ERROR: {:?} is not a directory", args[1]);
        return;
    }
    if let Err(e) = walk_dir(&dir_path, &mut glb_params) {
        eprintln!("ERROR: {}", e);
    }
    writeln!(glb_params.log_file, "Processing completed.").expect("Unable to write to log file");
    println!("Processing completed.");
}
