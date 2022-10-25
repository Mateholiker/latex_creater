#![allow(dead_code)]
#![allow(irrefutable_let_patterns)]
//#![allow(unused_variables)]
//#![allow(unused_imports)]
#![allow(unreachable_patterns)]
#![feature(exit_status_error)]

mod latex;
use std::{path::Path, str::from_utf8};

use latex::{AddOption, AddPointByParts, Color, DocumentClass, Latex, LatexPart, Polygon, Tikz};

use crate::latex::{TikzOption, ToLatex};

fn main() {
    let poly = Polygon::new()
        .option(Color {
            r: 100,
            g: 17,
            b: 8,
        })
        .point(0, 9.1)
        .point(30, 9.1)
        .point(30, 12.6)
        .point(0, 12.6);

    let tikz = Tikz::new().part(poly).option(TikzOption::Scale(0.5));

    let frame = LatexPart::Frame(vec![tikz.into()]);

    let latex = Latex::new(DocumentClass::Beamer).part(frame);

    let output = latex.write_and_compile(Path::new("test/test.tex")).unwrap();
    if output.status.exit_ok().is_err() {
        println!("{}\n\n\n", from_utf8(&output.stdout).unwrap());
        println!("{}", latex.export().unwrap());
    }
}
