use std::{
    env::args,
    fs::{read_to_string, File, write},
    process::{exit, Command},
};

use gen::genc;
use lexer::lexer;
use parse1::parse;

pub mod doast;
pub mod gen;
pub mod lexer;
pub mod parse1;

fn print_help() {
    println!(
        "Usage: neitc <input_file> [options]\n\n\
        Options:\n\
        --bcompiler, -bc <compiler>   Specify the C compiler (default: clang)\n\
        --output, -o <file>           Specify the output file for the generated C code\n\
        --help, -h                    Show this help message\n"
    );
}

fn main() {
    let args = args().collect::<Vec<String>>();
    if args.len() < 2 {
        eprintln!("Error: Correct args not provided");
        print_help();
        exit(1);
    }
    let _cmd = &args[0];
    let file = &args[1];

    // Check for --help or -h flag
    if args.contains(&"--help".to_string()) || args.contains(&"-h".to_string()) {
        print_help();
        exit(0);
    }

    // Check if a custom compiler is specified
    let mut compiler = String::from("clang"); // Default compiler is clang
    if let Some(pos) = args.iter().position(|arg| arg == "--bcompiler" || arg == "-bc") {
        if pos + 1 < args.len() {
            compiler = args[pos + 1].clone();
        } else {
            eprintln!("Error: No compiler specified after '--bcompiler' or '-bc'");
            exit(1);
        }
    }

    // Check for --output or -o flag for the output C file name
    let mut output_file = String::from("output.c"); // Default output file name
    if let Some(pos) = args.iter().position(|arg| arg == "--output" || arg == "-o") {
        if pos + 1 < args.len() {
            output_file = args[pos + 1].clone();
        } else {
            eprintln!("Error: No output file specified after '--output' or '-o'");
            exit(1);
        }
    }

    match File::open(&file) {
        Ok(_) => match read_to_string(file) {
            Ok(code) => {
                let toks = lexer(&code);
                let ast = parse(&toks, &code);
                println!("AST:\n{:?}", ast);

                let ccode = genc(&ast);

                // Write the cleaned C code to the specified output file
                if let Err(e) = write(&output_file, ccode) {
                    eprintln!("Error writing C code to file: {}", e);
                    exit(1);
                }

                // Compile the C code using the specified compiler
                match Command::new(&compiler)
                    .arg(&output_file)
                    .arg("-o")
                    .arg("output") // Output binary
                    .output()
                {
                    Ok(output) => {
                        if !output.status.success() {
                            eprintln!(
                                "Error during compilation with {}: {}",
                                compiler,
                                String::from_utf8_lossy(&output.stderr)
                            );
                            exit(1);
                        } else {
                            println!("Compilation successful! Executable created as 'output'.");
                        }
                    }
                    Err(e) => {
                        eprintln!("Error running the compiler '{}': {}", compiler, e);
                        exit(1);
                    }
                }
            }
            Err(e) => {
                eprintln!(
                    "Error: Unable to read from source file! Exact error: {}",
                    e
                );
                exit(1);
            }
        },
        Err(e) => {
            eprintln!(
                "Error: Unable to open source file! Exact error: {}",
                e
            );
            exit(1);
        }
    }
}