use std::env;
use std::io;
use std::io::prelude::*;
use std::fs::File;
use sdl2;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;


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


    let mut emulator = emulator::Emulator::new();

    emulator.load_program(&program);


    emulator.run();

    Ok(())

}
