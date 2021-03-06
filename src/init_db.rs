//! Compare to C Git `builtin/init-db.c`

#![allow(unused)]

use crate::cache::{
    SharedRepo, DEFAULT_GIT_DIR_ENVIRONMENT, GIT_DIR_ENVIRONMENT, GIT_WORK_TREE_ENVIRONMENT,
};
use crate::sha1_file::safe_create_leading_directories;
use crate::usage::{die, usage};
use clap::ArgMatches;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

fn init_db(
    git_dir: String,
    real_git_dir: PathBuf,
    template_dir: PathBuf,
    quiet: bool,
    exists_ok: bool,
) -> Result<(), std::io::Error> {
    Ok(())
}

fn guess_repository_type(git_dir: &str) -> Result<bool, std::io::Error> {
    /*
     * "GIT_DIR=. git init" is always bare.
     * "GIT_DIR=`pwd` git init" too.
     */
    if git_dir == "." {
        return Ok(true);
    }
    if git_dir == env::current_dir()?.to_str().unwrap_or_default() {
        return Ok(true);
    }
    /*
     * "GIT_DIR=.git or GIT_DIR=something/.git is usually not.
     */
    
    if git_dir == ".git" {
        return Ok(false);
    }
    if git_dir.ends_with("/.git") {
        return Ok(false);
    }

    /*
     * Otherwise it is often bare.  At this point
     * we are just guessing.
     */
    Ok(true)
}

/*
 * If you want to, you can share the DB area with any number of branches.
 * That has advantages: you can save space by sharing all the SHA1 objects.
 * On the other hand, it might just make lookup slower and messier. You
 * be the judge.  The default case is to have one DB per managed directory.
 */
/// Compare to `builtin/init-db.c` `cmd_init_db`
///
/// Handles cli `git init` command.
pub fn cmd_init_db(args: &ArgMatches) -> Result<(), std::io::Error> {
    let real_git_dir: &str = args.value_of("real_git_dir").unwrap_or_default();
    let real_git_dir: PathBuf = PathBuf::from(real_git_dir).canonicalize()?;

    let template_dir: &str = args.value_of("template_dir").unwrap_or_default();
    let template_dir: PathBuf = PathBuf::from(template_dir).canonicalize()?;

    let directory: Option<PathBuf> = args.value_of("directory").map(PathBuf::from);
    if directory.is_none() {
        usage("vc init [-q | --quiet] [--bare] [--template=<template-directory>] [--shared[=<permissions>]] [<directory>]")
    }
    let directory: PathBuf = directory.unwrap(); // guaranteed not to panic

    let mut mkdir_tried = false;
    loop {
        if env::set_current_dir(&directory).is_ok() {
            break;
        }
        if mkdir_tried {
            die(format!("cannot chdir to {}", directory.to_str().unwrap()));
        }
        if safe_create_leading_directories(&directory).is_err() {
            die(format!("cannot mkdir {}", directory.to_str().unwrap()));
        }

        // C's mkdir from libc takes the passed-in mode and masks it with the current process's
        // umask. C Git passes in 0777 to mkdir to enable all of the bits that don't get masked out,
        // which is what fs::create_dir() appears to do by default.
        if fs::create_dir(&directory).is_err() {
            die(format!("cannot mkdir {}", directory.to_str().unwrap()));
        }
        mkdir_tried = true;
    }

    // todo: See if we can simplify from here to line 90. C Git makes a round trip through the
    // environment, which seems like a big smell. It appears that the reason for the round trip in C
    // Git is it is relying on logic in setenv to only replace the value if the env var is not
    // already set.  But maybe something down the line needs to inherit the environment (?)
    if args.is_present("bare")
        && (args.is_present("directory") || env::var(GIT_DIR_ENVIRONMENT).is_err())
    {
        // By this time, we have chdir'd into the "directory" arg.
        // todo: refactor so the logic isn't so deviously clever that you can't understand it.
        env::set_var(GIT_DIR_ENVIRONMENT, env::current_dir()?);
    }

    // I traced quite a bit of logic related to this value in the C code, and I think I have
    // managed to contain that logic entirely with SharedRepo::from()
    let _shared_repo = SharedRepo::from(args.value_of("shared_repo"));

    /*
     * GIT_WORK_TREE makes sense only in conjunction with GIT_DIR
     * without --bare.  Catch the error early.
     */
    let git_dir =
        env::var(GIT_DIR_ENVIRONMENT).unwrap_or_else(|_| DEFAULT_GIT_DIR_ENVIRONMENT.to_string());
    let work_tree = env::var(GIT_WORK_TREE_ENVIRONMENT).unwrap_or_default();
    // todo: `|| args.is_present("bare")` will *never* be true if we reach it, because if it were
    //       true then we would have set `git_dir` to non-empty via line 53.  Remove it once the
    //       line 50 todo has been completed.
    if (git_dir.is_empty() || args.is_present("bare")) && !work_tree.is_empty() {
        die(format!(
            "{} (or --work-tree=<directory>) not allowed without
             specifying {} (or --git-dir=<directory>)",
            GIT_WORK_TREE_ENVIRONMENT, GIT_DIR_ENVIRONMENT
        ));
    }

    /*
     * Set up the default .git directory contents
     */
    let is_bare_repository_cfg = if args.is_present("bare") {
        true
    } else {
        guess_repository_type(&git_dir)?
    };

    // todo: continue from https://github.com/git/git/blob/efd54442381a2792186abc994060b8f7dd8b834b/builtin/init-db.c#L573
    if !is_bare_repository_cfg {
        let git_work_tree_cfg = if let Some(slash_idx) = git_dir.rfind('/') {
            PathBuf::from_str(git_dir.split_at(slash_idx).0)
                .unwrap() // Given .rfind() succeeded, this should not be able to crash
                .canonicalize()?
        } else {
            env::current_dir()?
        };
    // todo: In C, set_git_work_tree() sets the global `git_work_tree` and then calls
    //       repo_set_worktree() to set `.worktree` on the global repo struct.  Since none of
    //       those globals are used yet, I'm not yet certain what to do here.
    /* Block-commented-out because clippy doesn't like identical branches
            if !work_tree.is_empty() {
                // set_git_work_tree(work_tree);
            } else {
                // set_git_work_tree(git_work_tree_config);
            }
    */
    } else if !work_tree.is_empty() {
        // set_git_work_tree(work_tree);
    }

    // Combined into "flags" in C
    let quiet = args.is_present("quiet");
    let exists_ok = true;

    init_db(git_dir, real_git_dir, template_dir, quiet, exists_ok)
}
