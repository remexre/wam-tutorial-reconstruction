extern crate linefeed;
#[macro_use]
extern crate structopt;
extern crate wam_tutorial_reconstruction;

use std::path::PathBuf;

use linefeed::{ReadResult, Reader};
use structopt::StructOpt;

fn main() {
    let options = Options::from_args();
    let mut reader =
        Reader::new(Options::clap().get_name().to_string()).expect("Couldn't start the REPL");
    reader.set_prompt("?- ");
    std::env::home_dir()
        .and_then(|home| linefeed::inputrc::parse_file(&home.join(".inputrc")))
        .map(|ds| reader.evaluate_directives(ds));
    loop {
        match reader.read_line().expect("Couldn't read a line") {
            ReadResult::Eof => break,
            l => println!("{:?}", l),
        }
    }
}

/// A Rust implementation of the different machines introduced in Warren's
/// Abstract Machine: A Tutorial Reconstruction.
#[derive(Debug, StructOpt)]
#[structopt(name = "wam")]
struct Options {
    /// The subcommand to run.
    #[structopt(subcommand)]
    machine: Machine,

    /// An expression to evaluate. If not present, will start a REPL.
    #[structopt(short = "e", long = "eval")]
    expr: Option<String>,
}

/// A Prolog interpreter to run.
#[derive(Debug, StructOpt)]
enum Machine {
    /// The unification-only machine from chapter 2.
    #[structopt(name = "unification")]
    Unification {
        /// The file to read.
        #[structopt(name = "FILE", parse(from_os_str))]
        src_file: Option<PathBuf>,
    },
}

impl Machine {
    /// Returns the src_file argument.
    fn src_file(self) -> Option<PathBuf> {
        match self {
            Machine::Unification { src_file } => src_file,
        }
    }
}
