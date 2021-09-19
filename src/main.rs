use std::env;
use std::io;
use std::io::prelude::*;
use std::fs::{self, File};
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


    let program;
    if args[1].ends_with(".ch8t") {
        program = load_program_text(&args)?;
    }
    else{
        program = load_program_binary(&args)?;
    }


    let mut emulator = emulator::Emulator::new();

    emulator.load_program(&program);


    emulator.run();

    Ok(())

}

fn load_program_text(args: &Vec<String>) -> Result<chip::Program, io::Error> {
    // if ends with chip8t load as text, otherwise binary
    let path = &args[1];

    let program_text = fs::read_to_string(path)?;


    Ok(chip::Program::Text(program_text))
}


fn load_program_binary(args: &Vec<String>) -> Result<chip::Program, io::Error> {
    // if ends with chip8t load as text, otherwise binary
    let path = &args[1];
    let mut buffer = [0; 0xDFF];

    let mut f = File::open(path)?;

    f.read(&mut buffer)?;

    Ok(chip::Program::Binary(buffer))
}
