#![feature(vec_remove_item)]

use gtk::*;

mod lib;

/*Structure of app:
 * App -
 *    Body
 *        -Entry
 *        -collection of labels
*/

struct App {
    window: Window,
    body: Body,
}

impl App {
    fn new() -> Self {
        let window = Window::new(WindowType::Toplevel);
        let body = Body::new();
        window.set_default_size(700, 350);
        window.set_decorated(false);
        window.add(&body.content_grid);
        Self::load_css();
        App { window, body }
    }
    fn load_css() -> () {
        let provider = CssProvider::new();
        provider
            .load_from_path("/home/bender/.config/launcher/style.css")
            .unwrap_or_else(|err| {
                eprintln!("Couldn't load CSS, got error: {}", err);
                eprintln!("Continuing on without CSS");
            });
        StyleContext::add_provider_for_screen(
            &gdk::Screen::get_default().expect("failed to initialize css provider"),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }
}

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
        input.connect_changed(move |input| {
            let input = match input.get_text() {
                Some(gstring) => gstring.as_str().to_owned(),
                None => "".to_owned(),
            };
            let bins = lib::sorted_bins(&input);
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
        for i in 0..5 {
            labels.push(Label::new(None));
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
