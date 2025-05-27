use clap::Parser;

#[derive(Parser)]
struct Args {
    #[clap(short, long)]
    name: String,
}

fn main() {
    let args = Args::parse();
    println!("Hello, {}!", args.name);
}
