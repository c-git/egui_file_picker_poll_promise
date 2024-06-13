type SaveLoadPromise = Option<poll_promise::Promise<Option<String>>>;
pub struct BrowseApp {
    promise: SaveLoadPromise,
    sample_text: String,
}

impl Default for BrowseApp {
    fn default() -> Self {
        Self {
            sample_text: "This is some sample text".into(),
            promise: None,
        }
    }
}

impl BrowseApp {
    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }

    fn buttons_save_load(&mut self, ui: &mut egui::Ui) {
        // a simple button opening the dialog
        if ui.button("📂 Open text file").clicked() {
            // Context is wrapped in an Arc so it's cheap to clone as per:
            // > Context is cheap to clone, and any clones refers to the same mutable data (Context uses refcounting internally).
            // Taken from https://docs.rs/egui/0.24.1/egui/struct.Context.html
            let ctx = ui.ctx().clone();
            self.promise = execute(async move {
                let file = rfd::AsyncFileDialog::new().pick_file().await?; // Returns None if file is None
                let text = file.read().await;

                // Uncomment the following line to simulate taking long to load, only works on native
                // tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

                // If not present screen will not refresh until next paint (comment out to test, works better with the sleep above to demonstrate)
                ctx.request_repaint();

                Some(String::from_utf8_lossy(&text).to_string())
            });
        }

        if ui.button("💾 Save text to file").clicked() {
            let contents = self.sample_text.clone();
            self.promise = execute(async move {
                let file = rfd::AsyncFileDialog::new().save_file().await;
                if let Some(file) = file {
                    _ = file.write(contents.as_bytes()).await;
                }
                None
            });
        }
    }
}

impl eframe::App for BrowseApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // assign sample text once it comes in
        if let Some(promise) = &self.promise {
            if promise.ready().is_some() {
                // Clear promise and take the value out
                // Doesn't matter for string as we can just clone it but depending on the type you have
                // you may not be able to easily clone it and would prefer get the owned value
                let mut temp = None;
                std::mem::swap(&mut temp, &mut self.promise);

                let owned_promise = temp.expect("we got here because it was some");
                let inner_option = owned_promise.block_and_take(); // This should be fine because we know it's ready

                if let Some(text) = inner_option {
                    self.sample_text = text;
                } else {
                    // User probably cancelled or it was saving but the promise completed either way
                }
            }
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.text_edit_multiline(&mut self.sample_text);
            if self.promise.is_none() {
                self.buttons_save_load(ui);
            } else {
                ui.spinner();
            }
        });
    }
}

#[cfg(target_arch = "wasm32")]
fn execute<F>(f: F) -> SaveLoadPromise
where
    F: std::future::Future<Output = Option<String>> + 'static,
{
    Some(poll_promise::Promise::spawn_local(f))
}

#[cfg(not(target_arch = "wasm32"))]
fn execute<F>(f: F) -> SaveLoadPromise
where
    F: std::future::Future<Output = Option<String>> + std::marker::Send + 'static,
{
    Some(poll_promise::Promise::spawn_async(f))
}
