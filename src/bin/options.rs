use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

use failure::Error;
use log::LevelFilter;
use wam_tutorial_reconstruction::*;
use wam_tutorial_reconstruction::common::*;

/// A Rust implementation of the different machines introduced in Warren's
/// Abstract Machine: A Tutorial Reconstruction.
#[derive(Debug, StructOpt)]
#[structopt(name = "wam",
            raw(setting = "::structopt::clap::AppSettings::ColoredHelp"))]
pub struct Options {
    /// The subcommand to run.
    #[structopt(subcommand)]
    pub machine: MachineOpts,

    /// An expression to evaluate. If not present, will start a REPL.
    #[structopt(short = "e", long = "eval")]
    pub expr: Option<String>,

    /// Turns off message output.
    #[structopt(short = "q", long = "quiet")]
    pub quiet: bool,

    /// Increases the verbosity. Default verbosity is errors only.
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    pub verbose: u8,
}

impl Options {
    /// Returns the log level filter specified by the `-q` and `-v` options.
    pub fn verbosity(&self) -> LevelFilter {
        if self.quiet {
            LevelFilter::Off
        } else {
            match self.verbose {
                0 => LevelFilter::Error,
                1 => LevelFilter::Warn,
                2 => LevelFilter::Info,
                3 => LevelFilter::Debug,
                _ => LevelFilter::Trace,
            }
        }
    }
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

    /// The fact-unifying machine from chapter 2.
    #[structopt(name = "facts")]
    Facts {
        /// The file to read. Should contain only facts, with up to one fact
        /// per functor.
        #[structopt(name = "FILE", parse(from_os_str))]
        src_file: PathBuf,
    },

    /// The flat resolution machine from chapter 3.
    #[structopt(name = "flat")]
    Flat {
        /// The file to read. Can contain facts or rules, with up to one clause
        /// per functor.
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
                    bail!("M0 only supports one clause in the program.");
                }
                let Clause(head, body) = program.remove(0);
                if !body.is_empty() {
                    bail!("M0 doesn't support implications.");
                }
                Ok(Box::new(unification::Machine::new(Term::Structure(head))))
            }
            MachineOpts::Facts { ref src_file } => {
                let mut program = read_src_file(src_file)?;
                let facts = program
                    .into_iter()
                    .map(|Clause(head, body)| {
                        if !body.is_empty() {
                            bail!("M1 doesn't support implications.");
                        }
                        Ok(head)
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(Box::new(facts::Machine::new(facts)))
            }
            MachineOpts::Flat { ref src_file } => {
                let program = read_src_file(src_file)?;
                Ok(Box::new(flat::Machine::new(program)))
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
