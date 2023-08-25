use clap::Parser;

#[derive(Parser)]
#[clap()]
struct Args {
    #[clap(short, long, default_value = "demo.eth")]
    wallet: String,
    #[clap(short, long, default_value_t = 5)]
    coins: usize,
}

fn main() {

    let args = Args::parse();

    let var = distinct_cqt_query::print(args.wallet, args.coins);

    for (key, value) in var.iter() {
        println!("{}: {}", key.clone().unwrap(), value.clone().unwrap());
    }
}
