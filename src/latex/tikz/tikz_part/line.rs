use simple_math::Vec2;

#[derive(Clone)]
pub struct Line {
    points: Vec<Vec2>,
    line_options: Vec<LineOptions>,
}

#[derive(Clone)]
pub enum LineOptions {}

/*
let mut latex = String::new();
        write!(&mut latex, "\\fill[")?;
        for option in self.polygon_options.iter() {
            write!(&mut latex, "{}", option.export()?)?;
        }
        write!(&mut latex, "]")?;

        let mut iter = self.points.iter();
        if let Some(point) = iter.next() {
            write!(&mut latex, " {}", point.export()?)?;
        }
        for point in iter {
            write!(&mut latex, " -- {}", point.export()?)?;
        }
        */
