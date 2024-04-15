use std::fs::File;
use std::io::{self, BufRead, BufReader, Bytes, Read, Result};

use pdf_parser::tokenizer;
use pdf_parser::tokenizer::Token;

fn main() -> Result<()> {
    let mut file = File::open("./my_pdf.pdf")?;
    let mut buf: Vec<u8> = Vec::new();
    file.read_to_end(&mut buf).unwrap();
    
    let mut tokenizer = tokenizer::Tokenizer {
        cursor: 0,
        buffer: buf.as_slice()
    };

    for _ in 0..60 {
        let token = tokenizer.get_next_token();
        println!("{:?}", token);
    }


    Ok(())
}
