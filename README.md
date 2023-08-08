[//]: # (auto_md_to_doc_comments segment start A)

# foreground_scheduler  

[//]: # (auto_cargo_toml_to_md start)

**runs a command at interval in foreground**  
***version: 2023.531.1201 date: 2023-05-31 author: [bestia.dev](https://bestia.dev) repository: [Github](https://github.com/bestia-dev/foreground_scheduler)***  

![status](https://img.shields.io/badge/active_dev-green) 
![status](https://img.shields.io/badge/in_use-green) 

[//]: # (auto_cargo_toml_to_md end)

[//]: # (auto_lines_of_code start)
[![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-141-green.svg)](https://github.com/bestia-dev/foreground_scheduler/)
[![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-69-blue.svg)](https://github.com/bestia-dev/foreground_scheduler/)
[![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-46-purple.svg)](https://github.com/bestia-dev/foreground_scheduler/)
[![Lines in examples](https://img.shields.io/badge/Lines_in_examples-0-yellow.svg)](https://github.com/bestia-dev/foreground_scheduler/)
[![Lines in tests](https://img.shields.io/badge/Lines_in_tests-0-orange.svg)](https://github.com/bestia-dev/foreground_scheduler/)

[//]: # (auto_lines_of_code end)

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/bestia-dev/foreground_scheduler/blob/master/LICENSE)
![Hits](https://bestia.dev/webpage_hit_counter/get_svg_image/630941367.svg)

Hashtags: #rustlang #utility #cli  
My projects on Github are more like a tutorial than a finished product: [bestia-dev tutorials](https://github.com/bestia-dev/tutorials_rust_wasm).

## screen instead of background

In Linux I love to use the screen command. In a screen session I can run a program
and detach with `ctrl+a, d`.
The program will run indefinitely. With `screen -r name` I can attach the session again and see
what is going on. And then detach again.  
Watching the stdout of the program "in foreground" is easier then reading logs. This is from the viewpoint of a developer. I want to see my program how it works after every modification.  
This is great for beta web servers. They need to run indefinitely.  
For other tasks like fetching data every hour I need a scheduler. The scheduler will run indefinitely inside a screen. The fetch program will run for a few seconds every hour.  

## Run

Run it with this arguments minute, command, args:  

`foreground_scheduler 4 cargo "crev repo fetch trusted"`  
This will run every hour at xx:04 minutes.  

## Development

Documentation:  
<https://bestia-dev.github.io/foreground_scheduler>  
List of prepared automation tasks for development: build, run, doc, publish,...  
`cargo auto`  

## cargo crev reviews and advisory

We live in times of danger with [supply chain attacks](https://en.wikipedia.org/wiki/Supply_chain_attack).  
It is recommended to always use [cargo-crev](https://github.com/crev-dev/cargo-crev)  
to verify the trustworthiness of each of your dependencies.  
Please, spread this info.  
You can also read reviews quickly on the web:  
<https://web.crev.dev/rust-reviews/crates/>  

## Open-source and free as a beer

My open-source projects are free as a beer (MIT license).  
I just love programming.  
But I need also to drink. If you find my projects and tutorials helpful, please buy me a beer by donating to my [PayPal](https://paypal.me/LucianoBestia).  
You know the price of a beer in your local bar ;-)  
So I can drink a free beer for your health :-)  
[Na zdravje!](https://translate.google.com/?hl=en&sl=sl&tl=en&text=Na%20zdravje&op=translate) [Alla salute!](https://dictionary.cambridge.org/dictionary/italian-english/alla-salute) [Prost!](https://dictionary.cambridge.org/dictionary/german-english/prost) [Nazdravlje!](https://matadornetwork.com/nights/how-to-say-cheers-in-50-languages/) 🍻

[//bestia.dev](https://bestia.dev)  
[//github.com/bestia-dev](https://github.com/bestia-dev)  
[//bestiadev.substack.com](https://bestiadev.substack.com)  
[//youtube.com/@bestia-dev-tutorials](https://youtube.com/@bestia-dev-tutorials)  

[//]: # (auto_md_to_doc_comments segment end A)
