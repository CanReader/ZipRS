use std::collections::HashMap;
use std::fs;
use std::io::{self, Read, Write};
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

        /// Filter entries by glob pattern (e.g. "*.rs")
        #[arg(short, long)]
        filter: Option<String>,
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

    /// Display archive contents as a tree
    Tree {
        /// Path to the archive
        archive: PathBuf,

        /// Max depth to display
        #[arg(short, long)]
        depth: Option<usize>,

        /// Show file sizes
        #[arg(short, long)]
        size: bool,
    },

    /// Search for entries matching a pattern
    #[command(alias = "find")]
    Search {
        /// Path to the archive
        archive: PathBuf,

        /// Search pattern (case-insensitive substring match)
        pattern: String,

        /// Show sizes and dates
        #[arg(short, long)]
        long: bool,
    },

    /// Print file contents from archive to stdout
    Cat {
        /// Path to the archive
        archive: PathBuf,

        /// Entry path inside the archive
        entry: String,
    },

    /// Convert an archive to a different format
    Convert {
        /// Source archive
        input: PathBuf,

        /// Destination archive (format detected from extension)
        output: PathBuf,

        /// Show progress
        #[arg(short, long)]
        verbose: bool,
    },

    /// Compare two archives and show differences
    Diff {
        /// First archive
        archive_a: PathBuf,

        /// Second archive
        archive_b: PathBuf,
    },

    /// Show size statistics and top largest files
    Stats {
        /// Path to the archive
        archive: PathBuf,

        /// Number of top files to show
        #[arg(short = 'n', long, default_value = "10")]
        top: usize,
    },

    /// Show file type breakdown of archive contents
    Types {
        /// Path to the archive
        archive: PathBuf,
    },
}

pub fn run_cli(cli: Cli) -> Result<()> {
    match cli.command {
        Commands::List {
            archive,
            long,
            filter,
        } => cmd_list(&archive, long, filter),
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
        Commands::Tree {
            archive,
            depth,
            size,
        } => cmd_tree(&archive, depth, size),
        Commands::Search {
            archive,
            pattern,
            long,
        } => cmd_search(&archive, &pattern, long),
        Commands::Cat { archive, entry } => cmd_cat(&archive, &entry),
        Commands::Convert {
            input,
            output,
            verbose,
        } => cmd_convert(&input, &output, verbose),
        Commands::Diff {
            archive_a,
            archive_b,
        } => cmd_diff(&archive_a, &archive_b),
        Commands::Stats { archive, top } => cmd_stats(&archive, top),
        Commands::Types { archive } => cmd_types(&archive),
    }
}

// ---- Commands ----

fn cmd_list(path: &PathBuf, long: bool, filter: Option<String>) -> Result<()> {
    let backend = archive::open_archive(path)?;
    let mut entries = backend.list_entries()?;

    // Apply filter
    if let Some(ref pattern) = filter {
        let pat = pattern.to_lowercase();
        entries.retain(|e| {
            let name = e.name.to_lowercase();
            if pat.starts_with('*') && pat.ends_with('*') {
                name.contains(&pat[1..pat.len() - 1])
            } else if pat.starts_with('*') {
                name.ends_with(&pat[1..])
            } else if pat.ends_with('*') {
                name.starts_with(&pat[..pat.len() - 1])
            } else {
                name.contains(&pat)
            }
        });
    }

    if entries.is_empty() {
        println!("{}", "No matching entries.".dimmed());
        return Ok(());
    }

    if long {
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
                let r = (1.0 - e.compressed_size as f64 / e.uncompressed_size as f64) * 100.0;
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

fn cmd_extract(
    path: &PathBuf,
    output: Option<PathBuf>,
    files: Vec<String>,
    verbose: bool,
) -> Result<()> {
    let backend = archive::open_archive(path)?;
    let dest = output.unwrap_or_else(|| PathBuf::from("."));
    fs::create_dir_all(&dest)
        .with_context(|| format!("Cannot create directory: {}", dest.display()))?;
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

    let base_dir = if files.len() == 1 && files[0].is_dir() {
        files[0].parent().unwrap_or(&files[0]).to_path_buf()
    } else {
        std::env::current_dir()?
    };

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
    println!("  {}  {}", "Ratio:".bold(), overall_ratio.green());
    println!("  {}  {}", "Methods:".bold(), methods.join(", "));

    if encrypted_count > 0 {
        println!(
            "  {}  {} entries",
            "Encrypted:".bold(),
            encrypted_count.to_string().yellow()
        );
    }

    Ok(())
}

fn cmd_tree(path: &PathBuf, max_depth: Option<usize>, show_size: bool) -> Result<()> {
    let backend = archive::open_archive(path)?;
    let entries = backend.list_entries()?;

    let archive_name = path
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| path.display().to_string());
    println!("{}", archive_name.bold().cyan());

    // Build tree structure — synthesize directories from file paths
    let mut tree: HashMap<String, Vec<(String, bool, u64)>> = HashMap::new();
    let mut known_dirs: std::collections::HashSet<String> = std::collections::HashSet::new();

    // First pass: register all implicit directories
    for e in &entries {
        let parts: Vec<&str> = e.path.split('/').collect();
        for i in 1..parts.len() {
            let dir_path = parts[..i].join("/");
            known_dirs.insert(dir_path);
        }
    }

    // Add implicit directory entries
    for dir in &known_dirs {
        let parent = if dir.contains('/') {
            dir.rsplitn(2, '/').nth(1).unwrap_or("").to_string()
        } else {
            String::new()
        };
        let children = tree.entry(parent).or_default();
        if !children.iter().any(|(p, _, _)| p == dir) {
            children.push((dir.clone(), true, 0));
        }
    }

    // Add actual entries
    for e in &entries {
        let parent = if e.path.contains('/') {
            e.path.rsplitn(2, '/').nth(1).unwrap_or("").to_string()
        } else {
            String::new()
        };
        let children = tree.entry(parent).or_default();
        if !children.iter().any(|(p, _, _)| p == &e.path) {
            children.push((e.path.clone(), e.is_directory, e.uncompressed_size));
        }
    }

    // Sort children: dirs first, then alphabetical
    for children in tree.values_mut() {
        children.sort_by(|a, b| {
            b.1.cmp(&a.1)
                .then_with(|| a.0.to_lowercase().cmp(&b.0.to_lowercase()))
        });
    }

    let mut dir_count = 0usize;
    let mut file_count = 0usize;

    fn print_tree(
        tree: &HashMap<String, Vec<(String, bool, u64)>>,
        parent: &str,
        prefix: &str,
        max_depth: Option<usize>,
        current_depth: usize,
        show_size: bool,
        dir_count: &mut usize,
        file_count: &mut usize,
    ) {
        if let Some(max) = max_depth {
            if current_depth > max {
                return;
            }
        }

        if let Some(children) = tree.get(parent) {
            let count = children.len();
            for (i, (path, is_dir, size)) in children.iter().enumerate() {
                let is_last = i == count - 1;
                let connector = if is_last { "└── " } else { "├── " };
                let name = path.rsplitn(2, '/').next().unwrap_or(path);

                let size_str = if show_size && !is_dir {
                    format!(" ({})", format_size(*size)).dimmed().to_string()
                } else {
                    String::new()
                };

                if *is_dir {
                    *dir_count += 1;
                    println!(
                        "{}{}{}/{}",
                        prefix,
                        connector,
                        name.blue().bold(),
                        size_str
                    );
                    let new_prefix = format!("{}{}", prefix, if is_last { "    " } else { "│   " });
                    print_tree(
                        tree,
                        path,
                        &new_prefix,
                        max_depth,
                        current_depth + 1,
                        show_size,
                        dir_count,
                        file_count,
                    );
                } else {
                    *file_count += 1;
                    println!("{}{}{}{}", prefix, connector, name, size_str);
                }
            }
        }
    }

    print_tree(
        &tree,
        "",
        "",
        max_depth,
        0,
        show_size,
        &mut dir_count,
        &mut file_count,
    );

    println!();
    println!(
        "{} directories, {} files",
        dir_count.to_string().cyan(),
        file_count.to_string().cyan()
    );

    Ok(())
}

fn cmd_search(path: &PathBuf, pattern: &str, long: bool) -> Result<()> {
    let backend = archive::open_archive(path)?;
    let entries = backend.list_entries()?;
    let pat = pattern.to_lowercase();

    let matches: Vec<_> = entries
        .iter()
        .filter(|e| e.path.to_lowercase().contains(&pat))
        .collect();

    if matches.is_empty() {
        println!("{} No entries matching '{}'", "✗".red(), pattern);
        return Ok(());
    }

    println!(
        "Found {} matches for '{}':",
        matches.len().to_string().cyan(),
        pattern.yellow()
    );
    println!();

    if long {
        for e in &matches {
            let size_str = if e.is_directory {
                format!("{:>10}", "-".dimmed())
            } else {
                format!("{:>10}", format_size(e.uncompressed_size))
            };
            let modified = e.modified.as_deref().unwrap_or("-");
            let name = if e.is_directory {
                format!("{}/", e.path).blue().bold().to_string()
            } else {
                // Highlight the matching part
                highlight_match(&e.path, &pat)
            };
            println!("  {}  {:<16}  {}", size_str, modified, name);
        }
    } else {
        for e in &matches {
            if e.is_directory {
                println!("  {}/", e.path.blue());
            } else {
                println!("  {}", highlight_match(&e.path, &pat));
            }
        }
    }

    Ok(())
}

fn cmd_cat(path: &PathBuf, entry: &str) -> Result<()> {
    let tmp_dir = std::env::temp_dir().join("ziprs_cat");
    let _ = fs::remove_dir_all(&tmp_dir);
    fs::create_dir_all(&tmp_dir)?;

    let backend = archive::open_archive(path)?;
    let progress = CliProgress::new(false);
    backend.extract_entries(&[entry.to_string()], &tmp_dir, &progress)?;

    let extracted_path = tmp_dir.join(entry);
    if !extracted_path.exists() {
        anyhow::bail!("Entry not found: {entry}");
    }
    if extracted_path.is_dir() {
        anyhow::bail!("Cannot cat a directory: {entry}");
    }

    let mut file = fs::File::open(&extracted_path)?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;

    io::stdout().write_all(&buf)?;
    io::stdout().flush()?;

    let _ = fs::remove_dir_all(&tmp_dir);
    Ok(())
}

fn cmd_convert(input: &PathBuf, output: &PathBuf, verbose: bool) -> Result<()> {
    let src_format = ArchiveFormat::from_path(input)?;
    let dst_format = ArchiveFormat::from_path(output)?;

    if src_format == dst_format {
        anyhow::bail!("Source and destination formats are the same");
    }

    // Extract to temp dir, then create new archive
    let tmp_dir = std::env::temp_dir().join("ziprs_convert");
    let _ = fs::remove_dir_all(&tmp_dir);
    fs::create_dir_all(&tmp_dir)?;

    let progress = CliProgress::new(verbose);

    // Extract
    if verbose {
        eprintln!("Extracting {}...", input.display());
    }
    let backend = archive::open_archive(input)?;
    backend.extract_all(&tmp_dir, &progress)?;

    // Collect extracted files
    let mut files = Vec::new();
    collect_files_recursive(&tmp_dir, &mut files)?;

    // Create new archive
    if verbose {
        eprintln!("Creating {}...", output.display());
    }
    archive::create_archive(output, &files, &tmp_dir, dst_format, &progress)?;

    let src_size = fs::metadata(input)?.len();
    let dst_size = fs::metadata(output)?.len();

    let _ = fs::remove_dir_all(&tmp_dir);

    println!(
        "{} Converted {} ({}, {}) → {} ({}, {})",
        "✓".green().bold(),
        input
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .bold(),
        src_format.display_name(),
        format_size(src_size),
        output
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .bold(),
        dst_format.display_name().cyan(),
        format_size(dst_size),
    );

    Ok(())
}

fn cmd_diff(path_a: &PathBuf, path_b: &PathBuf) -> Result<()> {
    let backend_a = archive::open_archive(path_a)?;
    let backend_b = archive::open_archive(path_b)?;

    let entries_a = backend_a.list_entries()?;
    let entries_b = backend_b.list_entries()?;

    let map_a: HashMap<String, (u64, Option<String>)> = entries_a
        .iter()
        .map(|e| (e.path.clone(), (e.uncompressed_size, e.modified.clone())))
        .collect();
    let map_b: HashMap<String, (u64, Option<String>)> = entries_b
        .iter()
        .map(|e| (e.path.clone(), (e.uncompressed_size, e.modified.clone())))
        .collect();

    let name_a = path_a.file_name().unwrap_or_default().to_string_lossy();
    let name_b = path_b.file_name().unwrap_or_default().to_string_lossy();

    println!(
        "Comparing {} vs {}",
        name_a.bold().cyan(),
        name_b.bold().cyan()
    );
    println!();

    let mut added = 0;
    let mut removed = 0;
    let mut modified = 0;
    let mut identical = 0;

    // Files only in A (removed from B)
    for (path, (size_a, _)) in &map_a {
        if !map_b.contains_key(path) {
            removed += 1;
            println!(
                "  {} {} ({})",
                "-".red().bold(),
                path,
                format_size(*size_a).dimmed()
            );
        }
    }

    // Files only in B (added)
    for (path, (size_b, _)) in &map_b {
        if !map_a.contains_key(path) {
            added += 1;
            println!(
                "  {} {} ({})",
                "+".green().bold(),
                path,
                format_size(*size_b).dimmed()
            );
        }
    }

    // Files in both but different
    for (path, (size_a, mod_a)) in &map_a {
        if let Some((size_b, mod_b)) = map_b.get(path) {
            if size_a != size_b || mod_a != mod_b {
                modified += 1;
                let size_diff = *size_b as i64 - *size_a as i64;
                let diff_str = if size_diff > 0 {
                    format!("+{}", format_size(size_diff as u64)).red().to_string()
                } else if size_diff < 0 {
                    format!("-{}", format_size((-size_diff) as u64))
                        .green()
                        .to_string()
                } else {
                    "modified".yellow().to_string()
                };
                println!("  {} {} ({})", "~".yellow().bold(), path, diff_str);
            } else {
                identical += 1;
            }
        }
    }

    println!();
    println!(
        "{} added, {} removed, {} modified, {} identical",
        added.to_string().green(),
        removed.to_string().red(),
        modified.to_string().yellow(),
        identical.to_string().dimmed(),
    );

    Ok(())
}

fn cmd_stats(path: &PathBuf, top: usize) -> Result<()> {
    let backend = archive::open_archive(path)?;
    let entries = backend.list_entries()?;
    let file_size = fs::metadata(path)?.len();

    let files: Vec<_> = entries.iter().filter(|e| !e.is_directory).collect();
    let total_uncompressed: u64 = files.iter().map(|e| e.uncompressed_size).sum();
    let total_compressed: u64 = files.iter().map(|e| e.compressed_size).sum();

    // Size distribution
    let mut size_buckets = [0u64; 6]; // <1K, 1K-10K, 10K-100K, 100K-1M, 1M-10M, >10M
    for f in &files {
        let idx = match f.uncompressed_size {
            0..=1023 => 0,
            1024..=10239 => 1,
            10240..=102399 => 2,
            102400..=1048575 => 3,
            1048576..=10485759 => 4,
            _ => 5,
        };
        size_buckets[idx] += 1;
    }

    println!("{}", "Archive Statistics".bold().underline());
    println!();
    println!("  {}  {}", "Archive size:".bold(), format_size(file_size));
    println!(
        "  {}  {}",
        "Uncompressed:".bold(),
        format_size(total_uncompressed)
    );
    if total_uncompressed > 0 {
        let ratio = (1.0 - total_compressed as f64 / total_uncompressed as f64) * 100.0;
        println!(
            "  {}  {:.1}% saved",
            "Compression:".bold(),
            ratio.max(0.0)
        );
    }
    println!(
        "  {}  {} files, {} directories",
        "Entries:".bold(),
        files.len(),
        entries.len() - files.len()
    );
    if !files.is_empty() {
        let avg = total_uncompressed / files.len() as u64;
        println!(
            "  {}  {}",
            "Average size:".bold(),
            format_size(avg)
        );
    }

    // Size distribution
    println!();
    println!("{}", "Size Distribution".bold().underline());
    let labels = ["< 1 KiB", "1-10 KiB", "10-100 KiB", "100 KiB-1 MiB", "1-10 MiB", "> 10 MiB"];
    let max_count = *size_buckets.iter().max().unwrap_or(&1);
    let bar_width = 30;
    for (i, label) in labels.iter().enumerate() {
        let count = size_buckets[i];
        let bar_len = if max_count > 0 {
            (count as f64 / max_count as f64 * bar_width as f64) as usize
        } else {
            0
        };
        let bar = "█".repeat(bar_len);
        println!(
            "  {:>14}  {} {} {}",
            label.dimmed(),
            bar.cyan(),
            "░".repeat(bar_width - bar_len).dimmed(),
            count
        );
    }

    // Top files
    if !files.is_empty() {
        println!();
        println!(
            "{}",
            format!("Top {} Largest Files", top.min(files.len()))
                .bold()
                .underline()
        );
        let mut sorted: Vec<_> = files.iter().collect();
        sorted.sort_by(|a, b| b.uncompressed_size.cmp(&a.uncompressed_size));
        for (i, e) in sorted.iter().take(top).enumerate() {
            let pct = if total_uncompressed > 0 {
                e.uncompressed_size as f64 / total_uncompressed as f64 * 100.0
            } else {
                0.0
            };
            println!(
                "  {:>3}.  {:>10}  {:>5.1}%  {}",
                (i + 1).to_string().dimmed(),
                format_size(e.uncompressed_size).bold(),
                pct,
                e.path
            );
        }
    }

    Ok(())
}

fn cmd_types(path: &PathBuf) -> Result<()> {
    let backend = archive::open_archive(path)?;
    let entries = backend.list_entries()?;

    let files: Vec<_> = entries.iter().filter(|e| !e.is_directory).collect();
    let total_size: u64 = files.iter().map(|e| e.uncompressed_size).sum();

    // Group by extension
    let mut type_map: HashMap<String, (usize, u64)> = HashMap::new();
    for f in &files {
        let ext = f
            .name
            .rsplit('.')
            .next()
            .unwrap_or("(none)")
            .to_lowercase();
        let ext = if ext == f.name.to_lowercase() {
            "(no ext)".to_string()
        } else {
            format!(".{ext}")
        };
        let entry = type_map.entry(ext).or_insert((0, 0));
        entry.0 += 1;
        entry.1 += f.uncompressed_size;
    }

    let mut types: Vec<_> = type_map.into_iter().collect();
    types.sort_by(|a, b| b.1 .1.cmp(&a.1 .1));

    println!("{}", "File Type Breakdown".bold().underline());
    println!();
    println!(
        "  {:<12}  {:>6}  {:>10}  {:>6}  {}",
        "Extension".bold().underline(),
        "Count".bold().underline(),
        "Size".bold().underline(),
        "Share".bold().underline(),
        "".bold(),
    );

    let max_size = types.first().map(|t| t.1 .1).unwrap_or(1);
    for (ext, (count, size)) in &types {
        let pct = if total_size > 0 {
            *size as f64 / total_size as f64 * 100.0
        } else {
            0.0
        };
        let bar_len = if max_size > 0 {
            (*size as f64 / max_size as f64 * 20.0) as usize
        } else {
            0
        };
        let color = ext_color(ext);
        println!(
            "  {:<12}  {:>6}  {:>10}  {:>5.1}%  {}",
            ext.color(color).bold(),
            count,
            format_size(*size),
            pct,
            "█".repeat(bar_len).color(color),
        );
    }

    println!();
    println!(
        "  {} types, {} files, {} total",
        types.len().to_string().cyan(),
        files.len().to_string().cyan(),
        format_size(total_size).bold(),
    );

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

fn highlight_match(text: &str, pattern: &str) -> String {
    let lower = text.to_lowercase();
    if let Some(pos) = lower.find(pattern) {
        let before = &text[..pos];
        let matched = &text[pos..pos + pattern.len()];
        let after = &text[pos + pattern.len()..];
        format!("{}{}{}", before, matched.yellow().bold(), after)
    } else {
        text.to_string()
    }
}

fn ext_color(ext: &str) -> &'static str {
    match ext {
        ".rs" | ".go" | ".py" | ".js" | ".ts" | ".c" | ".cpp" | ".java" => "green",
        ".png" | ".jpg" | ".jpeg" | ".gif" | ".svg" | ".webp" => "magenta",
        ".mp4" | ".mkv" | ".avi" | ".mov" => "red",
        ".mp3" | ".wav" | ".flac" | ".ogg" => "cyan",
        ".zip" | ".tar" | ".gz" | ".bz2" | ".zst" => "yellow",
        ".html" | ".css" | ".json" | ".xml" | ".toml" | ".yaml" => "blue",
        ".md" | ".txt" | ".doc" | ".pdf" => "white",
        _ => "white",
    }
}
