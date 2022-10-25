use lazy_static::lazy_static;
use std::collections::HashSet;
use std::fmt::{write, Display, Formatter, Result as FmtResult, Write as FmtWrite};
use std::hash::{Hash, Hasher};
use std::io::Write;
use std::mem::discriminant;
use std::ops::{Deref, DerefMut};
mod tikz;
use derive_more::From;
pub use tikz::{
    AddOption, AddPoint, AddPointByParts, Color, Line, LineOptions, Node, NodeOptions, Polygon,
    PolygonOption, Tikz, TikzError, TikzOption, TikzPart,
};

mod error;
pub use error::{LatexError, LatexResult};

mod to_latex;
pub use to_latex::ToLatex;

mod latex_part;
pub use latex_part::LatexPart;

lazy_static! {
    static ref EMPTY_VEC: Vec<LatexPart> = Vec::with_capacity(0);
}

pub struct Latex {
    document_class: DocumentClass,
    parts: Vec<LatexPart>,
}

impl Latex {
    pub fn new(document_class: DocumentClass) -> Self {
        Latex {
            document_class,
            parts: Vec::new(),
        }
    }

    pub fn part(mut self, part: impl Into<LatexPart>) -> Self {
        self.parts.push(part.into());
        self
    }
}

#[derive(From)]
pub struct LatexLines {
    pub lines: Vec<LatexLine>,
}

impl Deref for LatexLines {
    type Target = Vec<LatexLine>;

    fn deref(&self) -> &Self::Target {
        &self.lines
    }
}

impl DerefMut for LatexLines {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.lines
    }
}

impl Display for LatexLines {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        for LatexLine {
            indentation,
            line_content,
        } in self.lines.iter()
        {
            for _ in 0..*indentation {
                write!(f, "\t")?;
            }
            writeln!(f, "{}", line_content)?;
        }
        Ok(())
    }
}

impl From<Vec<String>> for LatexLines {
    fn from(mut lines: Vec<String>) -> Self {
        LatexLines {
            lines: lines.drain(..).map(|l| l.into()).collect(),
        }
    }
}

#[derive(Debug)]
pub struct LatexLine {
    pub indentation: usize,
    pub line_content: String,
}

impl From<String> for LatexLine {
    fn from(line_content: String) -> Self {
        LatexLine {
            indentation: 0,
            line_content,
        }
    }
}

impl Display for LatexLine {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        for _ in 0..self.indentation {
            write!(f, "\t")?;
        }
        write!(f, "{}", self.line_content)
    }
}

impl ToLatex for Latex {
    fn export(&self) -> LatexResult<LatexLines> {
        let mut lines = Vec::new();

        let first_line = self.document_class.get_document_class_line();
        lines.push(first_line.into());
        let input_line = r"\usepackage[utf8] {inputenc}";
        lines.push(input_line.to_owned().into());

        if self
            .parts
            .iter()
            .flat_map(|part| part.iter_full())
            .any(|part| matches!(part, LatexPart::Tikz(_)))
        {
            //the tikz base package
            let line = r"\usepackage{tikz}";
            lines.push(line.to_owned().into());

            let line = r"\usepackage{pgfplots}";
            lines.push(line.to_owned().into());

            //makes compiling faster by incremental tikz compiling
            let line = r"\usetikzlibrary{external}";
            lines.push(line.to_owned().into());
            let line = r"\usepackage{color}";
            lines.push(line.to_owned().into());
            let line = r"\usetikzlibrary{arrows ,automata ,positioning, shapes.misc}";
            lines.push(line.to_owned().into());
        }

        //get all colors
        let mut colors = HashSet::new();
        for part in self.parts.iter().flat_map(|part| part.iter_full()) {
            if let LatexPart::Tikz(tikz) = part {
                for color in tikz.get_colors() {
                    colors.insert(color);
                }
            }
        }
        for color in colors {
            lines.push(color.get_color_definitions());
        }

        let line = r"\begin{document}";
        lines.push(line.to_owned().into());

        for part in self.parts.iter() {
            let mut part_lines = part.export()?;
            for mut part_line in part_lines.drain(..) {
                part_line.indentation += 1;
                lines.push(part_line)
            }
        }

        let last_line = r"\end{document}";
        lines.push(last_line.to_owned().into());

        Ok(lines.into())
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum DocumentClass {
    Article,
    Beamer,
}

impl DocumentClass {
    fn get_document_class_line(&self) -> String {
        match self {
            DocumentClass::Article => "\\documentclass[10pt]{article}".to_owned(),
            DocumentClass::Beamer => "\\documentclass{beamer}".to_owned(),
        }
    }
}
