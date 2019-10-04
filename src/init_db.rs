//! Compare to `builtin/init-db.c`

use crate::cache::SharedRepo;
use crate::environment::{get_shared_repository, set_shared_repository};
use crate::sha1_file::safe_create_leading_directories;
use crate::usage::{die, usage};
use clap::ArgMatches;
use std::env;
use std::fs;
use std::fs::Permissions;
// this adds the ability to create/modify Permissions, but restricts compatibility to unix-like os's
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

/// Compare to `builtin/init-db.c :: cmd_init_db`
///
/// Handles cli `git init` command.
pub fn cmd_init_db(args: &ArgMatches) -> Result<(), std::io::Error> {
    let real_git_dir: Option<PathBuf> = args
        .value_of("real_git_dir")
        .map(|x| PathBuf::from(x).canonicalize().unwrap());

    let template_dir: Option<PathBuf> = args
        .value_of("template_dir")
        .map(|x| PathBuf::from(x).canonicalize().unwrap());

    let directory: Option<PathBuf> = args.value_of("directory").map(|x| PathBuf::from(x));
    if directory.is_none() {
        usage("vc init [-q | --quiet] [--bare] [--template=<template-directory>] [--shared[=<permissions>]] [<directory>]")
    }
    let directory: PathBuf = directory.unwrap(); // guaranteed not to panic

    let shared_repo = SharedRepo::from(args.value_of("shared_repo"));
    let perms = Permissions::from_mode(shared_repo.as_permissions());

    let mut mkdir_tried = false;
    loop {
        if env::set_current_dir(&directory).is_ok() {
            break;
        }
        if mkdir_tried {
            die(format!("cannot chdir to {}", directory.to_str().unwrap()));
        }
        // todo: this noop saving/restoring is likely completely unnecessary since we're not using
        // mutable global state like git is in C.  Probably remove this when implementing retrieving
        // config values.
        let saved = get_shared_repository();
        set_shared_repository(0);
        if safe_create_leading_directories(&directory).is_err() {
            die(format!("cannot mkdir {}", directory.to_str().unwrap()));
        }
        set_shared_repository(saved);

        if fs::create_dir(&directory).is_err() || fs::set_permissions(&directory, perms.clone()).is_err() {
            die(format!("cannot mkdir {}", directory.to_str().unwrap()));
        }
        mkdir_tried = true;
    }

    if args.is_present("bare") {
        env::set_var("GIT_DIR_ENVIRONMENT", env::current_dir()?);
    }

    if args.is_present("shared") {
        // See init-db.c:549
    }

    Ok(())
}
