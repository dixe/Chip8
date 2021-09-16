use std::env;
use std::io;
use std::io::prelude::*;
use std::fs::File;


mod chip;
mod instructions;
mod emulator;
mod registers;
mod keyboard;
mod display;

fn main() -> io::Result<()> {

    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {

        println!("Specify a input file to read as rom");

        return Ok(());
    }


    let path = &args[1];


    let mut buffer = [0; 0xDFF];


    let mut f = File::open(path)?;

    f.read(&mut buffer)?;

    let program = chip::Program::Binary(buffer);

    let mut chip = chip::Chip8::new();

    chip.load_program(&program);

    for _ in 0..21 {
        emulator::cycle(&mut chip);
    }

    // show screen some how

    Ok(())



}
