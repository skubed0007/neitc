#[derive(Debug, Eq, PartialEq)]
pub enum Tokens {
    ///C Import
    CImport,
    ///Underscore - `_`
    Underscore,
    ///cimport libs names
    Libs(String),
    ///unknown thingy??
    /// holds
    /// - `i32` -> char position number
    /// - `i32` -> line number
    /// - `String` -> word
    Char(i32, i32, char),
    ///Left small bracket
    LSB,
    ///right small bracket
    RSB,
    ///Left Cursly Brace
    LCurlyB,
    ///Right Curly Brace,
    RCurlyBrace,
    ///Left Big Bracket
    LBBracket,
    ///Right Bug Bracket
    RBBracket,
    ///Space ' '
    Space,
    ///End of File
    Eof,
    ///end of line
    Eol,
    ///Comma `,`
    Comma,
    ///Semicolon `;`
    SColon,
    ///double quotes - `"`
    DQ,
    ///single quotes - `'`
    SQ,
    
}

pub fn lexer(code: &String) -> Vec<Tokens> {
    let mut code = code.trim().to_string();
    code.push(' ');
    let mut toks: Vec<Tokens> = Vec::new();
    //for line in code.lines(){
    let mut wrd = String::new();
    let (mut charpos, mut linec) = (0, 0);
    for char in code.chars() {
        charpos += 1;

        //println!("[DEBUG] char -> {char} | charpos -> {} | linec -> {}",charpos,linec);
        match char {
            '"' => {
                toks.push(Tokens::DQ);
            }
            '\'' => {
                toks.push(Tokens::SQ);
            }
            '\n' => {
                if !wrd.is_empty(){
                toks.push(checkwrd(&wrd));
                wrd.clear();
                }
                toks.push(Tokens::Eol);
                charpos = 0;
                linec += 1;
            }
            ' ' => {
                if !wrd.is_empty(){
                    toks.push(checkwrd(&wrd));
                    wrd.clear();
                }
                toks.push(Tokens::Space);
                wrd.clear();
            }
            '(' => {
                toks.push(Tokens::LSB);
            }
            ')' => {
                toks.push(Tokens::RSB);
            }
            '{' => {
                toks.push(Tokens::LCurlyB);
            }
            '}' => {
                toks.push(Tokens::RCurlyBrace);
            }
            '[' => {
                toks.push(Tokens::LBBracket);
            }
            ']' => {
                toks.push(Tokens::RBBracket);
            }
            ';' => {
                toks.push(Tokens::SColon);
                wrd.clear();
            }
            ',' => {
                toks.push(Tokens::Comma);
            }
            '_' => {
                toks.push(Tokens::Underscore);
            }
            _ => {
                wrd.push(char);
                toks.push(Tokens::Char(charpos, linec, char));
            }
        }
    }
    toks.push(Tokens::Eol);
    toks.push(Tokens::Eof);
    return toks;
}
fn checkwrd(wrd: &String) -> Tokens {
    match wrd.trim() {
        "cimport" => Tokens::CImport,
        _ => {
            Tokens::Space
        },
    }
}
