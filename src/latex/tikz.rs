use super::{LatexResult, ToLatex};
use derive_more::From;
use simple_math::Vec2;
use std::collections::HashSet;
use std::fmt::Write;
use std::hash::{Hash, Hasher};
use std::mem::discriminant;

mod node;
pub use node::{Node, NodeOptions};

mod line;
pub use line::{Line, LineOptions};

mod polygon;
pub use polygon::{Polygon, PolygonOption};

mod error;
pub use error::TikzError;

mod color;
pub use color::Color;

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

    pub fn get_color_definitions(&self) -> LatexResult {
        let mut latex = String::new();
        for color in self.parts.iter().filter_map(|part| part.get_color()) {
            let name = color.name();
            let r = color.r as f32 / 255.0;
            let g = color.g as f32 / 255.0;
            let b = color.b as f32 / 255.0;
            writeln!(
                &mut latex,
                "\\definecolor{{{name}}}{{rgb}}{{{r}, {g}, {b}}}"
            )?;
        }
        Ok(latex)
    }
}

impl ToLatex for Tikz {
    fn export(&self) -> LatexResult {
        let mut latex = String::new();
        write!(&mut latex, "\\begin{{tikzpicture}}[")?;
        for option in self.options.iter() {
            write!(&mut latex, "{}", option.export()?)?;
        }
        writeln!(&mut latex, "]")?;

        for part in self.parts.iter() {
            write!(&mut latex, "\t{}", part.export()?)?;
        }
        writeln!(&mut latex, "\\end{{tikzpicture}}")?;
        Ok(latex)
    }
}

#[derive(From, Clone)]
pub enum TikzPart {
    Node(Node),
    Line(Line),
    Polygon(Polygon),
}

impl TikzPart {
    fn get_color(&self) -> Option<Color> {
        match self {
            TikzPart::Node(_) => todo!(),
            TikzPart::Line(_) => todo!(),
            TikzPart::Polygon(polygon) => polygon.get_color(),
        }
    }
}

impl ToLatex for TikzPart {
    fn export(&self) -> LatexResult {
        match self {
            TikzPart::Node(_) => todo!(),
            TikzPart::Line(_) => todo!(),
            TikzPart::Polygon(polygon) => polygon.export(),
        }
    }
}

#[derive(Clone, Copy)]
pub enum TikzOption {
    Scale(f32),
}

impl ToLatex for TikzOption {
    fn export(&self) -> LatexResult {
        match self {
            TikzOption::Scale(scale) => {
                if scale.is_finite() {
                    Ok(format!("scale={scale}"))
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
