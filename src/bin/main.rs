use std::fs;
use std::path::Path;

use dialoguer::{MultiSelect, Select};
use git::Git;
use git_shadow::*;
use log::trace;

fn main() -> git_shadow::Result<()> {
    let opt = arguments::get_opt();

    logging::init(opt.verbose);

    let repo = Git::new()?;

    match opt.cmd {
        arguments::OptCmd::Add { path } => {
            repo.state_clean()?;

            let path = repo.get_relative_path(&path)?;

            if !path.is_file() {
                return Err(err_msg!("Currently only support single file"));
            }

            let mut path_list = repo.get_local_ignore()?;

            path_list.push(path_to_string(&path)?);
            trace!("{:?}", path_list);

            repo.update_local_ignore(path_list)?;

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
            repo.state_clean()?;

            let path = repo.get_relative_path(&path)?;

            let mut path_list = repo.get_local_ignore()?;

            if !path_list.contains(&path_to_string(&path)?) {
                println!("Not shadowed");
                return Ok(());
            }

            path_list.swap_remove(
                path_list
                    .binary_search(&path_to_string(&path)?)
                    .expect("internal error: failed to remove from index"),
            );

            repo.remove_skip_worktree(path_to_string(&path)?)?;
            repo.restore_file(path_to_string(&path)?)?;

            repo.update_local_ignore(path_list)?;
        }
        arguments::OptCmd::List => {
            let path_list = repo.get_local_ignore()?;
            let mut show_list = vec!["PRESS ENTER TO EXIT".to_string()];

            for path in path_list {
                if path.starts_with('#') {
                    continue;
                }
                show_list.push(path);
            }

            Select::new()
                .items(&show_list)
                .default(0)
                .paged(true)
                .interact()?;
        }
        arguments::OptCmd::Manage => {
            repo.state_clean()?;

            let mut path_list = repo.get_local_ignore()?;

            let mut show_list = vec!["PRESS SPACE TO SELECT | PRESS ENTER TO CONFIRM".to_string()];

            for path in path_list.clone() {
                if path.starts_with('#') {
                    continue;
                }
                show_list.push(path);
            }

            let choosen = MultiSelect::new()
                .items(&show_list)
                .paged(true)
                .interact()?;

            for index in choosen {
                if index == 0 {
                    continue;
                }
                let path = show_list[index].clone();

                path_list.swap_remove(
                    path_list
                        .binary_search(&path)
                        .expect("internal error: failed to remove from index"),
                );

                repo.remove_skip_worktree(path.clone())?;
                repo.restore_file(path)?;
            }

            repo.update_local_ignore(path_list)?;
        }
    }

    Ok(())
}

fn path_to_string(path: &Path) -> Result<String> {
    match path.to_path_buf().into_os_string().into_string() {
        Ok(s) => Ok(s),
        Err(_) => Err(err_msg!("Unsupported path")),
    }
}
