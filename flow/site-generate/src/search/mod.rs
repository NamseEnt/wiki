use crate::dir;
use anyhow::Result;
use std::fs;

pub fn create_search_page() -> Result<()> {
    let html = include_str!("search.html");

    fs::write(dir::docs_dir().join("search.html"), html)?;

    Ok(())
}
