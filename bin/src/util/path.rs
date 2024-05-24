use std::{
    env,
    ffi::OsString,
    fs::read_dir,
    io::{self, ErrorKind},
    path::PathBuf,
};

pub fn get_project_root() -> io::Result<PathBuf> {
    let path = env::current_dir()?;
    let mut path_ancestors = path.as_path().ancestors();

    while let Some(p) = path_ancestors.next() {
        let has_cargo = read_dir(p)?
            .into_iter()
            .any(|p| p.unwrap().file_name() == OsString::from("Cargo.lock"));
        if has_cargo {
            return Ok(PathBuf::from(p));
        }
    }
    Err(io::Error::new(
        ErrorKind::NotFound,
        "Ran out of places to find Cargo.toml",
    ))
}

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

pub fn get_graphql_url() -> String {
    let url = dotenvy::var("LEETCODE_BASE_URL").unwrap();
    format!("{}/graphql/", url)
}

pub fn get_base_url() -> String {
    dotenvy::var("LEETCODE_BASE_URL").unwrap()
}

pub fn gen_link_by_slug(slug: &str) -> String {
    let url = dotenvy::var("LEETCODE_BASE_URL").unwrap();
    format!("{}/problems/{}/", url, slug)
}

pub fn get_leetcode_session() -> String {
    dotenvy::var("LEETCODE_SESSION").expect("LEETCODE_SESSION not set")
}
