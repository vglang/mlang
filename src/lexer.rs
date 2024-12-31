//! Lexer for `mlang` program language.
//!

use std::{iter::Peekable, str::CharIndices};

use crate::errors::Error;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Token<'a> {
    /// identifier for nodes or filed.
    Ident(&'a str),
    LitNum(u32),
    /// keyword `tuple`
    Tuple,
    /// keyword `el`
    El,
    /// keyword `leaf`
    Leaf,
    /// keyword `attr`
    Attr,
    /// keyword `enum`
    Enum,
    /// keyword `data`
    Data,
    /// keyword `string`
    String,
    /// keyword `byte`
    Byte,
    /// keyword `ubyte`
    Ubyte,
    /// keyword `short`
    Short,
    /// keyword `ushort`
    Ushort,
    /// keyword `int`
    Int,
    /// keyword `uint`
    Uint,
    /// keyword `long`
    Long,
    /// keyword `ulong`
    Ulong,
    /// keyword `float`
    Float,
    /// keyword `double`
    Double,
    /// keyword `vec`
    Vec,
    /// keyword `->`
    ArrowRight,
    /// keyword `optional`
    Optional,
    /// keyword `variable`
    Variable,
    /// `{` (punct.)
    StartBlock,
    /// `}` (punct.)
    EndBlock,
    /// `(` (punct.)
    StartTuple,
    /// `)` (punct.)
    EndTuple,
    /// `,` (punct.)
    Comma,
    /// `:` (punct.)
    Colon,
    /// `;` (punct.)
    SemiColons,
    /// `#` (punct.)
    Pound,
    /// `[` (punct.)
    StartList,
    /// `]` (punct.)
    EndList,
}

pub struct Lexer<'a> {
    /// input source code.
    input: &'a str,
    /// inner char iterator.
    iter: Peekable<CharIndices<'a>>,
    /// the count of line
    lines: usize,
    /// The count of column.
    cols: usize,
}

impl<'a> From<&'a str> for Lexer<'a> {
    fn from(value: &'a str) -> Self {
        Self {
            iter: value.char_indices().peekable(),
            input: value,
            lines: 0,
            cols: 0,
        }
    }
}

impl<'a> Lexer<'a> {
    fn is_ws(c: char) -> bool {
        match c {
            '\n' | '\r' | '\t' | ' ' => true,
            _ => false,
        }
    }

    fn is_ident_char(c: char) -> bool {
        match c {
            '\n' | '\r' | '\t' | ' ' => false,
            '{' | '}' | '(' | ')' | ',' | ':' | '#' | '[' | ']' | ';' => false,
            _ => true,
        }
    }

    fn skip_ws(&mut self) {
        while let Some((_, c)) = self.iter.next_if(|(_, c)| Self::is_ws(*c)) {
            if c == '\n' {
                self.lines += 1;
                self.cols = 0;
            } else {
                self.cols += 1;
            }
        }
    }

    fn next_ident(&mut self, start: usize) -> usize {
        let mut end = start + 1;

        while let Some((offset, _)) = self.iter.next_if(|(_, c)| Self::is_ident_char(*c)) {
            self.cols += 1;
            end = offset + 1;
        }

        end
    }

    fn check_ident(ident: &str) -> Result<Token<'_>, Error> {
        assert!(!ident.is_empty());

        if ident.chars().next().unwrap().is_ascii_digit() {
            return Err(Error::Ident(ident));
        }

        return Ok(Token::Ident(ident));
    }

    fn next_lit_num(&mut self, start: usize) -> usize {
        let mut end = start + 1;

        while let Some((offset, _)) = self.iter.next_if(|(_, c)| c.is_ascii_digit()) {
            self.cols += 1;
            end = offset + 1;
        }

        end
    }

    pub fn position(&self) -> (usize, usize) {
        (self.lines, self.cols)
    }
}

impl<'a> Iterator for Lexer<'a> {
    /// lexer output item.
    type Item = Result<Token<'a>, Error<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let (offset, next) = self.iter.next()?;

            match next {
                '{' => {
                    self.cols += 1;
                    return Some(Ok(Token::StartBlock));
                }
                '}' => {
                    self.cols += 1;
                    return Some(Ok(Token::EndBlock));
                }
                '(' => {
                    self.cols += 1;
                    return Some(Ok(Token::StartTuple));
                }
                ')' => {
                    self.cols += 1;
                    return Some(Ok(Token::EndTuple));
                }
                ',' => {
                    self.cols += 1;
                    return Some(Ok(Token::Comma));
                }
                ':' => {
                    self.cols += 1;
                    return Some(Ok(Token::Colon));
                }
                '#' => {
                    self.cols += 1;
                    return Some(Ok(Token::Pound));
                }
                '[' => {
                    self.cols += 1;
                    return Some(Ok(Token::StartList));
                }
                ']' => {
                    self.cols += 1;
                    return Some(Ok(Token::EndList));
                }
                ';' => {
                    self.cols += 1;
                    return Some(Ok(Token::SemiColons));
                }
                '\r' | '\n' | '\t' | ' ' => {
                    if next == '\n' {
                        self.lines += 1;
                        self.cols = 0;
                    } else {
                        self.cols += 1;
                    }
                    self.skip_ws();
                }
                _ => {
                    // is lit number.
                    if next.is_ascii_digit() {
                        self.cols += 1;

                        let end = self.next_lit_num(offset);

                        let num = &self.input[offset..end];

                        return Some(Ok(Token::LitNum(u32::from_str_radix(num, 10).unwrap())));
                    }

                    let end = self.next_ident(offset);

                    let ident = &self.input[offset..end];

                    match ident {
                        "tuple" => return Some(Ok(Token::Tuple)),
                        "element" => return Some(Ok(Token::El)),
                        "leaf" => return Some(Ok(Token::Leaf)),
                        "attr" => return Some(Ok(Token::Attr)),
                        "enum" => return Some(Ok(Token::Enum)),
                        "data" => return Some(Ok(Token::Data)),
                        "string" => return Some(Ok(Token::String)),
                        "byte" => return Some(Ok(Token::Byte)),
                        "ubyte" => return Some(Ok(Token::Ubyte)),
                        "short" => return Some(Ok(Token::Short)),
                        "ushort" => return Some(Ok(Token::Ushort)),
                        "int" => return Some(Ok(Token::Int)),
                        "uint" => return Some(Ok(Token::Uint)),
                        "long" => return Some(Ok(Token::Long)),
                        "ulong" => return Some(Ok(Token::Ulong)),
                        "float" => return Some(Ok(Token::Float)),
                        "double" => return Some(Ok(Token::Double)),
                        "vec" => return Some(Ok(Token::Vec)),
                        "->" => return Some(Ok(Token::ArrowRight)),
                        "optional" => return Some(Ok(Token::Optional)),
                        "variable" => return Some(Ok(Token::Variable)),
                        _ => return Some(Self::check_ident(ident)),
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_lexer() {
        let lexer = Lexer::from(include_str!("../vglang.ml"));

        let tokens: Result<Vec<_>, Error> = lexer.collect();

        let token = tokens.unwrap();

        println!("{:?}", token);
    }
}
