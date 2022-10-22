use std::fmt::Error as FmtError;

use derive_more::From;

use super::TikzError;

pub type LatexResult = Result<String, LatexError>;

#[derive(From, Debug)]
pub enum LatexError {
    Formatting(FmtError),
    Tikz(TikzError),
}
