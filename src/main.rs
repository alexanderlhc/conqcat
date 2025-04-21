use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::path::Path;

use clap::{Arg, ArgAction, Command};
use walkdir::WalkDir;

fn main() {
    let matches = Command::new("Conqcat")
        .about("Conquers files and merges them into one")
        .arg(
            Arg::new("filetype")
                .required(true)
                .short('t')
                .long("type")
                .action(ArgAction::Append),
        )
        .arg(
            Arg::new("directory")
                .required(true)
                .short('d')
                .long("dir")
                .action(ArgAction::Append),
        )
        .arg(Arg::new("output").required(true).short('o').long("output"))
        .get_matches();

    let filetypes: HashSet<String> = matches
        .get_many::<String>("filetype")
        .unwrap_or_default()
        .map(|s| s.to_lowercase())
        .collect();

    let directories: Vec<String> = matches
        .get_many::<String>("directory")
        .unwrap_or_default()
        .map(|s| s.to_owned())
        .collect();

    let output: String = matches.get_one::<String>("output").unwrap().to_string();

    let output_file = File::create(output.clone()).unwrap();
    let mut writer = BufWriter::new(output_file);
    for directory in directories {
        combine_files(Path::new(&directory), &filetypes, &mut writer).unwrap();
    }

    println!("Done file has been created: {}", output)
}

fn combine_files(
    directory: &Path,
    filetypes: &HashSet<String>,
    writer: &mut BufWriter<File>,
) -> io::Result<()> {
    for entry in WalkDir::new(directory).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            if let Some(ext) = entry.path().extension().and_then(|s| s.to_str()) {
                if filetypes.contains(&ext.to_lowercase()) {
                    let fname = entry.path().to_string_lossy();
                    writeln!(writer, "\n\n-- {} --\n", fname)?;
                    let file = File::open(entry.path())?;
                    let reader = BufReader::new(file);
                    for line in reader.lines() {
                        writeln!(writer, "{}", line?)?;
                    }
                }
            }
        }
    }
    writer.flush()?;
    Ok(())
}
