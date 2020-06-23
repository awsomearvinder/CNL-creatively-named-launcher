use gtk::*;

mod app;
fn main() {
    gtk::init().unwrap_or_else(|_err| {
        eprintln!("failed to initialize gtk");
        std::process::exit(1);
    });
    let is_embed = std::env::args().into_iter().any(|x| &x == "--embed");
    app::execute();
}
