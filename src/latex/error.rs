use std::fmt::Error as FmtError;
use std::io::Error as IOError;

use derive_more::From;

use super::TikzError;

pub type LatexResult = Result<String, LatexError>;

#[derive(From, Debug)]
pub enum LatexError {
    Formatting(FmtError),
    Tikz(TikzError),
    IO(IOError),
    PathIsNoFile,
}
