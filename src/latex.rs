use std::collections::HashSet;
use std::fmt::Write;
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

#[derive(Default)]
pub struct Latex {
    parts: Vec<LatexPart>,
    options: HashSet<LatexOption>,
}

impl Latex {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn part(mut self, part: impl Into<LatexPart>) -> Self {
        self.parts.push(part.into());
        self
    }

    pub fn option(mut self, option: impl Into<LatexOption>) -> Self {
        self.options.insert(option.into());
        self
    }
}

impl ToLatex for Latex {
    fn export(&self) -> LatexResult {
        let mut latex = String::new();
        let mut indentation_level = 0;

        //beginning
        if self.options.contains(&LatexOption::FullDokument) {
            writeln!(&mut latex, "\\documentclass[10pt]{{article}}")?;
            writeln!(&mut latex, "\\usepackage[utf8] {{inputenc}}")?;
            if self
                .parts
                .iter()
                .any(|part| matches!(part, LatexPart::Tikz(_)))
            {
                writeln!(&mut latex, "\\usepackage{{tikz}}")?;
                writeln!(&mut latex, "\\usepackage{{pgfplots}}")?;
                writeln!(&mut latex, "\\usetikzlibrary{{external}}")?;
                writeln!(&mut latex, "\\usepackage{{color}}")?;
                writeln!(
                    &mut latex,
                    "\\usetikzlibrary{{arrows ,automata ,positioning, shapes.misc}}"
                )?;
                writeln!(&mut latex)?;
            }
            indentation_level += 1;

            let mut colors = String::new();
            for tikz in self.parts.iter().filter_map(|part| match part {
                LatexPart::Tikz(tikz) => Some(tikz),
                _ => None,
            }) {
                write!(&mut colors, "{}", tikz.get_color_definitions()?)?;
            }
            writeln!(&mut latex, "{}", colors)?;

            writeln!(&mut latex, "\\begin{{document}}")?;
        }

        //mid
        for part in self.parts.iter() {
            let string = part.export()?;
            let mut indented_string = String::new();
            for _ in 0..indentation_level {
                indented_string.push('\t');
            }
            for char in string.chars() {
                match char {
                    '\n' => {
                        indented_string.push('\n');
                        for _ in 0..indentation_level {
                            indented_string.push('\t');
                        }
                    }
                    other => indented_string.push(other),
                }
            }
            writeln!(&mut latex, "{}", indented_string.trim_end_matches('\t'))?;
        }

        //end
        latex.pop();
        if self.options.contains(&LatexOption::FullDokument) {
            writeln!(&mut latex, "\\end{{document}}")?;
        }

        Ok(latex)
    }
}

#[derive(From, Clone)]
pub enum LatexPart {
    Tikz(Tikz),
}

impl ToLatex for LatexPart {
    fn export(&self) -> LatexResult {
        match self {
            LatexPart::Tikz(tikz) => tikz.export(),
        }
    }
}

#[derive(PartialEq, Eq, Hash)]
pub enum LatexOption {
    FullDokument,
}
