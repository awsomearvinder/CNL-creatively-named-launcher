#![feature(vec_remove_item)]
#![allow(dead_code)]
use gio;
use gtk::LabelExt;
use gtk::*;
use std::{path, process};
use xdg;

mod lib;

const NUM_LABELS: i32 = 5;

//note: this will always be atleast as big as the minimum required space to fit
//everything on screen.
const WIDTH: i32 = 700;
const HEIGHT: i32 = 0;

/*Structure of app:
 * App -
 *    Body
 *        -Entry
 *        -collection of labels
*/
#[derive(Debug)]
struct App {
    window: Window,
    body: Body,
}

impl App {
    fn new() -> Self {
        let window = Window::new(WindowType::Toplevel);
        let body = Body::new();
        window.set_default_size(WIDTH, HEIGHT);
        window.set_decorated(false);
        window.add(&body.content_grid);
        Self::load_css();
        App { window, body }
    }
    fn load_css() -> () {
        let provider = CssProvider::new();
        let xdg_base = match xdg::BaseDirectories::with_prefix("launcher") {
            Ok(base) => base,
            Err(e) => {
                eprintln!(
                    "got err, {} Does your system support XDG spec? aborting process.",
                    e
                );
                process::exit(1);
            }
        };
        let css_sheet = match xdg_base.find_config_file("style.css") {
            Some(file_path) => dbg!(file_path),
            None => {
                eprintln!("couldn't find css sheet, assuming .config");
                path::PathBuf::from("~/.config/launcher/style.css")
            }
        };
        match provider.load_from_file(&gio::File::new_for_path(&css_sheet)) {
            Ok(_) => (),
            Err(e) => {
                eprintln!(
                    "couldn't find launcher.css, using default, error for debug:{}",
                    e
                );
                match provider.load_from_path("./style.css") {
                    Ok(_) => (),
                    Err(e) => {
                        eprintln!("couldn't find default, err: {}", e);
                        eprintln!("continuing without css");
                    }
                }
            }
        }
        StyleContext::add_provider_for_screen(
            &gdk::Screen::get_default().expect("failed to initialize css provider"),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }
}

#[derive(Debug)]
struct Body {
    input: Entry,
    labels: LabelContainer,
    content_grid: Grid,
}

impl Body {
    fn new() -> Self {
        let input = Entry::new();
        let labels = LabelContainer::new();
        let cloned_labels = labels.clone();
        //whenever input is populated, a new search through bins is performed.
        let searcher = lib::Searcher::new();
        input.connect_changed(move |input| {
            let input = match input.get_text() {
                Some(gstring) => gstring.as_str().to_owned(),
                None => "".to_owned(),
            };
            let bins = searcher.sorted_bins(&input);
            for (i, label) in cloned_labels.labels.iter().enumerate() {
                if input != "" && i < bins.len() {
                    label.set_text(bins[i].name());
                } else {
                    label.set_text("");
                }
            }
        });
        let content_grid = Grid::new();
        content_grid.set_column_homogeneous(true);
        content_grid.attach(&labels.content_grid, 0, 1, 1, 1);
        content_grid.attach(&input, 0, 0, 1, 1);
        Body {
            input,
            labels,
            content_grid,
        }
    }
}

#[derive(Clone, Debug)]
struct LabelContainer {
    labels: Vec<Label>,
    content_grid: Grid,
}

impl LabelContainer {
    fn new() -> Self {
        let mut labels = vec![];
        for _ in 0..NUM_LABELS {
            let label = Label::new(None);
            label.set_xalign(0.0);
            labels.push(label);
        }
        let content_grid = Grid::new();
        for (i, item) in labels.iter().enumerate() {
            content_grid.attach(item, 0, i as i32, 1, 1);
        }
        LabelContainer {
            labels,
            content_grid,
        }
    }
}

fn main() {
    gtk::init().unwrap_or_else(|_err| {
        eprintln!("failed to initialize gtk");
        std::process::exit(1);
    });
    let app = App::new();
    app.window.show_all();
    println!("Hello, world!");
    gtk::main();
}
