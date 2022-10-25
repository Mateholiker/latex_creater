use std::io::Write;
use std::path::Path;
use std::process::Output;
use std::{fs::File, process::Command};

use simple_math::Vec2;

use super::{LatexError, LatexLines, LatexResult};

pub trait ToLatex {
    fn export(&self) -> LatexResult<LatexLines>;

    fn write_and_compile(&self, path: &Path) -> Result<Output, LatexError> {
        let mut file = File::create(path)?;
        write!(&mut file, "{}", self.export()?)?;

        let file_name = path.file_name().ok_or(LatexError::PathIsNoFile)?;
        let mut command = Command::new("pdflatex");
        if let Some(dir) = path.parent() {
            command.current_dir(dir);
        }

        let output = command.arg("-halt-on-error").arg(file_name).output()?;

        Ok(output)
    }
}

impl ToLatex for Vec2 {
    fn export(&self) -> LatexResult<LatexLines> {
        Ok(vec![format!("({}, {})", self.x(), self.y())].into())
    }
}
