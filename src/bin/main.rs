//! A REPL and interpreter for the various machines in "Warren's Abstract
//! Machine: A Tutorial Reconstruction."

#[macro_use]
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
    match run(options) {
        Ok(()) => {}
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    }
}

fn run(options: Options) -> Result<(), Error> {
    let mut machine = options.machine.new_machine()?;
    if let Some(expr) = options.expr {
        run_query(&mut *machine, &expr, || true)
    } else {
        let mut reader = Reader::new(Options::clap().get_name().to_string())?;
        std::env::home_dir()
            .and_then(|home| {
                let inputrc = home.join(".inputrc");
                if inputrc.exists() {
                    linefeed::inputrc::parse_file(&inputrc)
                } else {
                    None
                }
            })
            .map(|ds| reader.evaluate_directives(ds));

        let mut query_buf = String::new();
        loop {
            // Read a line of input.
            reader.set_prompt(if query_buf.len() == 0 { "?- " } else { "   " });
            match reader.read_line().expect("Couldn't read a line") {
                ReadResult::Eof => break Ok(()),
                ReadResult::Input(s) => {
                    query_buf += &s;
                    reader.add_history(s);
                }
                ReadResult::Signal(sig) => unimplemented!("{:?}", sig),
            };

            // Try running the query.
            let keep_going = || true;
            match run_query(&mut *machine, &query_buf, keep_going) {
                Ok(()) => {
                    query_buf.clear();
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

fn run_query<F: FnMut() -> bool>(
    m: &mut Machine,
    q: &str,
    mut keep_going: F,
) -> Result<(), Error> {
    let query = ParseError::from_iresult(parsers::query(q), q)?;
    let mut iter = m.run_query(query);

    let mut written = 0;
    loop {
        let bindings = if let Some(r) = iter.next() {
            let b = r?;
            if written != 0 {
                println!(";");
            }
            b
        } else {
            if written != 0 {
                println!(".");
            }
            break;
        };

        let mut first = true;
        for (var, val) in bindings {
            if first {
                first = false;
            } else {
                print!(",\n");
            }
            print!("{} = {}", var, val);
        }
        written += 1;

        if !keep_going() {
            println!(".");
            break;
        }
    }

    if written == 0 {
        println!("true.");
    }
    Ok(())
}
