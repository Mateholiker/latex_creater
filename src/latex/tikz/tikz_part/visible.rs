use std::collections::HashSet;

use crate::latex::ToLatex;

use super::{LatexLines, LatexResult, TikzPart};

#[derive(Clone)]
pub struct Visible {
    frames: HashSet<u32>,
    inner: Vec<TikzPart>,
}

impl Visible {
    pub fn iter_inner(&self) -> impl Iterator<Item = &TikzPart> + DoubleEndedIterator {
        self.inner.iter()
    }
}

impl ToLatex for Visible {
    fn export(&self) -> LatexResult<LatexLines> {
        todo!()
    }
}
