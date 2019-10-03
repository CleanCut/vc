use crate::environment::{get_shared_repository, set_shared_repository};
use crate::usage::{die, usage};
use clap::ArgMatches;
use std::path::PathBuf;
use std::env;
use std::fs;
use crate::sha1_file::safe_create_leading_directories;
use std::fs::Permissions;

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

    // todo: Implement ::from() method on SharedRepo and convert this into one of the variants.
    let shared_repo = args.value_of("shared_repo").unwrap_or("false");

    let directory: PathBuf = directory.unwrap();

    let mut mkdir_tried = false;
    loop {
        if env::set_current_dir(&directory).is_ok() {
            break;
        }
        if mkdir_tried {
            die(format!("cannot chdir to {}", directory.to_str().unwrap()));
        }
        let saved = get_shared_repository();
        set_shared_repository(0);
        if safe_create_leading_directories(&directory).is_err() {
            die(format!("cannot mkdir {}", directory.to_str().unwrap()));
        }
        set_shared_repository(saved);
        let mut perms = fs::metadata(&directory)?.permissions();
        if fs::create_dir(&directory).is_err() || fs::set_permissions(&directory, perms).is_err() {
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
