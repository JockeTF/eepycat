use std::env::args;
use std::fs::read_to_string;
use std::io::Result;
use std::io::Write;
use std::io::stdout;
use std::process::exit;
use std::thread::sleep;

use fastrand::Rng;

use crate::delay::CharDelay;
use crate::input::search;
use crate::syntax::Syntect;

mod delay;
mod input;
mod syntax;

fn main() -> Result<()> {
    let argv: Vec<_> = args().collect();

    let [_, path] = argv.as_slice() else {
        eprintln!("Usage: eepycat <DIRECTORY>");
        exit(1);
    };

    let sources = search(path);
    let syntect = Syntect::new();
    let mut rng = Rng::new();

    while let Some((path, ext)) = rng.choice(&sources) {
        let content = read_to_string(path)?;
        let syntax = syntect.highlight(ext, &content);
        let delay = CharDelay::from(syntax.chars());
        print!("\x1bc");

        for (duration, char) in delay {
            sleep(duration);
            print!("{char}");
            stdout().flush().ok();
        }
    }

    eprintln!("Found no files to display");
    exit(1);
}
