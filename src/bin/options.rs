use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

use failure::Error;
use wam_tutorial_reconstruction::*;
use wam_tutorial_reconstruction::common::*;

/// A Rust implementation of the different machines introduced in Warren's
/// Abstract Machine: A Tutorial Reconstruction.
#[derive(Debug, StructOpt)]
#[structopt(name = "wam")]
pub struct Options {
    /// The subcommand to run.
    #[structopt(subcommand)]
    pub machine: MachineOpts,

    /// An expression to evaluate. If not present, will start a REPL.
    #[structopt(short = "e", long = "eval")]
    pub expr: Option<String>,
}

/// A Prolog interpreter to run.
#[derive(Debug, StructOpt)]
pub enum MachineOpts {
    /// The unification-only machine from chapter 2.
    #[structopt(name = "unification")]
    Unification {
        /// The file to read. Should contain a single term.
        #[structopt(name = "FILE", parse(from_os_str))]
        src_file: PathBuf,
    },
}

impl MachineOpts {
    /// Creates a new instance of the specified machine.
    pub fn new_machine(&self) -> Result<Box<Machine>, Error> {
        match *self {
            MachineOpts::Unification { ref src_file } => {
                let mut program = read_src_file(src_file)?;
                if program.len() != 1 {
                    bail!("M0 doesn't support more than one clause in the program.");
                }
                let Clause(head, body) = program.remove(0);
                if !body.is_empty() {
                    bail!("M0 doesn't support implications.");
                }
                Ok(Box::new(unification::Machine::new(head)))
            }
        }
    }
}

fn read_src_file<P: AsRef<Path>>(path: P) -> Result<Vec<Clause>, Error> {
    let src = File::open(path).and_then(|mut file| {
        let mut buf = String::new();
        file.read_to_string(&mut buf).map(|_| buf)
    })?;
    ParseError::from_iresult(parsers::program(&src), &src).map_err(Into::into)
}
