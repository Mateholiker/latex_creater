use std::collections::{HashSet, VecDeque};

use derive_more::From;

use super::{
    AddOption, AddPoint, AddPointByParts, Color, LatexLines, LatexResult, TikzError, ToLatex,
};

mod node;
pub use node::{Node, NodeOptions};

mod line;
pub use line::{Line, LineOptions};

mod polygon;
pub use polygon::{Polygon, PolygonOption};

mod visible;
pub use visible::Visible;

#[derive(From, Clone)]
pub enum TikzPart {
    Node(Node),
    Line(Line),
    Polygon(Polygon),
    Visible(Visible),
}

impl TikzPart {
    pub fn get_colors(&self) -> HashSet<Color> {
        match self {
            TikzPart::Node(_) => todo!(),
            TikzPart::Line(_) => todo!(),
            TikzPart::Polygon(polygon) => {
                let mut set = HashSet::new();
                if let Some(color) = polygon.get_color() {
                    set.insert(color);
                }
                set
            }
            TikzPart::Visible(vis) => vis
                .iter_inner()
                .flat_map(|inner| inner.get_colors())
                .collect(),
        }
    }
}

impl TikzPart {
    pub fn iter_next_inner(&self) -> Option<impl Iterator<Item = &TikzPart> + DoubleEndedIterator> {
        match self {
            TikzPart::Line(_) | TikzPart::Polygon(_) | TikzPart::Node(_) => None,
            TikzPart::Visible(vis) => Some(vis.iter_inner()),
        }
    }

    pub fn iter_full(&self) -> FullPartIter {
        let mut queue = VecDeque::new();
        queue.push_back(self);
        FullPartIter { queue }
    }
}

impl ToLatex for TikzPart {
    fn export(&self) -> LatexResult<LatexLines> {
        match self {
            TikzPart::Node(_) => todo!(),
            TikzPart::Line(_) => todo!(),
            TikzPart::Polygon(polygon) => polygon.export(),
            TikzPart::Visible(vis) => vis.export(),
        }
    }
}

pub struct FullPartIter<'p> {
    queue: VecDeque<&'p TikzPart>,
}

impl<'p> Iterator for FullPartIter<'p> {
    type Item = &'p TikzPart;

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
