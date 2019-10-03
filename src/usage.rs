pub fn usage<T: Into<String>>(msg: T) {
    eprintln!("usage: {}", msg.into());
    std::process::exit(129);
}

pub fn die<T: Into<String>>(msg: T) {
    eprintln!("fatal: {}", msg.into());
    std::process::exit(128);
}

pub fn error<T: Into<String>>(msg: T) {
    eprintln!("error: {}", msg.into());
}

pub fn warning<T: Into<String>>(msg: T) {
    eprintln!("warning: {}", msg.into());
}
