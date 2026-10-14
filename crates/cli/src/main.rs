//! The `scribe` command.

use std::process::ExitCode;

use scribe_syntax::{LexOptions, lex};

const USAGE: &str = "usage: scribe lex [--oracle] [--elision] [--fragment] FILE...";

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("lex") => lex_command(&args[1..]),
        _ => {
            eprintln!("{USAGE}");
            ExitCode::from(2)
        }
    }
}

/// Print each file's tokens. `--oracle` prints them in the form
/// `tools/compare_lexers.py` compares against the reference grammar's lexer.
fn lex_command(args: &[String]) -> ExitCode {
    let mut options = LexOptions::default();
    let mut oracle = false;
    let mut files = Vec::new();
    for arg in args {
        match arg.as_str() {
            "--oracle" => oracle = true,
            "--elision" => options.elision = true,
            "--fragment" => options.fragment = true,
            flag if flag.starts_with("--") => {
                eprintln!("unknown option {flag}\n{USAGE}");
                return ExitCode::from(2);
            }
            file => files.push(file.to_string()),
        }
    }
    if files.is_empty() {
        eprintln!("{USAGE}");
        return ExitCode::from(2);
    }
    let mut failed = false;
    for file in &files {
        let source = match std::fs::read_to_string(file) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("{file}: {e}");
                failed = true;
                continue;
            }
        };
        if oracle {
            println!("== {file}");
        }
        match lex(&source, options) {
            Ok(tokens) => {
                for t in tokens {
                    if oracle {
                        println!("{}\t{}", t.kind.oracle_name(), t.text);
                    } else {
                        println!("{file}:{}:{}\t{}\t{}", t.line, t.column, t.kind.oracle_name(), t.text);
                    }
                }
            }
            Err(e) => {
                if oracle {
                    println!("error\t{}", e.message);
                } else {
                    eprintln!("{file}:{e}");
                    failed = true;
                }
            }
        }
    }
    if failed { ExitCode::FAILURE } else { ExitCode::SUCCESS }
}
