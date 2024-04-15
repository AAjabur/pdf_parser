use std::{fs::{self, File}, io::{self, BufReader, Read}, str::Bytes};

pub struct Tokenizer<'a> {
    pub cursor: usize,
    pub buffer: &'a [u8],
}

#[derive(Debug)]
pub enum Token {
    IndirectObjectDefinitionStart,
    IndirectObjectDefinitionEnd,
    IndirectObjectReference,

    DictStart,
    DictEnd,

    ListStart,
    ListEnd,

    Null,
    Boolean(bool),
    IntegerNumber(i32),
    FloatNumber(f32),
    Name(String),
    String(String),
    Stream
}

impl Tokenizer<'_> {
    pub fn get_next_token(&mut self) -> Token{
        let mut token = String::new();

        self.ignore_useless_chars();

        let mut is_integer = true;
        while self.curr_byte().is_ascii_digit() {
            token.push(self.curr_byte() as char);
            if self.next_byte() == b'.' {
                self.cursor += 2;
                is_integer = false;
                token.push('.');
            } else if !self.next_byte().is_ascii_digit() && is_integer {
                self.cursor += 1;
                return Token::IntegerNumber(token.parse::<i32>().unwrap());
            }  else if !self.next_byte().is_ascii_digit() {
                self.cursor += 1;
                return Token::FloatNumber(token.parse::<f32>().unwrap());
            } else {
                self.cursor += 1;
            }
        }

        while(self.curr_byte().is_ascii_alphabetic()){
            token.push(self.curr_byte() as char);

            if !self.next_byte().is_ascii_alphabetic() {
                self.cursor += 1;

                match &token[..] {
                    "obj" => return Token::IndirectObjectDefinitionStart,
                    "endobj" => return Token::IndirectObjectDefinitionEnd,
                    "stream" => {
                        
                    },
                    _ => return Token::String(token)
                }
            }
            self.cursor += 1;
        }

        if self.curr_byte() == b'<' && self.next_byte() == b'<' {
            self.cursor += 2;
            return Token::DictStart;
        }
        if self.curr_byte() == b'>' && self.next_byte() == b'>' {
            self.cursor += 2;
            return Token::DictEnd;
        }

        if self.curr_byte() == b'/' && self.next_byte().is_ascii_alphabetic() {
            self.cursor += 1;
            while self.curr_byte() != b' ' {
                token.push(self.curr_byte() as char);
                self.cursor += 1;
            }
            self.cursor += 1;
            return Token::Name(token);
        }

        if self.curr_byte() == b'[' {
            self.cursor += 1;
            return Token::ListStart;
        }

        if self.curr_byte() == b']' {
            self.cursor += 1;
            return Token::ListEnd;
        }

        return Token::Null;

    }

    fn goto_word(&mut self, word: &[u8]){
        for (i, window) in self.buffer[self.cursor..].windows(word.len()).enumerate() {
            if window == word {
                self.cursor += i;
            }
        }
    }

    fn ignore_useless_chars (&mut self){
        loop {
            if !is_ascii(self.curr_byte()) {
                self.cursor += 1;
                continue;
            }

            if self.curr_byte().is_ascii_whitespace() {
                self.cursor += 1;
                continue;
            }

            if self.curr_byte() == b'\n' {
                self.cursor += 1;
                continue;
            }

            if self.curr_byte() == b'%' {
                while self.curr_byte() != b'\n' {
                    self.cursor += 1;
                }
                continue;
            }

            break;
        }
    }

    fn curr_byte(&self) -> u8 {
        return self.buffer[self.cursor];
    }

    fn next_byte(&self) -> u8 {
        return self.buffer[self.cursor + 1];
    }

    fn next_nth_byte(&self, nth: usize) -> u8{
        return self.buffer[self.cursor + nth]
    }
}

fn is_ascii(byte: u8) -> bool{
    byte <= 127
}