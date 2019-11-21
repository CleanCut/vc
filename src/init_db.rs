//! Compare to C Git `builtin/init-db.c`

use crate::cache::{
    SharedRepo, DEFAULT_GIT_DIR_ENVIRONMENT, GIT_DIR_ENVIRONMENT, GIT_WORK_TREE_ENVIRONMENT,
};
use crate::sha1_file::safe_create_leading_directories;
use crate::usage::{die, usage};
use clap::ArgMatches;
use std::env;
use std::fs;
use std::path::PathBuf;

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
    let _real_git_dir: Option<PathBuf> = args
        .value_of("real_git_dir")
        .map(|x| PathBuf::from(x).canonicalize().unwrap());

    let _template_dir: Option<PathBuf> = args
        .value_of("template_dir")
        .map(|x| PathBuf::from(x).canonicalize().unwrap());

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

    // todo: See if we can simplify from here to line 87. C Git makes a round trip through the
    // environment, which seems like a big smell. It appears that the reason for the round trip in C
    // Git is it is relying on logic in setenv to only replace the value if the env var is not
    // already set.
    //
    // I'm not certain about the `args.is_present("directory")` logic. I'm inferring it from the
    // `argc > 0` bit of https://github.com/git/git/blob/efd54442381a2792186abc994060b8f7dd8b834b/builtin/init-db.c#L545
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
    let mut git_dir =
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

    // todo: continue from https://github.com/git/git/blob/efd54442381a2792186abc994060b8f7dd8b834b/builtin/init-db.c#L570

    Ok(())
}
