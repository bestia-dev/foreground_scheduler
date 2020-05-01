// region: lmake_readme include "readme.md" //! A
//! # foreground_scheduler  
//! 
//! version: 2020.501.1621  date: 2020-05-01 authors: Luciano Bestia  
//! **runs a command at interval in foreground**
//! 
//! 
//! ## screen instead of background
//! 
//! In Linux I love to use the screen command. In a screen session I can run a program
//! and detach with `ctrl+a, d`.
//! The program will run indefinitely. With `screen -r name` I can attach the session again and see
//! what is going on. And then detach again.  
//! Watching the stdout of the program "in foreground" is easier then reading logs. This is from the viewpoint of a developer. I want to see my program how it works after every modification.  
//! This is great for beta web servers. They need to run indefinitely.  
//! For other tasks like fetching data every hour I need a scheduler. The scheduler will run indefinitely. The fetch program will run for a few seconds every hour.  
//! 
//! ## Run
//! 
//! Run it with this arguments:  
//! 
//! `foreground_scheduler "4 *","cargo crev repo fetch trusted"`  
//! This will run every hour at xx:04 minutes.  
//! 
//! ## Development
//! 
//! Documentation:  
//! <https://lucianobestia.github.io/foreground_scheduler>  
//! List of prepared make tasks for development: build, run, doc, publish,...  
//! `clear; cargo make`  
// endregion: lmake_readme include "readme.md" //! A

// region: Clippy
#![warn(
    clippy::all,
    clippy::restriction,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
    // variable shadowing is idiomatic to Rust, but unnatural to me.
    clippy::shadow_reuse,
    clippy::shadow_same,
    clippy::shadow_unrelated,

)]
#![allow(
    // library from dependencies have this clippy warnings. Not my code.
    // Why is this bad: It will be more difficult for users to discover the purpose of the crate, 
    // and key information related to it.
    clippy::cargo_common_metadata,
    // Why is this bad : This bloats the size of targets, and can lead to confusing error messages when 
    // structs or traits are used interchangeably between different versions of a crate.
    clippy::multiple_crate_versions,
    // Why is this bad : As the edition guide says, it is highly unlikely that you work with any possible 
    // version of your dependency, and wildcard dependencies would cause unnecessary 
    // breakage in the ecosystem.
    clippy::wildcard_dependencies,
    // Rust is more idiomatic without return statement
    // Why is this bad : Actually omitting the return keyword is idiomatic Rust code. 
    // Programmers coming from other languages might prefer the expressiveness of return. 
    // It’s possible to miss the last returning statement because the only difference 
    // is a missing ;. Especially in bigger code with multiple return paths having a 
    // return keyword makes it easier to find the corresponding statements.
    clippy::implicit_return,
    // I have private function inside a function. Self does not work there.
    // Why is this bad: Unnecessary repetition. Mixed use of Self and struct name feels inconsistent.
    clippy::use_self,
    // Cannot add #[inline] to the start function with #[wasm_bindgen(start)]
    // because then wasm-pack build --target web returns an error: export run not found 
    // Why is this bad: In general, it is not. Functions can be inlined across crates when that’s profitable 
    // as long as any form of LTO is used. When LTO is disabled, functions that are not #[inline] 
    // cannot be inlined across crates. Certain types of crates might intend for most of the 
    // methods in their public API to be able to be inlined across crates even when LTO is disabled. 
    // For these types of crates, enabling this lint might make sense. It allows the crate to 
    // require all exported methods to be #[inline] by default, and then opt out for specific 
    // methods where this might not make sense.
    clippy::missing_inline_in_public_items,
    // Why is this bad: This is only checked against overflow in debug builds. In some applications one wants explicitly checked, wrapping or saturating arithmetic.
    // clippy::integer_arithmetic,
    // Why is this bad: For some embedded systems or kernel development, it can be useful to rule out floating-point numbers.
    clippy::float_arithmetic,
    // Why is this bad : Doc is good. rustc has a MISSING_DOCS allowed-by-default lint for public members, but has no way to enforce documentation of private items. This lint fixes that.
    clippy::doc_markdown,
    // Why is this bad : Splitting the implementation of a type makes the code harder to navigate.
    clippy::multiple_inherent_impl,

    clippy::missing_docs_in_private_items,
)]
// endregion

// region: mod, extern and use statements

use clap::*;
//use unwrap::unwrap;

#[allow(unused_imports)]
use ansi_term::Colour::{Green, Red, Yellow};
use chrono::{Timelike, Utc};
use std::env;
use std::io::{self, Write};
use unwrap::unwrap;
// endregion

/// the main just calls try_main and shows errors if there are any
#[tokio::main]
async fn main() -> Result<()> {
    println!("start foreground_scheduler minute command \"args\"");
    println!("example: $ foreground_scheduler 4 cargo \"repo fetch trusted\"");
    // this function is different for Windows and for Linux.
    // Look at the code of this function (2 variations).
    enable_ansi_support();

    // define the CLI input line parameters using the clap library
    let arguments = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name("minute")
                .required(true)
                .value_name("minute every hour schedule")
                .help("04 - every hour at 04 minutes"),
        )
        .arg(
            Arg::with_name("command")
                .required(true)
                .value_name("command to run")
                .help("cargo"),
        )
        .arg(
            Arg::with_name("args")
                .required(true)
                .value_name("args")
                .help("'crev repo fetch trusted'"),
        )
        .get_matches();
    if let Some(minute) = arguments.value_of("minute") {
        println!("1st arg minute_hours: {}", minute);
        let minute = unwrap!(minute.parse::<usize>());
        if let Some(command) = arguments.value_of("command") {
            println!("2nd arg command: {}", command);
            if let Some(args) = arguments.value_of("args") {
                println!("3rd arg args: {}", args);
                loop {
                    if is_scheduled_run(minute) {
                        println!("Executed every hour on {} minute.", minute);
                        try_main(command, &args).await.unwrap();
                    }
                    // I need the tick resolution once per minute
                    std::thread::sleep(
                        std::time::Duration::from_millis(millis_until_next_minute()),
                    );
                    if false {
                        break;
                    }
                }
            }
        }
    }
    Ok(())
}

/// the errors are mostly propagated to the try_main
async fn try_main(command: &str, args: &str) -> Result<()> {
    use std::process::Command;
    println!(" $ {} {}", command, args);
    let args: Vec<&str> = args.split(' ').collect();
    Command::new(command)
        .args(args)
        .spawn()
        .expect("command failed to start");

    Ok(())
}

// region: different function code for Linux and Windows
#[cfg(target_family = "windows")]
/// only on windows "enable ansi support" must be called
pub fn enable_ansi_support() {
    let _enabled = ansi_term::enable_ansi_support();
}

#[cfg(target_family = "unix")]
//on Linux "enable ansi support" must not be called
pub fn enable_ansi_support() {
    // do nothing
}
// endregion

/// if the time is scheduled, then return true
/// resolution is 1 minute.
pub fn is_scheduled_run(minute: usize) -> bool {
    let now = Utc::now();
    let now_minute = now.minute();
    if now_minute == minute as u32 {
        return true;
    } else {
        print!("{}...", now_minute);
        io::stdout().flush().unwrap();
        return false;
    }
}
/// millis until next minute
pub fn millis_until_next_minute() -> u64 {
    let now = Utc::now();
    //return
    (60u64 - now.second() as u64) * 1000u64
}
