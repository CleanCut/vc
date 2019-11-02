//! Compare to `builtin/init-db.c`

use crate::cache::SharedRepo;
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

    if args.is_present("bare") {
        env::set_var("GIT_DIR_ENVIRONMENT", env::current_dir()?);
    }

    // I traced quite a bit of logic related to this value in the C code, and I think I have
    // managed to contain that logic entirely with SharedRepo::from()
    let shared_repo = SharedRepo::from(args.value_of("shared_repo"));


    // todo: continue translation from
    // https://github.com/github/git/blob/81ed82f5e00b805039bd79e36c74a56533211cd0/builtin/init-db.c#L552

    Ok(())
}
