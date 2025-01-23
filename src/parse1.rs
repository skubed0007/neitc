use crate::lexer::Tokens;
use colored::*;
use std::process::exit;

#[derive(Debug)]
pub enum ParseError {
    InvalidCharacter {
        line: usize,
        col: usize,
        code_line: String,
    },
    InvalidLibrary {
        line: usize,
        name: String,
        code_line: String,
    },
    UnexpectedToken {
        line: usize,
        token: String,
        code_line: String,
    },
    UnterminatedString {
        line: usize,
        code_line: String,
    },
    InvalidFunction {
        line: usize,
        name: String,
        code_line: String,
    },
    InvalidArgument {
        line: usize,
        expected: String,
        found: String,
        code_line: String,
        code: String,
    },
    NoCimport,
}

#[derive(Debug,PartialEq, Eq)]
pub enum AST {
    Cimport(String),
    CWrt(i32, String, i32),
}

struct ParseContext {
    errors: Vec<ParseError>,
    line: usize,
    col: usize,
}

impl ParseContext {
    fn new() -> Self {
        Self {
            errors: Vec::new(),
            line: 1,
            col: 0,
        }
    }

    fn add_error(&mut self, error: ParseError) {
        self.errors.push(error);
    }

    fn print_errors(&self, _code: &String) {
        for err in &self.errors {
            match err {
                ParseError::InvalidCharacter { line, col, code_line } => {
                    eprintln!(
                        "{}: {}\n{}",
                        "Error".red().bold(),
                        format!("Invalid character at line {}, col {}", line, col),
                        highlight_code(*line, col, code_line)
                    );
                }
                ParseError::InvalidLibrary { line, name, code_line } => {
                    eprintln!(
                        "{}: {}\n{}",
                        "Error".red().bold(),
                        format!("Invalid library '{}' at line {}", name, line),
                        highlight_code(*line, &0, code_line)
                    );
                }
                ParseError::UnexpectedToken { line, token, code_line } => {
                    eprintln!(
                        "{}: {}\n{}",
                        "Error".red().bold(),
                        format!("Unexpected token '{}' at line {}", token, line),
                        highlight_code(*line, &0, code_line)
                    );
                }
                ParseError::UnterminatedString { line, code_line } => {
                    eprintln!(
                        "{}: {}\n{}",
                        "Error".red().bold(),
                        format!("Unterminated string at line {}", line),
                        highlight_code(*line, &0, code_line)
                    );
                }
                ParseError::InvalidFunction { line, name, code_line } => {
                    eprintln!(
                        "{}: {}\n{}",
                        "Error".red().bold(),
                        format!("Invalid function '{}' at line {}", name, line),
                        highlight_code(*line, &0, code_line)
                    );
                }
                ParseError::InvalidArgument { line, expected, found, code_line, code } => {
                    eprintln!(
                        "{}: {}\n{}{}",
                        "Error".red().bold(),
                        format!(
                            "Expected '{}' but found '{}' at line {}",
                            expected, found, line
                        ),
                        highlight_code(*line, &0, code_line),
                        code
                    );
                }
                ParseError::NoCimport => {
                    eprintln!(
                        "{}: {}",
                        "Error".red().bold(),
                        format!("No import of cstd found")
                    );
                }
            }
        }
    }
}

pub fn parse(toks: &Vec<Tokens>, code: &String) -> Vec<AST> {
    let mut ctx = ParseContext::new();
    let mut ast = Vec::new();
    let mut tok_iter = toks.iter();

    while let Some(tok) = tok_iter.next() {
        ctx.line += 1;

        if let Tokens::Eof = tok {
            eprintln!(
                "{}{}{}",
                "Reached EOF at line: ".blue(),
                ctx.line,
                " (End of file reached)".blue().bold()
            );
            break;
        }

        eprintln!(
            "{}{}{}",
            "Parsing token: ".cyan(),
            format!("{:?}", tok).bold(),
            format!(" at line {}.", ctx.line).cyan()
        );

        match tok {
            Tokens::CImport => {
                parse_imports(&mut tok_iter, &mut ast, &mut ctx, code);
            }
            Tokens::Underscore => {
                parse_function(&mut tok_iter, &mut ast, &mut ctx, code);
            }
            _ => continue,
        }
    }

    if !ctx.errors.is_empty() {
        ctx.print_errors(code);
        exit(1);
    }
    ast
}

fn parse_imports(
    tok_iter: &mut std::slice::Iter<Tokens>,
    ast: &mut Vec<AST>,
    ctx: &mut ParseContext,
    code: &String,
) {
    let mut curlib = String::new();

    while let Some(citok) = tok_iter.next() {
        if let Tokens::Eof = citok {
            eprintln!(
                "{}{}{}",
                "Reached EOF while parsing import at line: ".blue(),
                ctx.line,
                " (End of file reached)".blue().bold()
            );
            break;
        }

        match citok {
            Tokens::Char(_, _, c) => curlib.push(*c),
            Tokens::Comma | Tokens::Eol => {
                if !curlib.is_empty() {
                    if curlib == "cstd" {
                        ast.push(AST::Cimport(curlib.clone()));
                    } else {
                        ctx.add_error(ParseError::InvalidLibrary {
                            line: ctx.line,
                            name: curlib.clone(),
                            code_line: code
                                .lines()
                                .nth(ctx.line - 1)
                                .unwrap_or_default()
                                .to_string(),
                        });
                    }
                    curlib.clear();
                }
                if matches!(citok, Tokens::Eol) {
                    break;
                }
            }
            Tokens::Space => continue,
            _ => {
                ctx.add_error(ParseError::InvalidCharacter {
                    line: ctx.line,
                    col: ctx.col,
                    code_line: code
                        .lines()
                        .nth(ctx.line - 1)
                        .unwrap_or_default()
                        .to_string(),
                });
            }
        }
    }
}

fn parse_function(
    tok_iter: &mut std::slice::Iter<Tokens>,
    ast: &mut Vec<AST>,
    ctx: &mut ParseContext,
    code: &String,
) {
    let mut fn_name = String::new();

    while let Some(tok) = tok_iter.next() {
        if let Tokens::Eof = tok {
            eprintln!(
                "{}{}{}",
                "Reached EOF while parsing function at line: ".blue(),
                ctx.line,
                " (End of file reached)".blue().bold()
            );
            break;
        }

        match tok {
            Tokens::Char(_, _, c) => fn_name.push(*c),
            Tokens::LSB => break,
            _ => continue,
        }
    }

    eprintln!(
        "{}{}{}",
        "Function parsed: ".green(),
        fn_name.bold(),
        " Proceeding to arguments.".green()
    );
    checkci(ast,ctx);
    match fn_name.trim() {
        "wrt" => parse_write(tok_iter, ast, ctx, code),
        _ => ctx.add_error(ParseError::InvalidFunction {
            line: ctx.line,
            name: fn_name,
            code_line: code
                .lines()
                .nth(ctx.line - 1)
                .unwrap_or_default()
                .to_string(),
        }),
    }
}
fn checkci(ast: &mut Vec<AST>, ctx: &mut ParseContext){
    let mut cif = false;
    for i in &mut *ast{
        if i == &AST::Cimport("cstd".to_string()){
            cif = true;
        }
    }
    if !cif{
        ctx.add_error(ParseError::NoCimport);
    }
}
fn parse_write(
    tok_iter: &mut std::slice::Iter<Tokens>,
    ast: &mut Vec<AST>,
    ctx: &mut ParseContext,
    code: &String,
) {
    
    let mut argv = Vec::new();
    let mut in_quotes = false;
    let mut quote_char = ' ';

    eprintln!("{}", "Parsing write arguments.".yellow());

    while let Some(tok) = tok_iter.next() {
        if let Tokens::Eof = tok {
            eprintln!(
                "{}{}{}",
                "Reached EOF while parsing write arguments at line: ".blue(),
                ctx.line,
                " (End of file reached)".blue().bold()
            );
            break;
        }

        match tok {
            Tokens::DQ | Tokens::SQ => {
                let current_quote = if matches!(tok, Tokens::DQ) { '"' } else { '\'' };
                if !in_quotes {
                    in_quotes = true;
                    quote_char = current_quote;
                } else if quote_char == current_quote {
                    in_quotes = false;
                }
                argv.push(current_quote);
            }
            Tokens::Char(_, _, c) => argv.push(*c),
            Tokens::Space => {
                if in_quotes {
                    argv.push(' ');
                }
            }
            Tokens::Comma => {
                argv.push(','); // Collect token
            }
            Tokens::Eol => break,
            _ => continue,
        }
    }

    if !argv.is_empty() {
        process_write_args(&argv, ast, ctx, code);
    }
}

fn process_write_args(argv: &Vec<char>, ast: &mut Vec<AST>, ctx: &mut ParseContext, code: &String) {
    let args = argv.iter().collect::<String>();
    let mut parts = Vec::new();
    let mut current_part = String::new();
    let mut in_quotes = false;
    let mut quote_char = ' ';
    //let inside_period = false;

    // Iterate through the string and split on commas unless inside quotes
    for c in args.chars() {
        if in_quotes {
            current_part.push(c);

            if c == quote_char {
                in_quotes = false;
            }
        } else {
            if c == '"' || c == '\'' {
                if !in_quotes {
                    in_quotes = true;
                    quote_char = c;
                }
                current_part.push(c);
            } else if c == ',' {
                // Only split on comma if we're not inside quotes
                if !in_quotes {
                    parts.push(current_part.trim().to_string());
                    current_part.clear();
                } else {
                    current_part.push(c);
                }
            } else {
                current_part.push(c);
            }
        }
    }

    // Push the last part
    if !current_part.is_empty() {
        parts.push(current_part.trim().to_string());
    }

    // Check if we have exactly 3 arguments, otherwise report an error
    if parts.len() != 3 {
        ctx.add_error(ParseError::InvalidArgument {
            line: ctx.line,
            expected: String::from("3 arguments"),
            found: parts.len().to_string(),
            code_line: code
                .lines()
                .nth(ctx.line - 1)
                .unwrap_or_default()
                .to_string(),
            code: code.clone(),
        });
        return;
    }

    // Check if the first argument is "stdout" or "stderr"
    let first = parts[0].trim().to_lowercase();
    let (stdout_stderr, text) = match first.as_str() {
        "stdout" => (1, parts[1].clone()),
        "stderr" => (0, parts[1].clone()),
        _ => {
            ctx.add_error(ParseError::InvalidArgument {
                line: ctx.line,
                expected: String::from("stdout or stderr"),
                found: first.to_string(),
                code_line: code
                    .lines()
                    .nth(ctx.line - 1)
                    .unwrap_or_default()
                    .to_string(),
                code: code.clone(),
            });
            return;
        }
    };

    // Check the third argument (should be an integer)
    let size_str = parts[2].trim().to_string();
    let size: i32 = match size_str.parse::<i32>() {
        Ok(parsed_size) => parsed_size,
        Err(_) => {
            ctx.add_error(ParseError::InvalidArgument {
                line: ctx.line,
                expected: String::from("integer"),
                found: size_str,
                code_line: code
                    .lines()
                    .nth(ctx.line - 1)
                    .unwrap_or_default()
                    .to_string(),
                code: code.clone(),
            });
            return;
        }
    };

    // Add the CWrt AST node
    ast.push(AST::CWrt(stdout_stderr, text, size));
}

fn highlight_code(line_num: usize, col_num: &usize, code_line: &str) -> String {
    format!(
        "{}{}\n{}{}{}",
        "Line:".blue(),
        line_num,
        "Code:".blue(),
        &code_line[..*col_num],
        format!("{}", &code_line[*col_num..]).red().bold()
    )
}
