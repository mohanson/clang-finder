pub fn find() -> String {
    if let Ok(clang) = std::env::var("CLANG") {
        return iter(vec![clang]);
    }
    return iter(
        (17..=30)
            .rev()
            .map(|ver| format!("clang-{}", ver))
            .collect::<Vec<String>>(),
    );
}

pub fn iter(array: Vec<String>) -> String {
    for bin in array {
        if let Ok(ok) = show(&bin) {
            print!("{}", String::from_utf8_lossy(&ok.stdout));
            return bin;
        }
    }
    panic!("No clang compiler found");
}

pub fn show(clang: &str) -> std::io::Result<std::process::Output> {
    std::process::Command::new(clang).arg("--version").output()
}
