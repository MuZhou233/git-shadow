# git-shadow

A command line tool that helps you ignore/restore files locally in git repository

⚠️WARNING: Unstable, use for function test only⚠️

## Get started

1. Download binary from [release](https://github.com/muzhou233/git-shadow/releases/latest)
2. Put binary to `/usr/local/bin` and comfirm file name is `git-shadow`
3. Now you can try following commands

```shell
mkdir test_repo
cd test_repo
git init
touch test1 test2
git add test1 test2
git commit -m "test"
git shadow add test1
git status
ls
git shadow restore test1
git status
ls
```