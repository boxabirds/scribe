//! The `scribe` command.

use std::process::ExitCode;

use scribe_syntax::{LexOptions, ast, lex, parse};

const USAGE: &str = "usage: scribe lex [--oracle] [--elision] [--fragment] FILE...\n       scribe parse [--oracle] [--fragment] FILE...\n       scribe impact OLD NEW [--dependents FILE...]";

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("lex") => lex_command(&args[1..]),
        Some("parse") => parse_command(&args[1..]),
        Some("impact") => impact_command(&args[1..]),
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

/// Parse each file. `--oracle` prints each tree in the form
/// `tools/compare_parsers.py` compares against the reference grammar's parse.
fn parse_command(args: &[String]) -> ExitCode {
    let mut options = LexOptions::default();
    let mut oracle = false;
    let mut files = Vec::new();
    for arg in args {
        match arg.as_str() {
            "--oracle" => oracle = true,
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
        let result = lex(&source, options)
            .map_err(|e| format!("{}:{}: {}", e.line, e.column, e.message))
            .and_then(|tokens| parse(&tokens).map_err(|e| e.to_string()));
        match result {
            Ok(tree) => {
                if oracle {
                    println!("== {file}\n{}", ast::to_sexpr(&tree));
                } else {
                    println!("{file}: ok");
                }
            }
            Err(message) => {
                if oracle {
                    println!("== {file}\nerror {message}");
                } else {
                    eprintln!("{file}:{message}");
                    failed = true;
                }
            }
        }
    }
    if failed { ExitCode::FAILURE } else { ExitCode::SUCCESS }
}

fn parse_file(path: &str) -> Result<ast::File, String> {
    let source = std::fs::read_to_string(path).map_err(|e| format!("{path}: {e}"))?;
    let tokens = lex(&source, LexOptions::default()).map_err(|e| format!("{path}:{e}"))?;
    parse(&tokens).map_err(|e| format!("{path}:{e}"))
}

/// Compare two versions of a module and print what the change could break
/// (specification, Section 12). `--dependents` names other modules' sources,
/// read for what they require.
fn impact_command(args: &[String]) -> ExitCode {
    let (versions, dependents) = match args.iter().position(|a| a == "--dependents") {
        Some(i) => (&args[..i], &args[i + 1..]),
        None => (args, &args[args.len()..]),
    };
    let [old, new] = versions else {
        eprintln!("{USAGE}");
        return ExitCode::from(2);
    };
    let files: Result<Vec<ast::File>, String> =
        [old, new].into_iter().chain(dependents).map(|p| parse_file(p)).collect();
    let files = match files {
        Ok(f) => f,
        Err(e) => {
            eprintln!("{e}");
            return ExitCode::FAILURE;
        }
    };
    let label = |p: &str| {
        std::path::Path::new(p).file_name().map_or(p.to_string(), |n| n.to_string_lossy().into_owned())
    };
    let report = scribe_impact::compare(&files[0], &files[1], &files[2..]);
    print!("{}", report.render(&label(old), &label(new)));
    ExitCode::SUCCESS
}
