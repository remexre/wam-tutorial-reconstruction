use std::path::PathBuf;

use wam_tutorial_reconstruction::*;

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
        /// The file to read.
        #[structopt(name = "FILE", parse(from_os_str))]
        src_file: Option<PathBuf>,
    },
}

impl MachineOpts {
    /// Creates a new instance of the specified machine.
    pub fn new_machine(&self) -> Box<Machine> {
        match *self {
            MachineOpts::Unification { .. } => {
                let code = Vec::new(); // TODO
                Box::new(unification::Machine::new(code))
            }
        }
    }

    /// Returns the src_file argument.
    pub fn src_file(self) -> Option<PathBuf> {
        match self {
            MachineOpts::Unification { src_file } => src_file,
        }
    }
}
