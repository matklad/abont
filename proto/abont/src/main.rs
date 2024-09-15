struct App;

impl abont_egui::AbontApp for App {
    fn start(self, abont_api: impl abont_api::AbontApi + Send + 'static) {
        std::thread::spawn(move || {
            let rt = tokio::runtime::Builder::new_current_thread().build().unwrap();
            rt.block_on(abont_shell::main(&abont_api));
        });
    }
}

fn main() {
    if let Err(err) = abont_egui::run(App) {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}
