#![allow(dead_code)]
#![allow(irrefutable_let_patterns)]
//#![allow(unused_variables)]
//#![allow(unused_imports)]
#![allow(unreachable_patterns)]

mod latex;
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

    println!("{}", latex.export().unwrap());
}
