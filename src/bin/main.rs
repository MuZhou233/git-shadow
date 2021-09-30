use std::fs;
use std::path::PathBuf;

use git::Git;
use git_shadow::*;
use log::{error, trace};

fn main() -> git_shadow::Result<()> {
    let opt = arguments::get_opt();

    logging::init(opt.verbose);

    let repo = Git::new()?;

    match opt.cmd {
        arguments::OptCmd::Add { path } => {
            let path = path.canonicalize()?;
            if !path.starts_with(repo.path()) {
                return Err(err_msg!("Check your path!"));
            }

            repo.state_clean()?;

            if !path.is_file() {
                return Err(err_msg!("Currently only support single file"));
            }

            let mut paths = repo.get_local_ignore()?;

            paths.push(path_to_string(&path)?);
            trace!("{:?}", paths);

            repo.update_local_ignore(paths)?;

            repo.add_skip_worktree(path_to_string(&path)?)?;

            if path.is_dir() {
                fs::remove_dir(path)?;
            } else if path.is_file() {
                fs::remove_file(path)?;
            } else {
                println!("Shadow completed but Nothing deleted")
            }
        }
        arguments::OptCmd::Restore { path } => {
            let path = path.canonicalize()?;
            if !path.starts_with(repo.path()) {
                return Err(err_msg!("Check your path!"));
            }

            repo.state_clean()?;

            let mut paths = repo.get_local_ignore()?;

            if !paths.contains(&path_to_string(&path)?) {
                println!("Not shadowed");
                return Ok(());
            }

            paths.swap_remove(paths.binary_search(&path_to_string(&path)?).unwrap());

            repo.update_local_ignore(paths)?;

            repo.remove_skip_worktree(path_to_string(&path)?)?;

            repo.restore_file(path_to_string(&path)?)?;
        }
        arguments::OptCmd::List => error!("Currently unsupported"),
        arguments::OptCmd::Manage => error!("Currently unsupported"),
    }

    Ok(())
}

fn path_to_string(path: &PathBuf) -> Result<String> {
    match path.clone().into_os_string().into_string() {
        Ok(s) => Ok(s),
        Err(_) => Err(err_msg!("Unsupported path")),
    }
}
