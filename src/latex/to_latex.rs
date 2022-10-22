use simple_math::Vec2;

use super::LatexResult;

pub trait ToLatex {
    fn export(&self) -> LatexResult;
}

impl ToLatex for Vec2 {
    fn export(&self) -> LatexResult {
        Ok(format!("({}, {})", self.x(), self.y()))
    }
}
