use chrono::NaiveDateTime;
use html5ever::tendril::SliceExt;
use nipper::{Document, Selection};
use rayon::prelude::*;
use regex::Regex;
use serde::Deserialize;

use std::env::current_dir;
use std::fs;
use std::fs::{read_to_string, File};
use std::io::ErrorKind::NotFound;
use std::io::{Error, Write};
use std::path::PathBuf;

extern crate markdown;

fn main() -> Result<(), Error> {
    let schema_version = 1;
    println!("Looking for a typeset.toml file...");
    let cwd = current_dir()?;
    let config_file = cwd.join("typeset.toml");
    if !config_file.exists() {
        eprintln!("typeset.toml not found!");
        return Err(Error::from(NotFound));
    }
    let config: Config = toml::from_str(&*read_to_string(config_file.clone())?)?;
    if schema_version != config.schema_version {
        panic!("Schema version does not match this version of Typeset! Please update Typeset.")
    }
    let template_file = config_file.with_file_name(config.template);
    let index_file = config_file.with_file_name(config.index);
    let out_path = cwd.join(PathBuf::from(config.output));
    if !template_file.exists() {
        eprintln!("template does not exist!");
        return Err(Error::from(NotFound));
    }
    if !index_file.exists() {
        println!("Index not found, assuming it is not present")
    }
    println!("Read configuration successfully! Converting markdown files to html...");

    let mut posts: Vec<Post> = vec![];
    let input_files: Vec<PathBuf> = fs::read_dir(cwd)?
        .filter(|file| {
            let file = file.as_ref().unwrap();
            Regex::new(&*config.input)
                .expect("invalid input regex!")
                .is_match(file.file_name().to_str().unwrap())
        })
        .map(|file| file.unwrap().path())
        .collect();
    println!("Found {} files matching the pattern", input_files.len());
    posts.par_extend(input_files.par_iter().map(|file| {
        let content = read_to_string(file).unwrap();
        let content = content.splitn(2, "\n\n").collect::<Vec<&str>>();
        let settings: PostSettings = toml::from_str(content[0]).unwrap();
        let body = markdown::to_html(content[1]);
        Post {
            id: file.file_stem().unwrap().to_str().unwrap().to_string(),
            body,
            title: settings.title,
            published: NaiveDateTime::parse_from_str(&*settings.published, &*config.time_format)
                .unwrap(),
        }
    }));
    println!("Successfully converted to HTML! Creating documents...");
    let template = read_to_string(template_file)?;
    let index = Document::from(read_to_string(index_file)?.as_str());
    for post in &posts {
        let template_html = Document::from(template.as_str());
        let output_html = template_html;
        let typeset_elements = output_html.select("meta[typeset]");
        for mut element in typeset_elements.iter() {
            match element.attr("typeset").unwrap().to_string().as_str() {
                "page-title" => {
                    let title = element
                        .attr("content")
                        .unwrap()
                        .replace("$", post.title.as_str());

                    element.replace_with_html(format!(r#"<title>{title}</title>"#));
                }
                "title" => {
                    element.replace_with_html(post.title.to_string());
                }
                "body" => {
                    element.replace_with_html(post.body.to_string());
                }
                "date" => element
                    .replace_with_html(post.published.format(&*config.time_format).to_string()),
                _ => {
                    eprintln!("Unknown meta typeset element parsed!")
                }
            }
        }
        let output = output_html.html().to_string();
        let path = out_path.join(PathBuf::from(format!("{}.html", &post.id)));
        let mut file = File::create(path)?;
        file.write_all(output.as_bytes())?;
    }
    println!("Writing index...");
    posts.par_sort_by(|a, b| b.published.cmp(&a.published));
    println!(
        "Found {} post lists",
        index.select(r#"meta[typeset="index-entry""#).length()
    );
    for index_ref in index.select(r#"meta[typeset="index-entry""#).iter() {
        let n = index_ref
            .attr("content")
            .unwrap_or("".to_tendril())
            .parse()
            .unwrap_or(0);
        let parent = nth_parent(index_ref.clone(), n);
        let parent_html = parent.html();
        for post in posts.iter() {
            let html = parent_html.to_string();

            parent.parent().append_html(html);
            nth_children(parent.parent(), n)
                .select(r#"meta[typeset="index-entry""#)
                .replace_with_html(
                    format!("<a href=\"./{}.html\">{}</a>\n", post.id, post.title).as_str(),
                );
        }
        parent.parent().children().iter().nth(0).unwrap().remove();
    }

    let output = index.html().to_string();
    let path = out_path.join(PathBuf::from("index.html"));
    let mut file = File::create(path)?;
    file.write_all(output.as_bytes())?;
    println!("Have a nice day :)");
    Ok(())
}
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
struct Post {
    id: String,
    body: String,
    title: String,
    published: NaiveDateTime,
}
#[derive(Deserialize, Debug)]
struct PostSettings {
    title: String,
    published: String,
}
#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub schema_version: i32,
    pub name: String,
    pub index: String,
    pub template: String,
    pub ref_from_index: String,
    pub input: String,
    pub output: String,
    pub time_format: String,
}
fn nth_parent(selection: Selection, n: usize) -> Selection {
    let mut sel: Selection = selection;
    for _ in 0..n {
        sel = sel.parent();
    }
    return sel;
}
fn nth_children(selection: Selection, n: usize) -> Selection {
    let mut sel: Selection = selection;
    for _ in 0..n {
        sel = sel.children();
    }
    return sel;
}
