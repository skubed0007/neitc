use crate::parse1::AST;

#[allow(unused)]
/// Parse the AST and generate C Code out of it
/// 
/// # Parameters
/// - `&Vec<AST>` : Vector of retrieved AST returned from `parse()` function
/// 
/// # Returns
/// - `String` : The generated C code
pub fn genc(ast: &Vec<AST>) -> String {
    let mut c_code = String::new();
    let mut imports = String::new();
    let mut mainf = String::new();
    let mut sidef = String::new();
    
    // Main function header
    mainf.push_str("int main(int argc, char const *argv[]) {\n");

    // Iterate over AST nodes to generate code
    for item in ast {
        match item {
            AST::Cimport(lib) => match lib.as_str() {
                "cstd" => {
                    // Include standard libraries and definitions for stdout and stderr
                    imports.push_str("#include <unistd.h>\nint STDOUT = 0;\nint STDERR = 1;\n");
                    sidef.push_str("int count(const char *str) {\n    int c = 0;\n    while (*str) {\n        c += 1;\n    }\n    return c;\n}\n");
                }
                _ => {}
            },
            AST::CWrt(stream, text, size) => {
                // Format write function call with proper indentation
                mainf.push_str(&format!(
                    "    write({}, \"{}\", {});\n",
                    stream, trim_quotes(text), size
                ));
            }
            _ => {}
        }
    }

    // Closing the main function
    mainf.push_str("\n}");

    // Combine the parts to form the full C code
    c_code.push_str(&imports);    // Import section
    c_code.push_str("\n");        // Newline for separation
    c_code.push_str(&sidef);      // Side functions like count()
    c_code.push_str("\n");        // Newline for separation
    c_code.push_str(&mainf);      // Main function

    c_code.push_str("\n");

    c_code
}

/// Helper function to trim quotes (both single and double) from a string
fn trim_quotes(input: &str) -> String {
    input.trim_matches(|c| c == '"' || c == '\'').to_string()
}
