use failure::Error;
use pulldown_cmark::{html, Parser};
use serde_derive::Serialize;
use std::fs::{create_dir_all, File};
use std::io::prelude::*;
use std::path::Path;
use tera::{Context, Tera};
use walkdir::{DirEntry, WalkDir};

const OUTPUT_DIR: &str = "public";
const CONTENTS_DIR: &str = "contents";

pub fn cmd_build() -> Result<(), Error> {
    create_dir_all(OUTPUT_DIR)?;

    let posts = build_pages(CONTENTS_DIR)?;
    build_process(posts)?;

    Ok(())
}

#[derive(Debug, Serialize)]
struct Page {
    pub title: String,
    url: String,
    content: String,
}

type Pages = Vec<Page>;

fn build_pages(dir: &str) -> Result<Pages, Error> {
    let mut pages = Vec::new();

    check_contents(dir)?;

    let entrys: Vec<DirEntry> = WalkDir::new(dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .collect();

    // Len becomes 1 even if there is nothing in the directory
    if entrys.len() <= 1 {
        failure::bail!("contents file not found");
    }

    for entry in entrys {
        if !entry.metadata()?.is_file() {
            continue;
        };

        let file = File::open(&entry.path())?;
        let path = entry
            .path()
            .display()
            .to_string()
            .replace(CONTENTS_DIR, OUTPUT_DIR)
            .replace(".md", ".html");

        let file_name = entry
            .file_name()
            .to_str()
            .unwrap()
            .to_string()
            .replace(".md", "");

        pages.push(Page {
            title: file_name,
            url: path,
            content: markdown_to_html(file)?,
        });
    }

    Ok(pages)
}

fn markdown_to_html(mut file: File) -> Result<String, Error> {
    let mut md = String::new();
    file.read_to_string(&mut md)?;

    let parser = Parser::new(&md);
    let mut html = String::new();
    html::push_html(&mut html, parser);

    Ok(html)
}

fn build_process(pages: Pages) -> Result<(), Error> {
    index_build(&pages)?;
    pages_build(&pages)?;

    Ok(())
}

fn index_build(pages: &Pages) -> Result<(), Error> {
    let tera = setted_tera();

    let mut context = Context::new();
    context.insert("pages", &pages);

    let rendered_html = tera.render("index.html", &context).unwrap();
    let mut write_buf = File::create(format!("{}/index.html", OUTPUT_DIR))?;
    write_buf.write(rendered_html.as_bytes())?;

    Ok(())
}

fn pages_build(pages: &Pages) -> Result<(), Error> {
    let tera = setted_tera();

    for page in pages {
        let mut context = Context::new();
        context.insert("content", &page.content);
        let rendered_html = tera.render("page.html", &context).unwrap();

        let mut write_buf = File::create(&page.url)?;
        write_buf.write(rendered_html.as_bytes())?;
    }

    Ok(())
}

fn check_contents(dir: &str) -> Result<(), Error> {
    let path = Path::new(dir);

    if !path.exists() {
        failure::bail!(
            r#"contents dir is not found.
hint: execute 'hibana new project_name'"#
        )
    }

    Ok(())
}

fn setted_tera() -> Tera {
    let mut tera = Tera::default();
    tera.add_raw_template("base.html", include_str!("../templates/base.html"))
        .expect("failed to load template");

    tera.add_raw_template("page.html", include_str!("../templates/page.html"))
        .expect("failed to load template");

    tera.add_raw_template("index.html", include_str!("../templates/index.html"))
        .expect("failed to load template");

    tera.autoescape_on(vec![]);
    tera
}
