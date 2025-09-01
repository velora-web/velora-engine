use eframe::egui;

#[derive(Clone)]
struct Tab {
    id: usize,
    title: String,
    url: String,
}

#[derive(Default)]
struct BrowserApp {
    tabs: Vec<Tab>,
    next_tab_id: usize,
    active_tab_index: usize,
}

impl BrowserApp {
    fn new() -> Self {
        let mut app = Self {
            tabs: Vec::new(),
            next_tab_id: 0,
            active_tab_index: 0,
        };
        
        // Create initial tab
        app.add_new_tab();
        app
    }
    
    fn add_new_tab(&mut self) {
        let tab_id = self.next_tab_id;
        let new_tab = Tab {
            id: tab_id,
            title: "New Tab".to_string(),
            url: "https://www.google.com".to_string(),
        };
        
        self.tabs.push(new_tab);
        self.active_tab_index = self.tabs.len() - 1;
        self.next_tab_id += 1;
    }
    
    fn close_tab(&mut self, tab_index: usize) {
        if self.tabs.len() <= 1 {
            return; // Don't close the last tab
        }
        
        self.tabs.remove(tab_index);
        
        // Adjust active tab index
        if self.active_tab_index >= self.tabs.len() {
            self.active_tab_index = self.tabs.len() - 1;
        }
    }
    
    fn get_active_tab_mut(&mut self) -> Option<&mut Tab> {
        self.tabs.get_mut(self.active_tab_index)
    }
    
    fn get_active_tab(&self) -> Option<&Tab> {
        self.tabs.get(self.active_tab_index)
    }
}

impl eframe::App for BrowserApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Tab bar
            ui.horizontal(|ui| {
                // Tab list
                for (index, tab) in self.tabs.iter().enumerate() {
                    let is_active = index == self.active_tab_index;
                    
                    ui.horizontal(|ui| {
                        // Tab button
                        if ui.selectable_label(is_active, &tab.title).clicked() {
                            self.active_tab_index = index;
                        }
                        
                        // Close button (only show if more than one tab)
                        if self.tabs.len() > 1 {
                            if ui.button("×").clicked() {
                                self.close_tab(index);
                                return;
                            }
                        }
                    });
                }
                
                // New tab button
                if ui.button("+").clicked() {
                    self.add_new_tab();
                }
            });
            
            ui.separator();
            
            // Browser header with URL bar
            if let Some(active_tab) = self.get_active_tab_mut() {
                ui.horizontal(|ui| {
                    // Back button (placeholder)
                    if ui.button("←").clicked() {
                        // TODO: Implement back navigation
                    }
                    
                    // Forward button (placeholder)
                    if ui.button("→").clicked() {
                        // TODO: Implement forward navigation
                    }
                    
                    // Refresh button (placeholder)
                    if ui.button("⟳").clicked() {
                        // TODO: Implement refresh
                    }
                    
                    // URL input box
                    let url_response = ui.text_edit_singleline(&mut active_tab.url);
                    
                    // Go button
                    if ui.button("Go").clicked() || url_response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                        if !active_tab.url.is_empty() {
                            // TODO: Navigate to URL
                            println!("Navigating to: {}", active_tab.url);
                            
                            // Update tab title
                            active_tab.title = active_tab.url.clone();
                        }
                    }
                });
            }
            
            ui.separator();
            
            // Content area (placeholder)
            if let Some(active_tab) = self.get_active_tab() {
                ui.allocate_ui(ui.available_size(), |ui| {
                    ui.centered_and_justified(|ui| {
                        ui.label("Browser content will appear here");
                        ui.label(format!("Current URL: {}", active_tab.url));
                        ui.label(format!("Active Tab: {}", active_tab.title));
                        ui.label(format!("Tab {} of {}", self.active_tab_index + 1, self.tabs.len()));
                    });
                });
            }
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_min_inner_size([800.0, 600.0]),
        ..Default::default()
    };
    
    eframe::run_native(
        "Velora Browser",
        options,
        Box::new(|_cc| Ok(Box::new(BrowserApp::new()))),
    )
}