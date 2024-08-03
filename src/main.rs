use clap::{Arg, Command};
use ignore::WalkBuilder;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use indicatif::{ProgressBar, ProgressStyle};
use cli_clipboard::{ClipboardContext, ClipboardProvider};

fn parse_arguments() -> clap::ArgMatches {
    Command::new("File Aggregator")
        .version("1.0")
        .author("Sabir Khan <simplysabir@gmail.com>")
        .about("Combines contents of files in a directory into a single file")
        .arg(
            Arg::new("directory")
                .help("The directory to search for files")
                .default_value(".")
                .index(1),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .help("Output file path")
                .num_args(1),
        )
        .arg(
            Arg::new("stdout")
                .short('s')
                .long("stdout")
                .help("Print output to stdout")
                .num_args(0),
        )
        .arg(
            Arg::new("clipboard")
                .short('c')
                .long("clipboard")
                .help("Copy output to clipboard")
                .num_args(0),
        )
        .arg(
            Arg::new("include_hidden")
                .short('i')
                .long("include-hidden")
                .help("Include hidden files")
                .num_args(0),
        )
        .arg(
            Arg::new("no_ignore")
                .short('n')
                .long("no-ignore")
                .help("Ignore .gitignore rules")
                .num_args(0),
        )
        .arg(
            Arg::new("file_types")
                .short('f')
                .long("file-types")
                .help("Comma-separated list of file types to include (e.g., 'rs,js,py')")
                .num_args(1),
        )
        .get_matches()
}

fn aggregate_files(
    directory: &Path,
    include_hidden: bool,
    use_gitignore: bool,
    file_types: Option<&str>,
) -> io::Result<String> {
    let mut result = String::new();
    let bar = ProgressBar::new_spinner();
    bar.set_style(ProgressStyle::default_spinner().template("{spinner:.green} {msg}"));
    bar.set_message("Processing files...");

    if !directory.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Path is not a directory: {:?}", directory),
        ));
    }

    let walker = WalkBuilder::new(directory)
        .hidden(!include_hidden)
        .git_ignore(use_gitignore)
        .build();

    let allowed_extensions: Vec<&str> = file_types
        .map(|types| types.split(',').collect())
        .unwrap_or_default();

    for entry in walker {
        match entry {
            Ok(entry) => {
                let path = entry.path();

                if path.is_file() && !path.to_str().unwrap_or("").contains("node_modules") {
                    if allowed_extensions.is_empty()
                        || path
                            .extension()
                            .and_then(|ext| ext.to_str())
                            .map_or(false, |ext| allowed_extensions.contains(&ext))
                    {
                        let comment_style = determine_comment_style(path);
                        result.push_str(&format!(
                            "\n{} File: {}\n{} Path: {}\n",
                            comment_style,
                            path.file_name().unwrap_or_default().to_string_lossy(),
                            comment_style,
                            path.display()
                        ));

                        match fs::read_to_string(path) {
                            Ok(contents) => {
                                result.push_str(&contents);
                                result.push_str("\n");
                            }
                            Err(e) => println!("Error reading file {:?}: {}", path, e),
                        }

                        if comment_style == "<!--" {
                            result.push_str("-->\n");
                        } else if comment_style == "/*" {
                            result.push_str("*/\n");
                        }
                    }
                }
            }
            Err(e) => println!("Error accessing entry: {}", e),
        }
        bar.tick();
    }
    bar.finish_with_message("Done!");

    if result.is_empty() {
        println!("No files were processed. Result is empty.");
    } else {
        println!("Processed {} characters", result.len());
    }

    Ok(result.trim().to_string())
}

fn determine_comment_style(file_path: &Path) -> &'static str {
    match file_path.extension().and_then(|s| s.to_str()) {
        Some("js") | Some("ts") | Some("java") | Some("rs") => "//",
        Some("py") | Some("rb") | Some("sh") | Some("yml") | Some("yaml") => "#",
        Some("html") => "<!--",
        Some("css") => "/*",
        _ => "//",
    }
}

fn main() -> io::Result<()> {
    let args = parse_arguments();

    let directory = PathBuf::from(args.get_one::<String>("directory").unwrap());
    // Use `map_or` to provide a default value and handle output_path as a &str
    let output_path: &str = args.get_one::<String>("output").map_or("fileagg_output.txt", |s| s.as_str());
    let stdout = args.get_flag("stdout");
    let clipboard = args.get_flag("clipboard");
    let include_hidden = args.get_flag("include_hidden");
    let no_ignore = args.get_flag("no_ignore");
    let file_types = args.get_one::<String>("file_types").map(|s| s.as_str());

    let output = aggregate_files(&directory, include_hidden, !no_ignore, file_types)?;

    if stdout {
        println!("{}", output);
    } else if clipboard {
        let mut ctx = ClipboardContext::new().expect("Failed to initialize clipboard");
        ctx.set_contents(output).expect("Failed to copy to clipboard");
        println!("Selected file contents have been copied to the clipboard.");
    } else {
        // Directly use output_path here as it is already a &str
        let mut file = fs::File::create(output_path)?;
        file.write_all(output.as_bytes())?;
        println!("Output written to {}", output_path);
    }

    Ok(())
}

