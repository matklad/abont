fn main() {
    if let Err(err) = abont_egui::run(Box::new(abont_shell::main)) {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}
