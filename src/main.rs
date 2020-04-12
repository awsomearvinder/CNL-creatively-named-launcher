#![feature(vec_remove_item)]
#![feature(bool_to_option)]
#![allow(dead_code)]
use gio;
use gtk::LabelExt;
use gtk::*;
use std::{cell, path, process, rc};
use xdg;

mod lib;

const NUM_LABELS: i32 = 5;
//note: this will always be atleast as big as the minimum required space to fit
//everything on screen.
const WIDTH: i32 = 700;
const HEIGHT: i32 = 0;

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
        let xdg_base = xdg::BaseDirectories::with_prefix("launcher").unwrap_or_else(|err| {
            eprintln!(
                "got err, {} Does your system support XDG spec? aborting process.",
                err
            );
            process::exit(1);
        });
        let css_sheet = xdg_base.find_config_file("style.css").unwrap_or_else(|| {
            eprintln!("couldn't find css sheet, assuming .config");
            path::PathBuf::from("$HOME/config/launcher/style.css")
        });
        provider
            .load_from_file(&gio::File::new_for_path(&css_sheet))
            .unwrap_or_else(|e| {
                eprintln!("couldn't find launcher.css, using default, err:{}", e);
                provider.load_from_path("./style.css").unwrap_or_else(|e| {
                    eprintln!("couldn't find default, err: {}", e);
                    eprintln!("continuing without css");
                })
            });
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
    labels: rc::Rc<LabelContainer>,
    content_grid: Grid,
}

impl Body {
    fn new() -> Self {
        let input = Entry::new();
        let labels = rc::Rc::new(LabelContainer::new());
        let cloned_labels = labels.clone();
        //whenever input is populated, a new search through bins is performed.
        let searcher = lib::Searcher::new();
        let bins = rc::Rc::new(cell::RefCell::new(Vec::new()));
        let bins_ref = bins.clone();
        input.connect_changed(move |input| {
            let input = match input.get_text() {
                Some(gstring) => gstring.as_str().to_owned(),
                None => "".to_owned(),
            };
            *bins_ref.borrow_mut() = searcher.sorted_bins(&input);
            for (i, label) in cloned_labels.labels.iter().enumerate() {
                if input != "" && i < bins_ref.borrow().len() {
                    label.set_text(bins_ref.borrow()[i].name());
                } else {
                    label.set_text("");
                }
            }
        });
        input.connect_activate(move |_input| {
            let bin_code = bins.borrow()[0].clone().exec();
            eprintln!("{:?}", &bin_code);
            bin_code.is_ok().then(|| {
                std::thread::sleep(std::time::Duration::new(0, 500_000_000));
                process::exit(0)
            });
            bin_code.is_err().then(|| process::exit(1));
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
