mod analysis;
mod bindings;
mod clean;
mod error;
mod flags;
mod interpret;
mod lexer;
mod memory;
mod read;

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);
        std::process::exit(e.exit_code());
    }
}

fn run() -> Result<(), error::BitBitError> {
    let args: Vec<String> = std::env::args().collect();
    let flags = flags::Flags::from(args)?;
    let operations = lexer::lex(&flags)?;
    let instructions = analysis::analyze(operations, &flags)?;
    let mut program = interpret::Program::new(instructions, flags);
    program.run()
}
