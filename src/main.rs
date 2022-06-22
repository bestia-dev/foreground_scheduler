// region: auto_md_to_doc_comments include README.md A //!
//! # foreground_scheduler  
//!
//! **runs a command at interval in foreground**  
//! ***version: 2022.622.1243 date: 2022-06-22 author: [bestia.dev](https://bestia.dev) repository: [Github](https://github.com/bestia-dev/foreground_scheduler)***  
//!
//! [![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-140-green.svg)](https://github.com/bestia-dev/foreground_scheduler/)
//! [![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-38-blue.svg)](https://github.com/bestia-dev/foreground_scheduler/)
//! [![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-46-purple.svg)](https://github.com/bestia-dev/foreground_scheduler/)
//! [![Lines in examples](https://img.shields.io/badge/Lines_in_examples-0-yellow.svg)](https://github.com/bestia-dev/foreground_scheduler/)
//! [![Lines in tests](https://img.shields.io/badge/Lines_in_tests-0-orange.svg)](https://github.com/bestia-dev/foreground_scheduler/)
//!
//! [![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/bestia-dev/foreground_scheduler/blob/master/LICENSE)
//! [![Hits](https://hits.seeyoufarm.com/api/count/incr/badge.svg?url=https%3A%2F%2Fgithub.com%2Fbestia-dev%2Fforeground_scheduler&count_bg=%2379C83D&title_bg=%23555555&icon=&icon_color=%23E7E7E7&title=hits&edge_flat=false)](https://hits.seeyoufarm.com)
//!
//! Hashtags: #rustlang #utility #cli  
//! My projects on Github are more like a tutorial than a finished product: [bestia-dev tutorials](https://github.com/bestia-dev/tutorials_rust_wasm).
//!
//! ## screen instead of background
//!
//! In Linux I love to use the screen command. In a screen session I can run a program
//! and detach with `ctrl+a, d`.
//! The program will run indefinitely. With `screen -r name` I can attach the session again and see
//! what is going on. And then detach again.  
//! Watching the stdout of the program "in foreground" is easier then reading logs. This is from the viewpoint of a developer. I want to see my program how it works after every modification.  
//! This is great for beta web servers. They need to run indefinitely.  
//! For other tasks like fetching data every hour I need a scheduler. The scheduler will run indefinitely inside a screen. The fetch program will run for a few seconds every hour.  
//!
//! ## Run
//!
//! Run it with this arguments minute, command, args:  
//!
//! `foreground_scheduler 4 cargo "crev repo fetch trusted"`  
//! This will run every hour at xx:04 minutes.  
//!
//! ## Development
//!
//! Documentation:  
//! <https://bestia-dev.github.io/foreground_scheduler>  
//! List of prepared automation tasks for development: build, run, doc, publish,...  
//! `cargo auto`  
//!
//! ## cargo crev reviews and advisory
//!
//! We live in times of danger with [supply chain attacks](https://en.wikipedia.org/wiki/Supply_chain_attack).  
//! It is recommended to always use [cargo-crev](https://github.com/crev-dev/cargo-crev)  
//! to verify the trustworthiness of each of your dependencies.  
//! Please, spread this info.  
//! You can also read reviews quickly on the web:  
//! <https://web.crev.dev/rust-reviews/crates/>  
//!
//! ## open-source free and free as a beer
//!
//! My open-source projects are free as a beer (MIT license).  
//! I just love programming.  
//! But I need also to drink. If you find my projects and tutorials helpful,  
//! please buy me a beer donating on my [paypal](https://paypal.me/LucianoBestia).  
//! You know the price of a beer in your local bar ;-)  
//! So I can drink a free beer for your health :-)  
//! [Na zdravje!](https://translate.google.com/?hl=en&sl=sl&tl=en&text=Na%20zdravje&op=translate) [Alla salute!](https://dictionary.cambridge.org/dictionary/italian-english/alla-salute) [Prost!](https://dictionary.cambridge.org/dictionary/german-english/prost) [Nazdravlje!](https://matadornetwork.com/nights/how-to-say-cheers-in-50-languages/) 🍻
//!
//!
// endregion: auto_md_to_doc_comments include README.md A //!

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
mod scheduler_mod;

use clap::*;

#[allow(unused_imports)]
use ansi_term::Colour::{Green, Red, Yellow};
use chrono::{Local, Utc};
use std::env;
use unwrap::unwrap;
// endregion

/// starting CLI
fn main() -> Result<()> {
    // enable_ansi_support is different for Windows and for Linux.
    enable_ansi_support();
    println!("Start foreground_scheduler{}", "");
    println!("utc  : {}", &Utc::now().format("%Y-%m-%d %H:%M:%S"));
    println!("local: {}", &Local::now().format("%Y-%m-%d %H:%M:%S"));
    println!(
        "{}",
        Yellow.paint("example: $ foreground_scheduler 4 cargo \"repo fetch trusted\"")
    );

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
                scheduler_mod::loop_scheduler(minute, command, args);
            }
        }
    }
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
