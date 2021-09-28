use std::fs;

use git::Git;
use git_shadow::*;
use log::{error, trace};

fn main() -> git_shadow::Result<()> {
    let opt = arguments::get_opt();

    logging::init(opt.verbose);

    let repo = match Git::new() {
        Ok(repo) => repo,
        Err(e) => {
            error!("{}", e);
            return Ok(());
        }
    };

    match opt.cmd {
        arguments::OptCmd::Add { path } => {
            let path = path.canonicalize()?;
            repo.state_clean()?;
            if !path.is_file() {
                error!("Currently only support single file");
                return Ok(());
            }

            let mut paths = match repo.get_local_ignore() {
                Ok(p) => p,
                Err(e) => {
                    error!("{}", e);
                    return Ok(());
                }
            };

            paths.push(path.to_str().expect("Unsupported path").to_string());
            trace!("{:?}", paths);

            repo.update_local_ignore(paths)?;

            repo.add_skip_worktree(path.to_str().unwrap().to_string())?;

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
            repo.state_clean()?;
            // if !path.is_file() {
            //     error!("Currently only support single file");
            // }

            let mut paths = match repo.get_local_ignore() {
                Ok(p) => p,
                Err(e) => {
                    error!("{}", e);
                    return Ok(());
                }
            };

            if !paths.contains(&path.to_str().unwrap().to_string()) {
                println!("Not shadowed");
                return Ok(());
            }

            paths.swap_remove(
                paths
                    .binary_search(&path.to_str().unwrap().to_string())
                    .unwrap(),
            );

            repo.update_local_ignore(paths)?;

            repo.remove_skip_worktree(path.to_str().unwrap().to_string())?;

            repo.restore_file(path.to_str().unwrap().to_string())?;
        }
        arguments::OptCmd::List => error!("Currently unsupported"),
        arguments::OptCmd::Manage => error!("Currently unsupported"),
    }

    Ok(())
}
