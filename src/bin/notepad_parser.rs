use clap::{value_parser, Arg, Command};
use csv::WriterBuilder;
use glob::glob;
use notepad_parser::{
    enums::{CRType, Encoding},
    errors::NotepadErrors,
    NotepadTabStat,
};
use serde::Serialize;
use serde_json;
use std::{
    convert::From,
    fs::File,
    io::{self, Write},
    process::exit,
};

use log::*;
use log4rs::{
    append::console::{ConsoleAppender, Target},
    config::{Appender, Root},
    encode::pattern::PatternEncoder,
    Config,
};

use winparsingtools::date_time::FileTime;

enum OutputFormat {
    JSONL,
    CSV,
}

impl From<&str> for OutputFormat {
    fn from(value: &str) -> Self {
        match value {
            "jsonl" => OutputFormat::JSONL,
            "csv" => OutputFormat::CSV,
            _ => OutputFormat::JSONL,
        }
    }
}

#[derive(Debug, Serialize)]
struct CsvRecord {
    tabstate_path: Option<String>,
    is_saved_file: bool,
    path_size: u64,
    path: Option<String>,
    file_size: Option<u64>,
    encoding: Option<Encoding>,
    cr_type: Option<CRType>,
    last_write_time: Option<FileTime>,
    file_hash: Option<String>,
    cursor_start: Option<u64>,
    cursor_end: Option<u64>,
    word_wrap: bool,
    rtl: bool,
    show_unicode: bool,
    version: u64,
    file_content_size: u64,
    file_content: String,
    contain_unsaved_data: bool,
    checksum: String,
    unsaved_chunks_str: Option<String>,
    raw: String,
}

impl From<NotepadTabStat> for CsvRecord {
    fn from(value: NotepadTabStat) -> Self {
        let json_data = match serde_json::to_string(&value) {
            Ok(data) => data,
            Err(e) => e.to_string(),
        };
        Self {
            tabstate_path: value.tabstate_path,
            is_saved_file: value.is_saved_file,
            path_size: value.path_size,
            path: value.path,
            file_size: value.file_size,
            encoding: value.encoding,
            cr_type: value.cr_type,
            last_write_time: value.last_write_time,
            file_hash: value.file_hash,
            cursor_start: value.cursor_start,
            cursor_end: value.cursor_end,
            word_wrap: value.config_block.word_wrap,
            rtl: value.config_block.rtl,
            show_unicode: value.config_block.show_unicode,
            version: value.config_block.version,
            file_content_size: value.file_content_size,
            file_content: value.file_content,
            contain_unsaved_data: value.contain_unsaved_data,
            checksum: value.checksum,
            unsaved_chunks_str: value.unsaved_chunks_str,
            raw: json_data,
        }
    }
}

fn init_logger(level: log::LevelFilter) -> log4rs::Handle {
    let log_format = "{d(%Y-%m-%d %H:%M:%S)(utc)} [{t}:{L:<3}] {h({l:<5})} {m}\n";

    let stderr = ConsoleAppender::builder()
        .target(Target::Stderr)
        .encoder(Box::new(PatternEncoder::new(log_format)))
        .build();

    // Log Trace level output to file where trace is the default level
    // and the programmatically specified level to stderr

    let config_builder =
        Config::builder().appender(Appender::builder().build("stderr", Box::new(stderr)));

    let root_builder = Root::builder().appender("stderr");

    let config = config_builder.build(root_builder.build(level)).unwrap();

    log4rs::init_config(config).unwrap()
}

fn main() {
    let cli = Command::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author("AbdulRhman Alfaifi <aalfaifi@u0041.co>")
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .help_template("\
{before-help}

Created By: {author}
Version: v{version}
Reference: https://u0041.co/posts/articals/exploring-windows-artifacts-notepad-files/

{about}

{usage-heading} {usage}

{all-args}{after-help}
")
        .arg(
            Arg::new("input-file")
                .value_name("FILE")
                .help("Path the files to parse. Accepts glob.")
                .default_value("C:\\Users\\*\\AppData\\Local\\Packages\\Microsoft.WindowsNotepad_8wekyb3d8bbwe\\LocalState\\TabState\\????????-????-????-????-????????????.bin")
                .value_parser(value_parser!(String)),
        )
        .arg(
            Arg::new("output-format")
                .short('f')
                .long("output-format")
                .value_name("FORMAT")
                .help("Specifiy the output format")
                .value_parser(["jsonl", "csv"])
                .default_value("jsonl"),
        )
        .arg(
            Arg::new("output-path")
                .short('o')
                .long("output-path")
                .value_name("FILE")
                .help("Specifiy the output file")
                .value_parser(value_parser!(String))
                .default_value("stdout"),
        )
        .arg(
            Arg::new("log-level")
                .short('l')
                .long("log-level")
                .value_name("LEVEL")
                .help("Level for logs")
                .value_parser(["trace", "debug", "info", "error", "quiet"])
                .default_value("quiet"),
        )
        .get_matches();

    let path = match cli.get_one::<String>("input-file") {
        Some(path) => path,
        None => "C:\\Users\\*\\AppData\\Local\\Packages\\Microsoft.WindowsNotepad_8wekyb3d8bbwe\\LocalState\\TabState\\*.bin"
    };
    let output_format = match cli.get_one::<String>("output-format") {
        Some(format) => OutputFormat::from(format.to_owned().as_str()),
        None => OutputFormat::from("jsonl"),
    };

    let mut output_path = "stdout".to_string();
    let mut output: Box<dyn Write> = match cli.get_one::<String>("output-path") {
        Some(path) => {
            output_path = path.to_owned();
            match output_path.as_str() {
                "stdout" => Box::new(io::stdout()),
                path => match File::create(path) {
                    Ok(f) => Box::new(f),
                    Err(e) => {
                        error!(
                            "Unable create the output file '{}', ERROR: {}. Exiting...",
                            path, e
                        );
                        exit(1);
                    }
                },
            }
        }
        None => Box::new(io::stdout()),
    };

    let log_level = match cli
        .get_one::<String>("log-level")
        .unwrap()
        .to_owned()
        .as_str()
    {
        "trace" => log::LevelFilter::Trace,
        "debug" => log::LevelFilter::Debug,
        "info" => log::LevelFilter::Info,
        "error" => log::LevelFilter::Error,
        _ => log::LevelFilter::Off,
    };

    init_logger(log_level);

    let mut csv_headers_printed = false;
    // if let OutputFormat::CSV = output_format {}

    for entry in glob(path).expect("Failed to read glob pattern") {
        match entry {
            Ok(path_match) => {
                let path_str = match path_match.to_str() {
                    Some(p) => p,
                    None => {
                        error!(
                            "Unable to convert from String to &str for '{}'",
                            path_match.to_string_lossy()
                        );
                        continue;
                    }
                };
                match NotepadTabStat::from_path(path_str) {
                    Ok(data) => match output_format {
                        OutputFormat::JSONL => match serde_json::to_string(&data) {
                            Ok(json) => match write!(output, "{}\n", json) {
                                Ok(_) => debug!(
                                    "Successfully writen JSON data for the file '{}'",
                                    path_str
                                ),
                                Err(e) => error!(
                                    "Error while writing the JSON data for the file '{}', ERROR: {}",
                                    path_str,
                                    e
                                ),
                            },
                            Err(e) => {
                                error!(
                                    "{}",
                                    NotepadErrors::CLIError(
                                        e.to_string(),
                                        format!(
                                            "Unable to convert results to JSON for the file '{}'",
                                            path_str
                                        )
                                    )
                                );
                            }
                        },
                        OutputFormat::CSV => {
                            let mut csv_writer = WriterBuilder::new();
                            let mut csv_writer_builder;
                            if csv_headers_printed {
                                csv_writer_builder =
                                    csv_writer.has_headers(false).from_writer(vec![]);
                            } else {
                                csv_writer_builder =
                                    csv_writer.has_headers(true).from_writer(vec![]);
                                csv_headers_printed = true;
                            }

                            let csv_record = CsvRecord::from(data);
                            match csv_writer_builder.serialize(csv_record) {
                                Ok(_) => debug!(
                                    "Successfuly serilized CSV row for the file '{}'",
                                    path_str
                                ),
                                Err(e) => error!(
                                    "Unable to write CSV row, ERROR: {}, PATH: '{}'",
                                    e, path_str
                                ),
                            }
                            match csv_writer_builder.flush() {
                                Ok(_) => trace!(
                                    "Susseccfuly flushed the CSV record for the file '{}'",
                                    path_str
                                ),
                                Err(e) => error!(
                                    "Unable to flush CSV record, ERROR: {}, PATH: '{}'",
                                    e, path_str
                                ),
                            }

                            let row = match csv_writer_builder.into_inner() {
                                Ok(bytes) => match String::from_utf8(bytes) {
                                    Ok(r) => r,
                                    Err(e) => {
                                        error!("Unable to convert CSV writer buffer to String, ERROR: {}", e);
                                        continue;
                                    }
                                },
                                Err(e) => {
                                    error!("Unable to convert CSV writer to String, ERROR: {}", e);
                                    continue;
                                }
                            };
                            match write!(output, "{}", row) {
                                Ok(_) => debug!(
                                    "Successfully writen the CSV row for file '{}' to '{}'",
                                    path_str, output_path
                                ),
                                Err(e) => error!(
                                    "Unable to write the CSV row for file '{}' to '{}', ERROR: {}",
                                    path_str, output_path, e
                                ),
                            }
                        }
                    },
                    Err(e) => {
                        error!(
                            "{}",
                            NotepadErrors::CLIError(
                                e.to_string(),
                                format!("Unable to parse the file '{}'", path_str)
                            )
                        );
                    }
                }
            }
            Err(e) => eprintln!("{:?}", e),
        }
    }
}
