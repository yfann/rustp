use clap::Parser;
use std::fs::File;
use std::io::{self,BufRead,BufReader};

#[derive(Debug,Parser)]
#[command(author,version,about)]
struct Args{
    #[arg(value_name="FILE",default_value="-")]
    file: Vec<String>,
    #[arg(
        short('n'),
        long("number"),
        conflicts_with("number_nonblank_lines")
    )]
    number_lines: bool,

    #[arg(short('b'),long("number-nonblank"))]
    number_nonblank_lines: bool,
}

fn run(args: Args)->Result<(),Box<dyn std::error::Error>>{
    for filename in args.file{
        match open(&filename){
            Err(err)=>eprint!("Failed to open {filename}: {err}"),
            Ok(file)=>{
                let mut prev_num = 0;
                for (line_num,line) in file.lines().enumerate(){
                    let line=line?;
                    if args.number_lines{
                        println!("{:>6}\t{line}",line_num+1);
                    }else if args.number_nonblank_lines{
                        if line.is_empty(){
                            println!();
                        }else{
                            prev_num+=1;
                            println!("{prev_num:>6}\t{line}");
                        }
                    }
                    else{
                        print!("{line}");
                    }
                  
                }
            }
        }
    }
    Ok(())
}

fn main(){
    if let Err(e)=run(Args::parse()){
        eprintln!("{e}");
        std::process::exit(1);
    }
}

fn open(filename: &str)->Result<Box<dyn BufRead>,Box<dyn std::error::Error>>{
    match filename {
        "-"=>Ok(Box::new(BufReader::new(io::stdin()))),
        _=>Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}