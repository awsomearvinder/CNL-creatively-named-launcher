/* God I hate UI code
 * Closures in Rust are also a pain to work w/
 * Need to look into other UI libraries.
 */
use gtk::*;
use relm::{connect, Relm, Widget};
use relm_derive::{widget, Msg};
use std::{cell, path, process, rc};
use xdg;

use launcher_lib as lib;

const NUM_LABELS: usize = 5;

//note: this will always be atleast as big as the minimum required space to fit
//everything on screen.
const WIDTH: i32 = 700;
const HEIGHT: i32 = 0;

//#[derive(Debug)]
//pub struct App {
//    pub window: Window,
//    body: Body,
//}
//
//impl App {
//    pub fn new(is_embed: bool) -> Self {
//        let window = Window::new(WindowType::Toplevel);
//        let body = Body::new(is_embed);
//        window.set_default_size(WIDTH, HEIGHT);
//        window.set_decorated(false);
//        window.add(&body.content_grid);
//        Self::load_css();
//        App { window, body }
//    }
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
        path::PathBuf::from("$HOME/.config/launcher/style.css")
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

//
//#[derive(Debug)]
//struct Body {
//    input: Entry,
//    labels: rc::Rc<LabelContainer>,
//    content_grid: Grid,
//}
//
//impl Body {
//    fn new(is_embed: bool) -> Self {
//        let input = Entry::new();
//        let labels = rc::Rc::new(LabelContainer::new());
//        let cloned_labels = labels.clone();
//
//        //whenever input is populated, a new search through bins is performed.
//        //That means bins is moved into a closure, however we still need a reference
//        //to bins.
//        let bins = rc::Rc::new(cell::RefCell::new(Vec::new()));
//
//        let searcher = lib::Searcher::new();
//        let bins_ref = bins.clone();
//
//        input.connect_changed(move |input| {
//            let input = match input.get_text() {
//                Some(gstring) => gstring.as_str().to_owned(),
//                None => "".to_owned(),
//            };
//
//            //can't store directly because &bin's lifetime is tied to the scope
//            //of the closure. Bins outlives the closure so we have to clone it.
//            *bins_ref.borrow_mut() = searcher
//                .sorted_bins(&input)
//                .iter()
//                .map(|&x| x.clone())
//                .collect();
//            for (i, label) in cloned_labels.labels.iter().enumerate() {
//                if input != "" && i < bins_ref.borrow().len() {
//                    label.set_text(bins_ref.borrow()[i].name());
//                } else {
//                    label.set_text("");
//                }
//            }
//        });
//        input.connect_activate(move |_input| {
//            //Run the binary
//            if !is_embed {
//                bins.borrow()[0].clone().exec().unwrap();
//            } else {
//                println!("{}", bins.borrow()[0].clone().exec_cmd())
//            }
//            process::exit(0);
//        });
//        let content_grid = Grid::new();
//        content_grid.set_column_homogeneous(true);
//        content_grid.attach(&labels.content_grid, 0, 1, 1, 1);
//        content_grid.attach(&input, 0, 0, 1, 1);
//        Body {
//            input,
//            labels,
//            content_grid,
//        }
//    }
//}
//
//#[derive(Clone, Debug)]
//struct LabelContainer {
//    labels: Vec<Label>,
//    content_grid: Grid,
//}
//
//impl LabelContainer {
//    fn new() -> Self {
//        let mut labels = vec![];
//        for _ in 0..NUM_LABELS {
//            let label = Label::new(None);
//            label.set_xalign(0.0);
//            labels.push(label);
//        }
//
//        let content_grid = Grid::new();
//        for (i, item) in labels.iter().enumerate() {
//            content_grid.attach(item, 0, i as i32, 1, 1);
//        }
//
//        LabelContainer {
//            labels,
//            content_grid,
//        }
//    }
//}

pub struct Model {
    searcher: lib::Searcher,
    bins: Vec<lib::bin::Bin>,
}

#[derive(Msg, Debug)]
pub enum Msg {
    UpdatedInput(String),
    Enter,
    Quit,
}

struct Win {
    model: Model,
    widgets: Widgets,
}

struct Widgets {
    _input: gtk::Entry,
    labels: Vec<gtk::Label>,
    window: gtk::Window,
}

impl relm::Update for Win {
    type Model = Model;
    type ModelParam = ();
    type Msg = Msg;

    fn model(_: &Relm<Self>, _: ()) -> Model {
        Model {
            searcher: lib::Searcher::new(),
            bins: Vec::new(),
        }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::UpdatedInput(c) => {
                let model = &mut self.model;
                let searcher = &model.searcher;
                model.bins = searcher
                    .sorted_bins(&c)
                    .into_iter()
                    .map(|x| x.clone())
                    .collect();
                let mut output_values = self.model.bins.iter().map(|x| x.name().to_owned());
                for label in &mut self.widgets.labels {
                    label.set_text(&output_values.next().unwrap_or("".into()))
                }
            }
            Msg::Quit => std::process::exit(0),
            Msg::Enter => {
                println!("{}", self.model.bins[0].exec_cmd());
            }
        }
    }
}
impl Widget for Win {
    type Root = gtk::Window;

    fn root(&self) -> Self::Root {
        self.widgets.window.clone()
    }

    fn view(relm: &relm::Relm<Self>, model: Self::Model) -> Self {
        let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);

        let input = Entry::new();
        vbox.add(&input);

        let mut labels = Vec::new();
        for i in 0..NUM_LABELS {
            labels.push(gtk::Label::new(None));
            labels[i].set_halign(gtk::Align::Start);
            vbox.add(&labels[i])
        }

        let window = gtk::Window::new(gtk::WindowType::Toplevel);
        window.add(&vbox);

        window.show_all();

        connect!(
            relm,
            input,
            connect_changed(entry),
            Msg::UpdatedInput(if let Some(val) = entry.get_text() {
                val.to_string()
            } else {
                "".into()
            })
        );

        connect!(
            relm,
            window,
            connect_delete_event(_, _),
            return (Some(Msg::Quit), Inhibit(false))
        );

        connect!(relm, input, connect_activate(_), Msg::Enter);

        Win {
            model,
            widgets: Widgets {
                _input: input,
                labels,
                window,
            },
        }
    }
}

pub fn execute() {
    load_css();
    Win::run(()).unwrap();
}
