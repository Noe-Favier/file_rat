/// Represents the starting index for parsing optional command-line arguments.
///
/// In the `cmd_add` function, after the two required positional arguments
/// (archive path at index 0 and file path at index 1), all subsequent arguments
/// are optional flags like `--compression`, `--meta`, etc.
///
/// Starting at index 2 ensures we skip over the required arguments and begin
/// processing only the optional parameters.
const OPTIONAL_ARGS_START_INDEX: usize = 2;

use std::{
    env,
    fs::File,
    io::{self, Error, ErrorKind},
    path::PathBuf,
};
use uuid::Uuid;

use crate::structs::enums::compression_type::CompressionType;
use crate::structs::rat_meta::{RatMeta, RatMetaMap};
use structs::rat_file::RatFile;

mod structs;

fn main() {
    if let Err(error) = run() {
        eprintln!("Error: {error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty()
        || args[0] == "help"
        || args[0] == "--help"
        || args[0] == "-help"
        || args[0] == "-h"
    {
        print_usage();
        return Ok(());
    }

    match args[0].as_str() {
        "add" => cmd_add(&args[1..])?,
        "list" => cmd_list(&args[1..])?,
        "extract" => cmd_extract(&args[1..])?,
        "remove" => cmd_remove(&args[1..])?,
        _ => {
            print_usage();
            return Err(Box::new(Error::new(
                ErrorKind::InvalidInput,
                format!("Unknown command '{}'", args[0]),
            )));
        }
    }

    Ok(())
}

fn cmd_add(args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    if args.len() < 2 {
        return Err(Box::new(Error::new(
            ErrorKind::InvalidInput,
            "Usage: add <archive.rat> <file> [--compression fast|best|default] [--stfu] [--meta name=value] [--meta name2=value2 ...]",
        )));
    }

    let archive_path = PathBuf::from(&args[0]);
    let file_path = PathBuf::from(&args[1]);

    let mut metadata = RatMeta::<RatMetaMap>::new_object();
    let mut compression_override: Option<CompressionType> = None;

    let mut i = OPTIONAL_ARGS_START_INDEX;
    let mut stfu_flag = false;
    while i < args.len() {
        match args[i].as_str() {
            "--compression" => {
                let value = next_arg(args, &mut i, "--compression")?;

                if CompressionType::is_valid(&value) {
                    compression_override = Some(
                        CompressionType::from_str(&value)
                            .unwrap_or_default(),
                    );
                } else {
                    return Err(Box::new(Error::new(
                        ErrorKind::InvalidInput,
                        format!(
                            "Invalid compression value '{}', expected 'fast', 'best' or 'default'",
                            value
                        ),
                    )));
                }
            }
            "--meta" => {
                let assignment = next_arg(args, &mut i, "--meta")?;
                let (key, value) = parse_meta_assignment(&assignment)?;
                metadata.insert_custom(key, serde_json::Value::String(value));
            }
            "--stfu" => {
                // stfu flag suppresses all prompts and allows archive creation with enum default compression
                // if the archive needs to be created. It has no effect if the archive already exists.
                // This is useful for scripting and automation when you want to ensure the command runs without any prompts.
                // When --stfu is used, if the archive doesn't exist, it will be created with the default compression type without asking the user.

                stfu_flag = true;
            }
            _ => {
                if let Some(assignment) = args[i].strip_prefix("--meta=") {
                    let (key, value) = parse_meta_assignment(assignment)?;
                    metadata.insert_custom(key, serde_json::Value::String(value));
                } else {
                    return Err(Box::new(Error::new(
                        ErrorKind::InvalidInput,
                        format!("Unknown flag '{}'", args[i]),
                    )));
                }
            }
        }
        i += 1;
    }

    let mut rat_file: RatFile<RatMeta<RatMetaMap>>;
    let compression_to_use: CompressionType;
    if File::open(&archive_path).is_ok() {
        rat_file = RatFile::open(archive_path)?;
        compression_to_use = compression_override
            .unwrap_or_else(|| rat_file.compression_type.clone());
    } else {
        if !stfu_flag {
            println!("Archive not found, should be created: {:?}", archive_path);
        }

        let can_create = if stfu_flag {
            true
        } else {
            prompt_yes_or_no("Create archive? (y/n): ")?
        };
        if !can_create {
            println!("Aborting, Goodbye!");
            return Err(Box::new(Error::new(
                ErrorKind::NotFound,
                "Archive not found and creation was declined",
            )));
        }

        compression_to_use = compression_override.unwrap_or(
            if stfu_flag {
                CompressionType::default()
            } else {
                prompt_compression_type()?
            }
        );
        
        rat_file = RatFile::new(archive_path, can_create, compression_to_use.clone())?;
    }
    let item = rat_file.insert_to_rat_file(file_path, metadata, compression_to_use)?;

    if !stfu_flag {
        println!("Added '{}' ({})", item.name, item.id);
    }
    Ok(())
}

fn prompt_yes_or_no(message: &str) -> Result<bool, Box<dyn std::error::Error>> {
    print!("{}", message);
    io::Write::flush(&mut io::stdout())?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    loop {
        let value = input.trim().to_ascii_lowercase();
        if value == "yes" || value == "y" {
            return Ok(true);
        } else if value == "no" || value == "n" {
            return Ok(false);
        } else {
            print!("Please enter 'yes'/'y' or 'no'/'n': ");
            io::Write::flush(&mut io::stdout())?;
            input.clear();
            io::stdin().read_line(&mut input)?;
        }
    }
}

fn prompt_compression_type() -> Result<CompressionType, Box<dyn std::error::Error>> {
    loop {
        print!("Use default compression? (y/yes or Enter = default, n/no = choose): ");
        io::Write::flush(&mut io::stdout())?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let choice = input.trim().to_ascii_lowercase();

        if choice.is_empty() || choice == "y" || choice == "yes" {
            return Ok(CompressionType::default());
        }

        if choice == "n" || choice == "no" {
            println!("Available compression modes: fast | best | default");
            loop {
                print!("Choose compression mode: ");
                io::Write::flush(&mut io::stdout())?;

                input.clear();
                io::stdin().read_line(&mut input)?;
                let value = input.trim().to_ascii_lowercase();

                if CompressionType::is_valid(&value) {
                    return Ok(CompressionType::from_str(&value).unwrap_or_default());
                }

                println!("Invalid value. Please enter 'fast', 'best' or 'default'.");
            }
        }

        println!("Please enter 'y'/'yes', 'n'/'no', or press Enter.");
    }
}

fn cmd_list(args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    if args.len() != 1 {
        return Err(Box::new(Error::new(
            ErrorKind::InvalidInput,
            "Usage: list <archive.rat>",
        )));
    }

    let rat_file: RatFile<RatMeta<RatMetaMap>> = RatFile::open(PathBuf::from(&args[0]))?;
    let items = rat_file.list_rat_file()?;

    if items.is_empty() {
        println!("Archive is empty");
        return Ok(());
    }

    for item in items {
        println!("{} | {} | {} bytes", item.id, item.name, item.size);
    }

    Ok(())
}

fn cmd_extract(args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    if args.len() < 3 {
        return Err(Box::new(Error::new(
            ErrorKind::InvalidInput,
            "Usage: extract <archive.rat> <id> <destination> [--remove]",
        )));
    }

    let archive_path = PathBuf::from(&args[0]);
    let selector = &args[1];
    let destination = PathBuf::from(&args[2]);
    let should_remove = args.iter().skip(3).any(|arg| arg == "--remove");

    let mut rat_file: RatFile<RatMeta<RatMetaMap>> = RatFile::open(archive_path)?;
    let id = resolve_item_id(&rat_file, selector)?;
    let output = rat_file.extract(id, destination, should_remove)?;

    println!("Extracted to {:?}", output);
    Ok(())
}

fn cmd_remove(args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    if args.len() != 2 {
        return Err(Box::new(Error::new(
            ErrorKind::InvalidInput,
            "Usage: remove <archive.rat> <id>",
        )));
    }

    let mut rat_file: RatFile<RatMeta<RatMetaMap>> = RatFile::open(PathBuf::from(&args[0]))?;
    let id = resolve_item_id(&rat_file, &args[1])?;
    rat_file.remove(id)?;
    println!("Removed '{}'", args[1]);
    Ok(())
}

fn resolve_item_id(
    rat_file: &RatFile<RatMeta<RatMetaMap>>,
    selector: &str,
) -> Result<Uuid, Box<dyn std::error::Error>> {
    let items = rat_file.list_rat_file()?;

    if let Ok(parsed_uuid) = Uuid::parse_str(selector) {
        if items.iter().any(|item| item.id == parsed_uuid) {
            return Ok(parsed_uuid);
        }
    }

    if let Some(item) = items.iter().find(|item| item.name == selector) {
        return Ok(item.id);
    }

    Err(Box::new(Error::new(
        ErrorKind::NotFound,
        format!("Item '{}' not found", selector),
    )))
}

fn next_arg(
    args: &[String],
    index: &mut usize,
    flag: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let value_index = *index + 1;
    let Some(value) = args.get(value_index) else {
        return Err(Box::new(Error::new(
            ErrorKind::InvalidInput,
            format!("Missing value for {}", flag),
        )));
    };

    *index = value_index;
    Ok(value.to_string())
}

fn parse_meta_assignment(assignment: &str) -> Result<(String, String), Box<dyn std::error::Error>> {
    let mut parts = assignment.splitn(2, '=');
    let key = parts.next().unwrap_or_default().trim();
    let value = parts.next().unwrap_or_default();

    if key.is_empty() || value.is_empty() {
        return Err(Box::new(Error::new(
            ErrorKind::InvalidInput,
            format!(
                "Invalid metadata format '{}', expected --meta name=value",
                assignment
            ),
        )));
    }

    Ok((key.to_string(), value.to_string()))
}

fn print_usage() {
    println!("file_rat CLI");
    println!("Usage:");
    println!("  file_rat add <archive.rat> <file> [--compression fast|best|default] [--stfu] [--meta name=value] [--meta name2=value2 ...]");
    println!("  file_rat list <archive.rat>");
    println!("  file_rat extract <archive.rat> <id> <destination> [--remove]");
    println!("  file_rat remove <archive.rat> <id>");
    println!("  file_rat help");
}
