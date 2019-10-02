use crate::fatal;
use std::path::PathBuf;

const DEFAULT_GIT_TEMPLATE_DIR: &str = "/usr/share/git-core/templates";
const INIT_IS_BARE_REPOSITORY: i32 = 0;
const INIT_SHARED_REPOSITORY: i32 = -1;

pub struct Repository {
    worktree: PathBuf,
    gitdir: PathBuf,
}

impl Repository {
    pub fn new<T: Into<PathBuf>>(worktree: T, create_new: bool) -> Repository {
        let worktree = worktree.into();
        let gitdir = worktree.join(".git");
        if !gitdir.is_dir() {
            fatal(format!(
                "Not a Git repository {}",
                worktree.to_str().unwrap()
            ));
        }
        Repository { worktree, gitdir }
    }
    //    /// Create
    //    pub fn create<T: Into<PathBuf>>(worktree: T) -> Repository {
    //
    //    }
}
