use std::path::PathBuf;

use clap::arg_enum;
use ratson::vm::Vm;
use structopt::StructOpt;

arg_enum! {
    #[derive(Debug)]
    enum Output {
        JSON,
    }
}

#[derive(Debug, StructOpt)]
#[structopt(name = "ratson", about = "test")]
struct Opt {
    #[structopt(name = "INPUT")]
    input: PathBuf,
    // #[structopt(possible_values = &Output::variants(), case_insensitive = true)]
    // output: Option<Output>,
}

fn main() {
    let opt = Opt::from_args();
    let input = opt.input;
    let input = std::fs::read(input).unwrap();

    let mut vm = Vm::new(&input);
    let ret = vm.run();

    let x = serde_json::to_string(&ret.unwrap()).unwrap();

    println!("{}", x);
}
