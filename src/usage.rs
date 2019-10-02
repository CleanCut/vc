pub fn usage<T: AsRef(str)>(msg: T) {
    eprintln!("usage: {}", msg);
    std::process::exit(129);
}

pub fn die<T: AsRef(str)>(msg: T) {
    eprintln!("fatal: {}", msg);
    std::process::exit(128);
}

pub fn error<T: AsRef(str)>(msg: T) {
    eprintln!("error: {}", msg);
}

pub fn warning<T: AsRef(str)>(msg: T) {
    eprintln!("warning: {}", msg);
}
