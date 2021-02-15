use crate::opt::Opt;
use structopt::StructOpt;
mod opt;
mod err;
mod core;

fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt);
}
