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
            "Usage: add <archive.rat> <file> [--compression fast|best|default] [--meta name=value] [--meta name2=value2 ...]",
        )));
    }

    let archive_path = PathBuf::from(&args[0]);
    let file_path = PathBuf::from(&args[1]);

    let mut metadata = RatMeta::<RatMetaMap>::new_object();
    let mut compression = CompressionType::Best;

    let mut i = 2;
    while i < args.len() {
        match args[i].as_str() {
            "--compression" => {
                let value = next_arg(args, &mut i, "--compression")?;
                compression = parse_compression(&value)?;
            }
            "--meta" => {
                let assignment = next_arg(args, &mut i, "--meta")?;
                let (key, value) = parse_meta_assignment(&assignment)?;
                metadata.insert_custom(key, serde_json::Value::String(value));
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
    if File::open(&archive_path).is_ok() {
        rat_file = RatFile::open(archive_path)?;
    } else {
        println!("Archive not found, should be created: {:?}", archive_path);

        let can_create = prompt_yes_or_no("Create archive? (y/n): ")?;
        if !can_create {
            println!("Aborting, Goodbye!");
            return Err(Box::new(Error::new(
                ErrorKind::NotFound,
                "Archive not found and creation was declined",
            )));
        }

        rat_file = RatFile::new(archive_path, can_create, compression)?;
    }
    let item = rat_file.insert_to_rat_file(file_path, metadata)?;

    println!("Added '{}' ({})", item.name, item.id);
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

fn parse_compression(value: &str) -> Result<CompressionType, Box<dyn std::error::Error>> {
    match value.to_ascii_lowercase().as_str() {
        "fast" => Ok(CompressionType::Fast),
        "best" => Ok(CompressionType::Best),
        "default" => Ok(CompressionType::Default),
        _ => Ok(CompressionType::Default),
    }
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
    println!("  file_rat add <archive.rat> <file> [--compression fast|best|default] [--meta name=value] [--meta name2=value2 ...]");
    println!("  file_rat list <archive.rat>");
    println!("  file_rat extract <archive.rat> <id> <destination> [--remove]");
    println!("  file_rat remove <archive.rat> <id>");
    println!("  file_rat help");
}
