use std::path::PathBuf;
use structopt::StructOpt;

/// This tool help you ignore files in your repositories locally
#[derive(StructOpt)]
#[structopt(name = "git-shadow")]
pub struct Opt {
    /// Verbose mode (-v, -vv, -vvv)
    #[structopt(short, long, parse(from_occurrences))]
    pub verbose: u8,

    ///
    #[structopt(subcommand)]
    pub cmd: OptCmd,
}

///
#[derive(StructOpt)]
#[structopt(name = "main")]
pub enum OptCmd {
    /// Shadow new file or folder
    Add {
        ///
        #[structopt(parse(from_os_str))]
        path: PathBuf,
    },
    /// Restore shadowed file or folder
    Restore {
        ///
        #[structopt(parse(from_os_str))]
        path: PathBuf,
    },
    /// List shadowed file or folder
    List,
    /// Open a list to select files to restore
    Manage,
}

/// Return command arguments
pub fn get_opt() -> Opt {
    Opt::from_args()
}
