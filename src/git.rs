use crate::*;
use git2::{Repository, RepositoryState, StatusOptions};
use log::{debug, trace};
use std::{
    fs::OpenOptions,
    io::{BufRead, BufReader, BufWriter, Write},
    path::Path,
    process::Command,
};

///
pub struct Git {
    repo: Repository,
}

impl Git {
    /// find repository in current and parent directory
    pub fn new() -> Result<Self> {
        let mut repo_dir = std::env::current_dir()?;
        while Repository::open(repo_dir.clone()).is_err() {
            if !repo_dir.pop() {
                return Err(err_msg!(
                    "not a git repository (or any of the parent directories)",
                ));
            }
        }
        Ok(Self {
            repo: Repository::open(repo_dir)?,
        })
    }

    /// Return the path to `.git` folder
    pub fn path(&self) -> &Path {
        self.repo.path()
    }

    /// Return repository state
    pub fn state_clean(&self) -> Result<()> {
        if self.repo.state() != RepositoryState::Clean {
            Err(err_msg!("Repository state not clean"))
        } else {
            Ok(())
        }
    }

    /// modified & deleted & added file list
    pub fn get_uncommitted_files(&self) -> Result<Vec<String>> {
        if self.repo.state() != RepositoryState::Clean {
            Err(err_msg!("Repository state not clean"))
        } else {
            let mut opt = StatusOptions::new();
            opt.show(git2::StatusShow::IndexAndWorkdir);
            let statuses = self.repo.statuses(Some(&mut opt))?;

            debug!("uncommitted file number: {}", statuses.len());

            let mut res: Vec<String> = Vec::new();
            for stat in statuses.iter() {
                res.push(
                    stat.path()
                        .expect("Unable convert file path to str")
                        .to_string(),
                );
                trace!("uncommitted file: {}", res[res.len() - 1]);
            }
            Ok(res)
        }
    }

    /// Read `.git/info/exclude`
    pub fn get_local_ignore(&self) -> Result<Vec<String>> {
        let exclude = OpenOptions::new()
            .read(true)
            .create(false)
            .open(self.repo.path().join("info/exclude"))?;
        let exclude = BufReader::new(exclude);

        let mut res = Vec::new();

        for line in exclude.lines().flatten() {
            res.push(line)
        }

        Ok(res)
    }

    /// Clear & Write `.git/info/exclude`
    pub fn update_local_ignore(&self, paths: Vec<String>) -> Result<()> {
        self.state_clean()?;

        let exclude = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(self.repo.path().join("info/exclude"))?;
        let mut exclude = BufWriter::new(exclude);
        let mut index = self.repo.index()?;

        for path in paths {
            if let Some(c) = path.trim().chars().next() {
                if c != '#' {
                    debug!("modify index {}", path);
                    index.remove_path(Path::new(&path))?;
                }
            }
            exclude.write_fmt(format_args!("{}\n", path))?;
        }

        self.repo.set_index(&mut index)?;

        Ok(())
    }

    ///
    pub fn add_skip_worktree(&self, path: String) -> Result<()> {
        let output = Command::new("git")
            .args(["update-index", "--skip-worktree", &path])
            .output()?;

        if output.stdout.len() > 0 {
            return Err(err_msg!("{:?}", output.stdout));
        }

        Ok(())
    }

    ///
    pub fn remove_skip_worktree(&self, path: String) -> Result<()> {
        let output = Command::new("git")
            .args(["update-index", "--no-skip-worktree", &path])
            .output()?;

        if output.stdout.len() > 0 {
            return Err(err_msg!("{:?}", output.stdout));
        }

        Ok(())
    }

    ///
    pub fn restore_file(&self, path: String) -> Result<()> {
        let output = Command::new("git")
            .args(["checkout", "--", &path])
            .output()?;

        if output.stdout.len() > 0 {
            return Err(err_msg!("{:?}", output.stdout));
        }

        Ok(())
    }
}
