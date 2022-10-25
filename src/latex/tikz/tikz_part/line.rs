use super::{AddOption, AddPoint, AddPointByParts, Color, TikzError};
use crate::latex::{LatexLines, LatexResult, ToLatex};
use derive_more::From;
use simple_math::Vec2;
use std::collections::HashSet;
use std::fmt::Write;
use std::hash::{Hash, Hasher};
use std::mem::discriminant;

#[derive(Default, Clone)]
pub struct Line {
    points: Vec<Vec2>,
    options: HashSet<LineOption>,
}

impl Line {
    pub fn new() -> Self {
        Self::default()
    }

    pub(super) fn get_color(&self) -> Option<Color> {
        self.options.iter().find_map(|o| {
            if let LineOption::Color(color) = o {
                Some(*color)
            } else {
                None
            }
        })
    }
}

impl<T: Into<Vec2>> AddPoint<T> for Line {
    fn point(mut self, point: T) -> Self {
        self.points.push(point.into());
        self
    }
}
impl<N, M> AddPointByParts<N, M> for Line where (N, M): Into<Vec2> {}

impl<T: Into<LineOption>> AddOption<T> for Line {
    fn option(mut self, option: T) -> Self {
        self.options.insert(option.into());
        self
    }
}

impl ToLatex for Line {
    fn export(&self) -> LatexResult<LatexLines> {
        let mut latex = String::new();
        write!(&mut latex, "\\fill[")?;
        for option in self.options.iter() {
            write!(&mut latex, "{}", option.export()?)?;
        }
        write!(&mut latex, "]")?;

        match &self.points[..] {
            [] => return Err(TikzError::NoPoints.into()),
            [points @ ..] => {
                for point in points {
                    write!(&mut latex, " {} --", point.export()?)?;
                }
                write!(&mut latex, " cycle;")?;
            }
        }

        Ok(vec![latex].into())
    }
}

#[derive(Debug, Clone, Copy, From)]
pub enum LineOption {
    Color(Color),
}

impl ToLatex for LineOption {
    fn export(&self) -> LatexResult<LatexLines> {
        match self {
            LineOption::Color(color) => Ok(vec![format!("color={}", color.name())].into()),
        }
    }
}

impl PartialEq for LineOption {
    fn eq(&self, other: &Self) -> bool {
        discriminant(self) == discriminant(other)
    }
}

impl Eq for LineOption {}

impl Hash for LineOption {
    fn hash<H: Hasher>(&self, state: &mut H) {
        discriminant(self).hash(state);
    }
}
