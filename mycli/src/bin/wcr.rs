
use std::io::{self, BufRead, BufReader};
use std::fs::File;
use clap::Parser;


#[derive(Debug, Parser)]
#[command(author,version,about)]
struct Args{
    #[arg(value_name="FILE",default_value="-")]
    files: Vec<String>,
    #[arg(short,long)]
    lines: bool,
    #[arg(short,long)]
    words: bool,
    #[arg(short('c'),long)]
    bytes: bool,
    #[arg(short('m'),long,conflicts_with("bytes"))]
    chars: bool,
}

fn run(mut args:Args)->Result<(),Box<dyn std::error::Error>>{
    if [args.words,args.bytes,args.chars,args.lines].iter().all(|v| v==&false){
        args.lines=true;
        args.words=true;
        args.bytes=true;
    }
    Ok(())
}

fn open(filename: &str)->Result<Box<dyn BufRead>,Box<dyn std::error::Error>>{
    match filename{
        "-"=>Ok(Box::new(BufReader::new(io::stdin()))),
        _=>Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}