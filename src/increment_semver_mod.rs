//! includecargotomldatamod.rs

use crate::utilsmod::*;

#[allow(unused_imports)]
use ansi_term::Colour::{Green, Yellow};
use std::fs;
use unwrap::unwrap;

pub fn increment_part(part: &str) {
    println!("pub fn increment_patch");
    let cargo_toml_filename = "cargo.toml";
    let cargo_toml_text = unwrap!(fs::read_to_string(cargo_toml_filename));
    // find the line with "version = " including the start quote
    if let Some(pos_start_data) =
        find_pos_start_data_after_delimiter(&cargo_toml_text, 0, r#"version = ""#)
    {
        // find the end quote
        if let Some(pos_end_data) =
            find_pos_end_data_before_delimiter(&cargo_toml_text, pos_start_data, r#"""#)
        {
            let version = cargo_toml_text[pos_start_data..pos_end_data].to_string();
            println!(r#"version: "{}""#, &version);
            //increment the last number
            let pos = pos_start_data;
            let (major, pos) = parse_next_number(&cargo_toml_text, pos);
            //jump over dot
            let pos = pos + 1;
            let (mut minor, pos) = parse_next_number(&cargo_toml_text, pos);
            //jump over dot
            let pos = pos + 1;
            let (mut patch, pos) = parse_next_number(&cargo_toml_text, pos);
            let pos_at_the_end_of_semver = pos;
            // increment
            if part == "patch" {
                patch += 1;
            } else if part == "minor" {
                minor += 1;
                patch = 0;
            }
            println!(r#"major: {},minor: {}, patch: {}"#, major, minor, patch);
            let new_semver = format!("{}.{}.{}", major, minor, patch);
            println!("new semver: '{}'", Green.paint(&new_semver));
            let new_cargo_toml_text = format!(
                "{}{}{}",
                &cargo_toml_text[..pos_start_data],
                &new_semver,
                &cargo_toml_text[pos_at_the_end_of_semver..]
            );
            //save the file
            let _x = fs::write(cargo_toml_filename, new_cargo_toml_text);
        } else {
            panic!("no end quote for version");
        }
    } else {
        panic!("cargo.toml has no version");
    }
}

fn parse_next_number(cargo_toml_text: &str, pos: usize) -> (usize, usize) {
    let mut pos = pos;
    let mut number = "".to_string();
    let mut one_char = cargo_toml_text[pos..pos + 1].chars().next().unwrap();
    while one_char.is_numeric() {
        number.push(one_char);
        pos += 1;
        one_char = cargo_toml_text[pos..pos + 1].chars().next().unwrap();
    }
    let number: usize = unwrap!(number.parse());
    //return
    (number, pos)
}
