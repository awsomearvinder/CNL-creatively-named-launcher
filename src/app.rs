use gtk::*;
use relm::{connect, Relm, Update, Widget};
use relm_derive::Msg;
use std::{path, process};

use launcher_lib as lib;
const NUM_LABELS: usize = 5;

//note: this will always be atleast as big as the minimum required space to fit
//everything on screen.
const WIDTH: i32 = 700;
const HEIGHT: i32 = 0;

///Holds relevant data for the UI,
///does NOT hold the UI.
pub struct Model {
    searcher: lib::Searcher,
    bins: Vec<lib::bin::Bin>,
}

///Events from the UI
#[derive(Msg, Debug)]
pub enum Msg {
    UpdatedInput(String),
    Enter,
    Quit,
}

///Holds all data between model and widget.
struct Win {
    model: Model,
    widgets: Widgets,
}

///Holds all UI components
struct Widgets {
    _input: gtk::Entry,
    labels: Vec<gtk::Label>,
    window: gtk::Window,
}

///Defines what to do on a update.
impl Update for Win {
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
                model.bins = searcher.sorted_bins(&c).into_iter().cloned().collect();
                let mut output_values = self.model.bins.iter().map(|x| x.name().to_owned());
                for label in &mut self.widgets.labels {
                    label.set_text(&output_values.next().unwrap_or_else(|| "".into()))
                }
            }
            Msg::Quit => std::process::exit(0),
            Msg::Enter => {
                let is_embed = std::env::args().into_iter().any(|x| &x == "--embed");
                if is_embed {
                    println!("{}", self.model.bins[0].exec_cmd());
                } else {
                    self.model.bins[0].exec().unwrap();
                    self.widgets.window.hide();
                    std::thread::sleep(std::time::Duration::from_millis(500));
                }
                std::process::exit(0);
            }
        }
    }
}

///Defines how the UI works.
impl Widget for Win {
    type Root = gtk::Window;

    fn root(&self) -> Self::Root {
        self.widgets.window.clone()
    }

    ///This is where we build the UI.
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
        window.set_default_size(WIDTH, HEIGHT);
        window.set_resizable(false);
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

//Filesystems are an arcane beauty
///This loads CSS for GTK
fn load_css() {
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

///Execute the app.
pub fn execute() {
    load_css();
    Win::run(()).unwrap();
}
