use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use colored::Colorize;

use crate::archive::{self, ArchiveFormat};
use crate::progress::CliProgress;

#[derive(Parser)]
#[command(
    name = "ziprs",
    version,
    about = "ZipRS — A fast, modern archive manager",
    long_about = "ZipRS — A fast, modern archive manager built with Rust.\n\nRun without arguments to launch the GUI.\nUse subcommands for CLI operations."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// List contents of an archive
    #[command(alias = "ls")]
    List {
        /// Path to the archive
        archive: PathBuf,

        /// Long format with sizes, dates, and compression info
        #[arg(short, long)]
        long: bool,
    },

    /// Extract files from an archive
    #[command(alias = "x")]
    Extract {
        /// Path to the archive
        archive: PathBuf,

        /// Output directory [default: current directory]
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Extract only specific entries
        #[arg(long, num_args = 1..)]
        files: Vec<String>,

        /// Show progress
        #[arg(short, long)]
        verbose: bool,
    },

    /// Create a new archive
    #[command(alias = "c")]
    Create {
        /// Output archive path (format detected from extension)
        output: PathBuf,

        /// Files and directories to add
        #[arg(required = true, num_args = 1..)]
        files: Vec<PathBuf>,

        /// Show progress
        #[arg(short, long)]
        verbose: bool,
    },

    /// Add files to an existing archive (ZIP only)
    Add {
        /// Path to the archive
        archive: PathBuf,

        /// Files to add
        #[arg(required = true, num_args = 1..)]
        files: Vec<PathBuf>,

        /// Path prefix inside the archive
        #[arg(short, long, default_value = "")]
        prefix: String,

        /// Show progress
        #[arg(short, long)]
        verbose: bool,
    },

    /// Delete entries from an archive (ZIP only)
    #[command(alias = "rm")]
    Delete {
        /// Path to the archive
        archive: PathBuf,

        /// Entry paths to delete
        #[arg(required = true, num_args = 1..)]
        entries: Vec<String>,

        /// Show progress
        #[arg(short, long)]
        verbose: bool,
    },

    /// Test archive integrity
    #[command(alias = "t")]
    Test {
        /// Path to the archive
        archive: PathBuf,

        /// Show progress
        #[arg(short, long)]
        verbose: bool,
    },

    /// Show detailed archive information
    Info {
        /// Path to the archive
        archive: PathBuf,
    },
}

pub fn run_cli(cli: Cli) -> Result<()> {
    match cli.command {
        Commands::List { archive, long } => cmd_list(&archive, long),
        Commands::Extract {
            archive,
            output,
            files,
            verbose,
        } => cmd_extract(&archive, output, files, verbose),
        Commands::Create {
            output,
            files,
            verbose,
        } => cmd_create(&output, files, verbose),
        Commands::Add {
            archive,
            files,
            prefix,
            verbose,
        } => cmd_add(&archive, files, &prefix, verbose),
        Commands::Delete {
            archive,
            entries,
            verbose,
        } => cmd_delete(&archive, entries, verbose),
        Commands::Test { archive, verbose } => cmd_test(&archive, verbose),
        Commands::Info { archive } => cmd_info(&archive),
    }
}

// ---- Commands ----

fn cmd_list(path: &PathBuf, long: bool) -> Result<()> {
    let backend = archive::open_archive(path)?;
    let entries = backend.list_entries()?;

    if entries.is_empty() {
        println!("{}", "Archive is empty.".dimmed());
        return Ok(());
    }

    if long {
        // Header
        println!(
            "{:>10}  {:>10}  {:>5}  {:>8}  {:<16}  {}",
            "Size".bold().underline(),
            "Packed".bold().underline(),
            "Ratio".bold().underline(),
            "Method".bold().underline(),
            "Modified".bold().underline(),
            "Name".bold().underline(),
        );

        let mut total_size: u64 = 0;
        let mut total_packed: u64 = 0;
        let mut file_count = 0usize;
        let mut dir_count = 0usize;

        for e in &entries {
            let name = if e.is_directory {
                format!("{}/", e.path).blue().bold().to_string()
            } else {
                e.path.clone()
            };

            let size_str = if e.is_directory {
                "-".dimmed().to_string()
            } else {
                format_size(e.uncompressed_size)
            };

            let packed_str = if e.is_directory {
                "-".dimmed().to_string()
            } else {
                format_size(e.compressed_size)
            };

            let ratio_str = if e.is_directory || e.uncompressed_size == 0 {
                "-".dimmed().to_string()
            } else {
                let r =
                    (1.0 - e.compressed_size as f64 / e.uncompressed_size as f64) * 100.0;
                format!("{:.0}%", r.max(0.0))
            };

            let modified = e.modified.as_deref().unwrap_or("-");

            println!(
                "{:>10}  {:>10}  {:>5}  {:>8}  {:<16}  {}",
                size_str, packed_str, ratio_str, e.compression_method, modified, name,
            );

            if e.is_directory {
                dir_count += 1;
            } else {
                file_count += 1;
                total_size += e.uncompressed_size;
                total_packed += e.compressed_size;
            }
        }

        // Summary
        println!("{}", "─".repeat(72).dimmed());
        let overall_ratio = if total_size > 0 {
            format!(
                "{:.0}%",
                ((1.0 - total_packed as f64 / total_size as f64) * 100.0).max(0.0)
            )
        } else {
            "-".to_string()
        };
        println!(
            "{:>10}  {:>10}  {:>5}  {} files, {} folders",
            format_size(total_size).bold(),
            format_size(total_packed).bold(),
            overall_ratio.bold(),
            file_count.to_string().cyan(),
            dir_count.to_string().cyan(),
        );
    } else {
        // Short format — one entry per line
        for e in &entries {
            if e.is_directory {
                println!("{}/", e.path.blue());
            } else {
                println!("{}", e.path);
            }
        }
    }

    Ok(())
}

fn cmd_extract(path: &PathBuf, output: Option<PathBuf>, files: Vec<String>, verbose: bool) -> Result<()> {
    let backend = archive::open_archive(path)?;
    let dest = output.unwrap_or_else(|| PathBuf::from("."));
    fs::create_dir_all(&dest).with_context(|| format!("Cannot create directory: {}", dest.display()))?;
    let progress = CliProgress::new(verbose);

    if files.is_empty() {
        backend.extract_all(&dest, &progress)?;
        let entries = backend.list_entries()?;
        println!(
            "{} Extracted {} entries to {}",
            "✓".green().bold(),
            entries.len().to_string().cyan(),
            dest.display().to_string().bold(),
        );
    } else {
        let count = files.len();
        backend.extract_entries(&files, &dest, &progress)?;
        println!(
            "{} Extracted {} entries to {}",
            "✓".green().bold(),
            count.to_string().cyan(),
            dest.display().to_string().bold(),
        );
    }

    Ok(())
}

fn cmd_create(output: &PathBuf, files: Vec<PathBuf>, verbose: bool) -> Result<()> {
    let format = ArchiveFormat::from_path(output)?;
    let progress = CliProgress::new(verbose);

    // Collect all files, expanding directories
    let mut all_files = Vec::new();
    for f in &files {
        if f.is_dir() {
            collect_files_recursive(f, &mut all_files)?;
        } else if f.exists() {
            all_files.push(f.clone());
        } else {
            anyhow::bail!("File not found: {}", f.display());
        }
    }

    // Determine base directory (common parent)
    let base_dir = if files.len() == 1 && files[0].is_dir() {
        files[0].parent().unwrap_or(&files[0]).to_path_buf()
    } else {
        std::env::current_dir()?
    };

    // Make paths absolute
    let abs_files: Vec<PathBuf> = all_files
        .iter()
        .map(|f| {
            if f.is_absolute() {
                f.clone()
            } else {
                std::env::current_dir().unwrap().join(f)
            }
        })
        .collect();
    let abs_base = if base_dir.is_absolute() {
        base_dir
    } else {
        std::env::current_dir()?.join(&base_dir)
    };

    archive::create_archive(output, &abs_files, &abs_base, format, &progress)?;

    let file_size = fs::metadata(output)?.len();
    println!(
        "{} Created {} ({}, {})",
        "✓".green().bold(),
        output.display().to_string().bold(),
        format.display_name().cyan(),
        format_size(file_size),
    );

    Ok(())
}

fn cmd_add(path: &PathBuf, files: Vec<PathBuf>, prefix: &str, verbose: bool) -> Result<()> {
    let mut backend = archive::open_archive(path)?;
    if !backend.format().supports_modification() {
        anyhow::bail!(
            "{} archives do not support adding files",
            backend.format().display_name()
        );
    }
    let progress = CliProgress::new(verbose);
    let count = files.len();
    backend.add_files(&files, prefix, &progress)?;
    println!(
        "{} Added {} files to {}",
        "✓".green().bold(),
        count.to_string().cyan(),
        path.display().to_string().bold(),
    );
    Ok(())
}

fn cmd_delete(path: &PathBuf, entries: Vec<String>, verbose: bool) -> Result<()> {
    let mut backend = archive::open_archive(path)?;
    if !backend.format().supports_modification() {
        anyhow::bail!(
            "{} archives do not support deleting entries",
            backend.format().display_name()
        );
    }
    let progress = CliProgress::new(verbose);
    let count = entries.len();
    backend.delete_entries(&entries, &progress)?;
    println!(
        "{} Deleted {} entries from {}",
        "✓".green().bold(),
        count.to_string().cyan(),
        path.display().to_string().bold(),
    );
    Ok(())
}

fn cmd_test(path: &PathBuf, verbose: bool) -> Result<()> {
    let backend = archive::open_archive(path)?;
    let progress = CliProgress::new(verbose);
    let count = archive::test_archive(&*backend, &progress)?;
    println!(
        "{} Archive OK — {} entries verified",
        "✓".green().bold(),
        count.to_string().cyan(),
    );
    Ok(())
}

fn cmd_info(path: &PathBuf) -> Result<()> {
    let backend = archive::open_archive(path)?;
    let entries = backend.list_entries()?;
    let format = backend.format();

    let file_size = fs::metadata(path)?.len();
    let file_count = entries.iter().filter(|e| !e.is_directory).count();
    let dir_count = entries.iter().filter(|e| e.is_directory).count();
    let total_uncompressed: u64 = entries.iter().map(|e| e.uncompressed_size).sum();
    let total_compressed: u64 = entries.iter().map(|e| e.compressed_size).sum();

    let overall_ratio = if total_uncompressed > 0 {
        format!(
            "{:.1}%",
            ((1.0 - total_compressed as f64 / total_uncompressed as f64) * 100.0).max(0.0)
        )
    } else {
        "-".to_string()
    };

    // Collect compression methods used
    let mut methods: Vec<String> = entries
        .iter()
        .filter(|e| !e.is_directory)
        .map(|e| e.compression_method.clone())
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();
    methods.sort();

    let encrypted_count = entries.iter().filter(|e| e.encrypted).count();

    println!("{}", "Archive Information".bold().underline());
    println!();
    println!("  {}  {}", "Path:".bold(), path.display());
    println!(
        "  {}  {} ({})",
        "Format:".bold(),
        format.display_name().cyan(),
        if format.supports_modification() {
            "read/write"
        } else {
            "read-only"
        }
    );
    println!("  {}  {}", "Size on disk:".bold(), format_size(file_size));
    println!();
    println!(
        "  {}  {}",
        "Total entries:".bold(),
        entries.len().to_string().cyan()
    );
    println!("  {}  {}", "Files:".bold(), file_count);
    println!("  {}  {}", "Directories:".bold(), dir_count);
    println!();
    println!(
        "  {}  {}",
        "Uncompressed:".bold(),
        format_size(total_uncompressed)
    );
    println!(
        "  {}  {}",
        "Compressed:".bold(),
        format_size(total_compressed)
    );
    println!(
        "  {}  {}",
        "Ratio:".bold(),
        overall_ratio.green()
    );
    println!(
        "  {}  {}",
        "Methods:".bold(),
        methods.join(", ")
    );

    if encrypted_count > 0 {
        println!(
            "  {}  {} entries",
            "Encrypted:".bold(),
            encrypted_count.to_string().yellow()
        );
    }

    Ok(())
}

// ---- Helpers ----

fn format_size(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KiB", "MiB", "GiB", "TiB"];
    if bytes == 0 {
        return "0 B".to_string();
    }
    let mut size = bytes as f64;
    for unit in UNITS {
        if size < 1024.0 {
            return if size >= 100.0 || size.fract() < 0.05 {
                format!("{:.0} {}", size, unit)
            } else {
                format!("{:.1} {}", size, unit)
            };
        }
        size /= 1024.0;
    }
    format!("{:.1} PiB", size)
}

fn collect_files_recursive(dir: &PathBuf, out: &mut Vec<PathBuf>) -> Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            collect_files_recursive(&path, out)?;
        } else {
            out.push(path);
        }
    }
    Ok(())
}
