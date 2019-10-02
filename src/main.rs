use clap::{App, Arg, SubCommand};

fn main() {
    let args = App::new("vc")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
//        .subcommand(SubCommand::with_name("add"))
//        .subcommand(SubCommand::with_name("cat-file"))
//        .subcommand(SubCommand::with_name("checkout"))
//        .subcommand(SubCommand::with_name("commit"))
//        .subcommand(SubCommand::with_name("hash-object"))
        .subcommand(SubCommand::with_name("init")
            .about("Create an empty Git repository or reinitialize an existing one")
            .arg(Arg::with_name("quiet")
                .short("q")
                .long("quiet")
                .help("Only print error and warning messages; all other output will be suppressed."))
            .arg(Arg::with_name("bare")
                .long("bare")
                .help("Create a bare repository. If GIT_DIR environment is not set, it is set to the current working directory."))
            .arg(Arg::with_name("template_dir")
                .long("template")
                .takes_value(true)
                .value_name("template_directory")
                .help("Specify the directory from which templates will be used. (See the \"TEMPLATE DIRECTORY\" section below."))
            .arg(Arg::with_name("real_git_dir")
                .long("separate-git-dir")
                .takes_value(true)
                .value_name("git dir")
                .help("Instead of initializing the repository as a directory to either $GIT_DIR or ./.git/, create a text file there containing the path to the actual repository. This file acts as filesystem-agnostic Git symbolic link to the repository.\n\nIf this is reinitialization, the repository will be moved to the specified path."))
// todo: convert from string values to int
            .arg(Arg::with_name("shared")
                .long("shared")
                .takes_value(true)
                .value_name("(false|true|umask|group|all|world|everybody|0xxx)")
                .possible_values(&["false", "true", "umask", "group", "all", "world", "everybody", "0xxx"])
                .default_value("group")
                .help("\
Specify that the Git repository is to be shared amongst several users. This allows
users belonging to the same group to push into that repository. When specified, the
config variable \"core.sharedRepository\" is set so that files and directories under
$GIT_DIR are created with the requested permissions. When not specified, Git will use
permissions reported by umask(2).

The option can have the following values, defaulting to group if no value is given:

umask (or false)
   Use permissions reported by umask(2). The default, when --shared is not specified.

group (or true)
   Make the repository group-writable, (and g+sx, since the git group may be not the
   primary group of all users). This is used to loosen the permissions of an
   otherwise safe umask(2) value. Note that the umask still applies to the other
   permission bits (e.g. if umask is 0022, using group will not remove read
   privileges from other (non-group) users). See 0xxx for how to exactly specify the
   repository permissions.

all (or world or everybody)
   Same as group, but make the repository readable by all users.

0xxx
   0xxx is an octal number and each file will have mode 0xxx.  0xxx will override
   users' umask(2) value (and not only loosen permissions as group and all does).
   0640 will create a repository which is group-readable, but not group-writable or
   accessible to others.  0660 will create a repo that is readable and writable to
   the current user and group, but inaccessible to others."))
            .arg(Arg::with_name("directory")
                .help("If you provide a directory, the command is run inside it. If this directory does not exist, it will be created.")))
//        .subcommand(SubCommand::with_name("log"))
//        .subcommand(SubCommand::with_name("ls-tree"))
//        .subcommand(SubCommand::with_name("merge"))
//        .subcommand(SubCommand::with_name("rebase"))
//        .subcommand(SubCommand::with_name("rev-parse"))
//        .subcommand(SubCommand::with_name("rm"))
//        .subcommand(SubCommand::with_name("show-ref"))
//        .subcommand(SubCommand::with_name("tag"))
        .get_matches();

    match args.subcommand() {
        ("init", Some(subcommand_args)) => vc::init_db::cmd_init_db(subcommand_args),
        _ => println!("unhandled"),
    }
}
