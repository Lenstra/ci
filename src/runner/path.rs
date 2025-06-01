use ignore::Walk;
use std::error::Error;

pub fn find(patterns: &[&str]) -> Result<Vec<std::path::PathBuf>, Box<dyn Error + Send + Sync>> {
    let mut files = Vec::new();
    let matchers: Vec<_> = patterns
        .iter()
        .map(|p| glob::Pattern::new(p))
        .collect::<Result<_, _>>()?;

    for entry in Walk::new(".").filter_map(|e| e.ok()) {
        let filename = entry.file_name().to_str().unwrap_or("");
        let path = entry.path();
        if matchers.iter().any(|pattern| pattern.matches(filename)) {
            files.push(path.to_owned());
        }
    }
    Ok(files)
}

pub fn dir(
    result: Result<Vec<std::path::PathBuf>, Box<dyn Error + Send + Sync>>,
) -> Result<Vec<std::path::PathBuf>, Box<dyn Error + Send + Sync>> {
    if let Err(e) = result {
        return Err(e);
    }
    let mut dirs = Vec::new();
    let paths = result.unwrap();
    for path in paths {
        dirs.push(path.parent().unwrap().to_owned());
    }
    Ok(dirs)
}
