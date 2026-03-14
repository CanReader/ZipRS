#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    let args: Vec<String> = std::env::args().collect();

    // Check if a CLI subcommand is being used
    let has_subcommand = args.len() > 1
        && matches!(
            args[1].as_str(),
            "list" | "ls" | "extract" | "x" | "create" | "c" | "add" | "delete" | "rm"
                | "test" | "t" | "info" | "help" | "--help" | "-h" | "--version" | "-V"
        );

    if has_subcommand {
        // CLI mode
        use clap::Parser;
        use colored::Colorize;
        let cli = ziprs_lib::cli::Cli::parse();
        if let Err(e) = ziprs_lib::cli::run_cli(cli) {
            eprintln!("{} {e}", "error:".red().bold());
            std::process::exit(1);
        }
    } else {
        // GUI mode
        if std::env::var("WAYLAND_DISPLAY").is_ok() {
            std::env::set_var("GDK_BACKEND", "x11");
            std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");
        }
        ziprs_lib::run();
    }
}
