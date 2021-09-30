use std::fs;
use std::path::PathBuf;

use git::Git;
use git_shadow::*;
use log::{debug, error, trace};
use npath::{NormPathBufExt, NormPathExt};

fn main() -> git_shadow::Result<()> {
    let opt = arguments::get_opt();

    logging::init(opt.verbose);

    let repo = Git::new()?;

    match opt.cmd {
        arguments::OptCmd::Add { path } => {
            repo.state_clean()?;

            let path = path.absolute()?;
            if !path.starts_with(repo.path().parent().unwrap()) {
                debug!("required: {:?}, root: {:?}", path, repo.path());
                return Err(err_msg!("Check your path!"));
            }

            if !path.is_file() {
                return Err(err_msg!("Currently only support single file"));
            }

            let path_str = repo.get_relative_path_string(&path)?;

            let mut path_list = repo.get_local_ignore()?;

            path_list.push(path_str.clone());
            trace!("{:?}", path_list);

            repo.update_local_ignore(path_list)?;

            repo.add_skip_worktree(path_str)?;

            if path.is_dir() {
                fs::remove_dir(path)?;
            } else if path.is_file() {
                fs::remove_file(path)?;
            } else {
                println!("Shadow completed but Nothing deleted")
            }
        }
        arguments::OptCmd::Restore { path } => {
            repo.state_clean()?;

            let path = path.absolute()?;
            if !path.starts_with(repo.path().parent().unwrap()) {
                return Err(err_msg!("Check your path!"));
            }

            let path_str = repo.get_relative_path_string(&path)?;
            trace!("path_str {}", path_str);

            let mut path_list = repo.get_local_ignore()?;

            if !path_list.contains(&path_str) {
                println!("Not shadowed");
                return Ok(());
            }

            path_list.swap_remove(path_list.binary_search(&path_str).unwrap());

            repo.update_local_ignore(path_list)?;

            repo.remove_skip_worktree(path_str.clone())?;

            repo.restore_file(path_str)?;
        }
        arguments::OptCmd::List => error!("Currently unsupported"),
        arguments::OptCmd::Manage => error!("Currently unsupported"),
    }

    Ok(())
}
