#![allow(unused_imports, unused_variables, dead_code)]

pub trait Widget {
    /// Natural width of `self`.
    fn width(&self) -> usize;

    /// Draw the widget into a buffer.
    fn draw_into(&self, buffer: &mut dyn std::fmt::Write);

    /// Draw the widget on standard output.
    fn draw(&self) {
        let mut buffer = String::new();
        self.draw_into(&mut buffer);
        println!("{buffer}");
    }
}

pub struct Label {
    label: String,
}

impl Label {
    fn new(label: &str) -> Label {
        Label {
            label: label.to_owned(),
        }
    }
}

pub struct Button {
    label: Label,
    callback: Box<dyn FnMut()>,
}

impl Button {
    fn new(label: &str, callback: Box<dyn FnMut()>) -> Button {
        Button {
            label: Label::new(label),
            callback,
        }
    }
}

pub struct Window {
    title: String,
    widgets: Vec<Box<dyn Widget>>,
}

impl Window {
    fn new(title: &str) -> Window {
        Window {
            title: title.to_owned(),
            widgets: Vec::new(),
        }
    }

    fn add_widget(&mut self, widget: Box<dyn Widget>) {
        self.widgets.push(widget);
    }
}

impl Widget for Label {
    fn width(&self) -> usize {
        // unimplemented!()
        self.label.lines().map(|line| line.len()).max().unwrap_or(0)
    }

    fn draw_into(&self, buffer: &mut dyn std::fmt::Write) {
        // unimplemented!()
        writeln!(buffer, "{}", self.label).unwrap();
    }
}

impl Widget for Button {
    fn width(&self) -> usize {
        // unimplemented!()
        self.label.width() + 8 // left 4 padding spaces, right 4 padding spaces
    }

    fn draw_into(&self, buffer: &mut dyn std::fmt::Write) {
        // unimplemented!()
        let width = self.width();
        let mut label = String::new();
        self.label.draw_into(&mut label);
        writeln!(buffer, "+{:-<width$}+", "").unwrap();
        for line in label.lines() {
            writeln!(buffer, "|{:^width$}|", &line).unwrap();
        }
        writeln!(buffer, "+{:-<width$}+", "").unwrap();
    }
}

impl Widget for Window {
    fn width(&self) -> usize {
        // unimplemented!()
        std::cmp::max(
            self.title.len(),
            self.widgets.iter().map(|w| w.width()).max().unwrap_or(0),
        )
    }

    fn draw_into(&self, buffer: &mut dyn std::fmt::Write) {
        // unimplemented!()
        let mut inner = String::new();
        for w in &self.widgets {
            w.draw_into(&mut inner);
        }
        let width = self.width();

        writeln!(buffer, "+-{:-<width$}-+", "").unwrap();
        writeln!(buffer, "| {:^width$} |", &self.title).unwrap();
        writeln!(buffer, "+={:=>width$}=+", "").unwrap();
        for line in inner.lines() {
            writeln!(buffer, "| {:width$} |", line).unwrap();
        }
        writeln!(buffer, "+-{:-<width$}-+", "").unwrap();
    }
}

fn main() {
    let mut window = Window::new("Rust GUI Demo 1.23");
    window.add_widget(Box::new(Label::new("This is a small text GUI demo.")));
    window.add_widget(Box::new(Button::new(
        "Click me!",
        Box::new(|| println!("You clicked the button!")),
    )));
    window.draw();
}
