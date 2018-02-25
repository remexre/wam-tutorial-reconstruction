//! A REPL and interpreter for the various machines in "Warren's Abstract
//! Machine: A Tutorial Reconstruction."

extern crate failure;
extern crate linefeed;
extern crate nom;
#[macro_use]
extern crate structopt;
extern crate wam_tutorial_reconstruction;

mod options;

use failure::Error;
use linefeed::{ReadResult, Reader};
use structopt::StructOpt;
use wam_tutorial_reconstruction::*;
use wam_tutorial_reconstruction::common::*;

use options::Options;

fn main() {
    let options = Options::from_args();
    let machine = options.machine.new_machine();
    if let Some(expr) = options.expr {
        match run_query(&*machine, &expr) {
            Ok(result) => unimplemented!("{:?}", result),
            Err(err) => {
                eprintln!("{}", err);
                std::process::exit(1);
            }
        }
    } else {
        let mut reader = Reader::new(Options::clap().get_name().to_string())
            .expect("Couldn't start the REPL");
        std::env::home_dir()
            .and_then(|home| {
                linefeed::inputrc::parse_file(&home.join(".inputrc"))
            })
            .map(|ds| reader.evaluate_directives(ds));

        let mut query_buf = String::new();
        loop {
            // Read a line of input.
            reader.set_prompt(if query_buf.len() == 0 { "?- " } else { "   " });
            match reader.read_line().expect("Couldn't read a line") {
                ReadResult::Eof => break,
                ReadResult::Input(s) => {
                    query_buf += &s;
                    reader.add_history(s);
                }
                ReadResult::Signal(sig) => unimplemented!("{:?}", sig),
            };

            // Try running the query.
            match run_query(&*machine, &query_buf) {
                Ok(result) => {
                    query_buf.clear();
                    unimplemented!("{:?}", result)
                }
                Err(err) => match err.downcast::<ParseError>() {
                    Ok(ParseError::Incomplete(_)) => continue,
                    Ok(err) => {
                        query_buf.clear();
                        eprintln!("{}", err);
                    }
                    Err(err) => {
                        query_buf.clear();
                        eprintln!("{}", err);
                    }
                },
            }
        }
    }
}

fn run_query(m: &Machine, q: &str) -> Result<(), Error> {
    let query = ParseError::from_iresult(parsers::query(q), q)?;
    m.run_query(query)
}
