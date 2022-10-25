use std::collections::VecDeque;

use derive_more::From;

use super::{Latex, LatexLines, LatexResult, Tikz, ToLatex};

#[derive(From, Clone)]
pub enum LatexPart {
    Tikz(Tikz),
    Frame(Vec<LatexPart>),
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
            LatexPart::Frame(inner) => Some(inner.iter()),
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
            LatexPart::Frame(inner) => {
                let mut lines = Vec::new();

                let line = r"\begin{frame}";
                lines.push(line.to_owned().into());
                for inner in inner {
                    let mut inner_lines = inner.export()?;
                    for mut inner_line in inner_lines.drain(..) {
                        inner_line.indentation += 1;
                        lines.push(inner_line)
                    }
                }

                let line = r"\end{frame}";
                lines.push(line.to_owned().into());

                Ok(lines.into())
            }
        }
    }
}
