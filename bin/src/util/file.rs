use std::{
    fs::File,
    io::{self, Read},
    path::PathBuf,
};

use super::path::get_project_root;

pub fn get_solution_by_id(id: &str) -> io::Result<PathBuf> {
    let root_path = get_project_root()?;
    let file_name = format!("{}/solution/solution{}.rs", root_path.to_str().unwrap(), id);

    Ok(PathBuf::from(file_name))
}

pub fn get_solution_lib() -> io::Result<PathBuf> {
    let root_path = get_project_root()?;
    let file_name = format!("{}/solution/lib.rs", root_path.to_str().unwrap());

    Ok(PathBuf::from(file_name))
}

pub fn get_slug_by_solution_id(id: &str) -> io::Result<String> {
    let file_name = get_solution_by_id(id)?;
    let mut solution_file = File::open(file_name)?;
    let mut buf = String::new();
    solution_file.read_to_string(&mut buf)?;

    let result = buf.split('\n').nth(1).unwrap();
    let prefix = "///";

    Ok(result[result.find(prefix).unwrap() + prefix.len() + 1..].to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_solution_by_id() {
        let result = get_solution_by_id("1").unwrap();
        assert!(result.to_str().unwrap().contains("solution/solution1.rs"));
    }

    #[test]
    fn test_get_solution_lib() {
        let result = get_solution_lib().unwrap();
        assert!(result.to_str().unwrap().contains("solution/lib.rs"));
    }

    #[test]
    fn test_get_slug_by_solution_file() {
        let result = get_slug_by_solution_id("2951").unwrap();

        assert_eq!(result, "find-the-peaks".to_string());
    }
}
