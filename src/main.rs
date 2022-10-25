#![allow(dead_code)]
#![allow(irrefutable_let_patterns)]
//#![allow(unused_variables)]
//#![allow(unused_imports)]
#![allow(unreachable_patterns)]
#![feature(exit_status_error)]

mod latex;
use std::{path::Path, str::from_utf8};

use latex::{AddOption, AddPointByParts, Color, Latex, LatexOption, Polygon, Tikz};

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

    let latex = Latex::new().part(tikz).option(LatexOption::FullDokument);

    //println!("{}", latex.export().unwrap());
    let output = latex.write_and_compile(Path::new("test/test.tex")).unwrap();
    if output.status.exit_ok().is_err() {
        println!("{}", from_utf8(&output.stdout).unwrap())
    }
}
