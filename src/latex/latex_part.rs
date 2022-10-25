use std::collections::VecDeque;

use derive_more::From;

use super::{LatexLines, LatexResult, Tikz, ToLatex};

#[derive(From, Clone)]
pub enum LatexPart {
    #[from]
    Tikz(Tikz),
    Frame(Vec<LatexPart>),
    Center(Vec<LatexPart>),
}

pub struct FullPartIter<'p> {
    queue: VecDeque<&'p LatexPart>,
}

impl<'p> Iterator for FullPartIter<'p> {
    type Item = &'p LatexPart;

    fn next(&mut self) -> Option<Self::Item> {
        let first = self.queue.pop_front()?;
        if let Some(inner) = first.iter_next_inner() {
            for inner in inner.rev() {
                self.queue.push_front(inner)
            }
        }
        Some(first)
    }
}

impl LatexPart {
    pub fn iter_next_inner(
        &self,
    ) -> Option<impl Iterator<Item = &LatexPart> + DoubleEndedIterator> {
        match self {
            LatexPart::Tikz(_) => None,
            LatexPart::Center(inner) | LatexPart::Frame(inner) => Some(inner.iter()),
        }
    }

    pub fn iter_full(&self) -> FullPartIter {
        let mut queue = VecDeque::new();
        queue.push_back(self);
        FullPartIter { queue }
    }
}

impl ToLatex for LatexPart {
    fn export(&self) -> LatexResult<LatexLines> {
        match self {
            LatexPart::Tikz(tikz) => tikz.export(),
            LatexPart::Frame(inner) => wrap(
                r"\begin{frame}".to_owned(),
                r"\end{frame}".to_owned(),
                inner,
            ),
            LatexPart::Center(inner) => wrap(
                r"\begin{center}".to_owned(),
                r"\end{center}".to_owned(),
                inner,
            ),
        }
    }
}

fn wrap(start: String, end: String, inner: &[LatexPart]) -> LatexResult<LatexLines> {
    let mut lines = Vec::new();
    lines.push(start.into());
    for inner in inner {
        let mut inner_lines = inner.export()?;
        for mut inner_line in inner_lines.drain(..) {
            inner_line.indentation += 1;
            lines.push(inner_line)
        }
    }

    lines.push(end.into());

    Ok(lines.into())
}
