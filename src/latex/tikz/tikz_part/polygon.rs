use super::{AddOption, AddPoint, AddPointByParts, Color, TikzError};
use crate::latex::{LatexLine, LatexLines, LatexResult, ToLatex};
use derive_more::From;
use simple_math::Vec2;
use std::collections::HashSet;
use std::fmt::Write;
use std::hash::{Hash, Hasher};
use std::mem::discriminant;

#[derive(Default, Clone)]
pub struct Polygon {
    points: Vec<Vec2>,
    options: HashSet<PolygonOption>,
}

impl Polygon {
    pub fn new() -> Self {
        Self::default()
    }

    pub(super) fn get_color(&self) -> Option<Color> {
        self.options.iter().find_map(|o| {
            if let PolygonOption::Color(color) = o {
                Some(*color)
            } else {
                None
            }
        })
    }
}

impl<T: Into<Vec2>> AddPoint<T> for Polygon {
    fn point(mut self, point: T) -> Self {
        self.points.push(point.into());
        self
    }
}
impl<N, M> AddPointByParts<N, M> for Polygon where (N, M): Into<Vec2> {}

impl<T: Into<PolygonOption>> AddOption<T> for Polygon {
    fn option(mut self, option: T) -> Self {
        self.options.insert(option.into());
        self
    }
}

impl ToLatex for Polygon {
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
                writeln!(&mut latex, " cycle;")?;
            }
        }

        Ok(vec![latex].into())
    }
}

#[derive(Debug, Clone, Copy, From)]
pub enum PolygonOption {
    Color(Color),
}

impl ToLatex for PolygonOption {
    fn export(&self) -> LatexResult<LatexLines> {
        match self {
            PolygonOption::Color(color) => Ok(vec![format!("color={}", color.name())].into()),
        }
    }
}

impl PartialEq for PolygonOption {
    fn eq(&self, other: &Self) -> bool {
        discriminant(self) == discriminant(other)
    }
}

impl Eq for PolygonOption {}

impl Hash for PolygonOption {
    fn hash<H: Hasher>(&self, state: &mut H) {
        discriminant(self).hash(state);
    }
}
