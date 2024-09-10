use std::path::PathBuf;

use crate::error::BitBitError;

pub struct Flags {
    /// File path of source codec
    file_path: PathBuf,
    /// Whether the program will run in raw brainfuck mode
    raw: bool,
}

impl Flags {
    #[inline]
    const fn new(file_path: PathBuf, raw: bool) -> Flags {
        Flags { file_path, raw }
    }

    #[inline]
    pub const fn file_path(&self) -> &PathBuf {
        &self.file_path
    }

    #[inline]
    pub const fn raw(&self) -> bool {
        self.raw
    }

    /// Read and parse command line arguments for interpreting
    /// `bitbit.exe file_path -raw`
    /// `file_path` is not optional
    /// Validate in order: arg fetch > arg len > exe path > file path > flags
    /// NOTE: This should use `OsString` with `std::env::args_os` but im too lazy :P
    pub fn from(args: Vec<String>) -> Result<Flags, BitBitError> {
        // validate: arg fetch
        if args.is_empty() {
            return Err(BitBitError::hint("argument read fail", "empty args"));
        }

        // validate: arg len
        match args.len() {
            l if l < 2 => return Err(BitBitError::hint("missing argument", "file path required")),
            l if l > 3 => return Err(BitBitError::err("too many arguments")),
            _ => {}
        }

        // args: [exe, file] or [exe, file, flag]
        let maybe_file_path = &args[1];
        let file_path = PathBuf::from(maybe_file_path);

        // validate: file path exists
        if !file_path.exists() {
            return Err(BitBitError::hint("invalid file path", "path/to/program.bf"));
        }

        // validate: extension exists
        let maybe_file_path_extension = file_path.extension();
        if maybe_file_path_extension.is_none() {
            return Err(BitBitError::hint("missing file extension", ".bf or .b"));
        }

        // SAFETY: None case handled
        let file_path_extension = unsafe { maybe_file_path_extension.unwrap_unchecked() };

        // validate: correct extension
        if !matches!(file_path_extension.to_str(), Some("b") | Some("bf")) {
            return Err(BitBitError::hint("invalid file extension", ".bf or .b"));
        }

        // args: [exe, file, flag]
        let mut raw = false;
        if args.len() < 3 {
            return Ok(Flags::new(file_path, raw));
        }

        // validate: raw mode activated
        match args[2].as_str() {
            "-raw" | "-r" => raw = true,
            _ => {
                return Err(BitBitError::hint(
                    format!("'{}' invalid flag", args[2]),
                    "-raw, -r",
                ))
            }
        }

        return Ok(Flags::new(file_path, raw));
    }
}
