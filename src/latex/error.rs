use std::fmt::Error as FmtError;
use std::io::Error as IOError;

use derive_more::From;

use super::{LatexLine, TikzError};

pub type LatexResult<T> = Result<T, LatexError>;

#[derive(From, Debug)]
pub enum LatexError {
    Formatting(FmtError),
    Tikz(TikzError),
    IO(IOError),
    PathIsNoFile,
}
