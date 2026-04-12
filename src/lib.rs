pub fn find() -> String {
    if let Ok(ok) = std::env::var("CLANG") {
        return iter(vec![ok]);
    }
    if let Ok(ok) = std::env::var("CLANG_VERSION") {
        return iter(vec![format!("clang-{}", ok)]);
    }
    return iter(vec![
        "clang".to_string(),
        "clang-30".to_string(),
        "clang-29".to_string(),
        "clang-28".to_string(),
        "clang-27".to_string(),
        "clang-26".to_string(),
        "clang-25".to_string(),
        "clang-24".to_string(),
        "clang-23".to_string(),
        "clang-22".to_string(),
        "clang-21".to_string(),
        "clang-20".to_string(),
        "clang-19".to_string(),
        "clang-18".to_string(),
        "clang-17".to_string(),
    ]);
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
