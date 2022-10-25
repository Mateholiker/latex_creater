pub use super::{LatexLine, LatexLines, LatexResult, ToLatex};

use simple_math::Vec2;
use std::collections::HashSet;
use std::fmt::Write;
use std::hash::{Hash, Hasher};
use std::mem::discriminant;

mod error;
pub use error::TikzError;

mod color;
pub use color::Color;

mod tikz_part;
pub use tikz_part::{
    Line, LineOptions, Node, NodeOptions, Polygon, PolygonOption, TikzPart, Visible,
};

#[derive(Default, Clone)]
pub struct Tikz {
    parts: Vec<TikzPart>,
    options: HashSet<TikzOption>,
}

impl Tikz {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn part(mut self, part: impl Into<TikzPart>) -> Self {
        self.parts.push(part.into());
        self
    }

    pub fn option(mut self, option: impl Into<TikzOption>) -> Self {
        self.options.insert(option.into());
        self
    }

    pub fn get_colors(&self) -> HashSet<Color> {
        self.parts
            .iter()
            .flat_map(|part| part.get_colors())
            .collect()
    }
}

impl ToLatex for Tikz {
    fn export(&self) -> LatexResult<LatexLines> {
        let mut first_line = String::new();
        write!(&mut first_line, "\\begin{{tikzpicture}}[")?;
        for option in self.options.iter() {
            write!(&mut first_line, "{}", option.export()?)?;
        }
        first_line.push(']');
        let last_line = "\\end{tikzpicture}".to_owned();

        let mut lines = Vec::new();

        lines.push(first_line.into());
        for part in self.parts.iter() {
            let mut part_lines = part.export()?;
            for mut line in part_lines.drain(..) {
                line.indentation += 1;
                lines.push(line);
            }
        }

        lines.push(last_line.into());
        Ok(lines.into())
    }
}

#[derive(Clone, Copy)]
pub enum TikzOption {
    Scale(f32),
}

impl ToLatex for TikzOption {
    fn export(&self) -> LatexResult<LatexLines> {
        match self {
            TikzOption::Scale(scale) => {
                if scale.is_finite() {
                    Ok(vec![format!("scale={scale}")].into())
                } else {
                    Err(TikzError::NotFiniteFloat.into())
                }
            }
        }
    }
}

impl PartialEq for TikzOption {
    fn eq(&self, other: &Self) -> bool {
        discriminant(self) == discriminant(other)
    }
}

impl Eq for TikzOption {}

impl Hash for TikzOption {
    fn hash<H: Hasher>(&self, state: &mut H) {
        discriminant(self).hash(state);
    }
}

pub trait AddPoint<T>
where
    T: Into<Vec2>,
{
    fn point(self, point: T) -> Self;
}

pub trait AddPointByParts<N, M>: Sized + AddPoint<(N, M)>
where
    (N, M): Into<Vec2>,
{
    fn point(self, x: N, y: M) -> Self {
        AddPoint::point(self, (x, y))
    }
}

pub trait AddOption<T> {
    fn option(self, option: T) -> Self;
}
