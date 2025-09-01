use eframe::egui;
use tokio::runtime::Runtime;
use anyhow::Result;
use log::{info, error};
use std::sync::mpsc;

// Velora engine imports
use velora_parser::HtmlParser;
use velora_dom::{Document, Node, NodeType};
use velora_net::HttpClient;

#[derive(Clone)]
struct Tab {
    #[allow(dead_code)]
    id: usize,
    title: String,
    url: String,
    content: Option<String>,
    dom: Option<Document>,
    loading: bool,
}

#[derive(Clone)]
enum Action {
    Switch(usize),
    Close(usize),
    New,
    Navigate(String),
}

#[derive(Clone)]
struct NavigationRequest {
    url: String,
    tab_index: usize,
}

#[derive(Default)]
struct BrowserApp {
    tabs: Vec<Tab>,
    next_tab_id: usize,
    active_tab_index: usize,
    runtime: Option<Runtime>,
    http_client: Option<HttpClient>,
    _html_parser: HtmlParser, // Mark as intentionally unused for now
    navigation_queue: Vec<NavigationRequest>,
    result_sender: Option<mpsc::Sender<NavigationResult>>,
    result_receiver: Option<mpsc::Receiver<NavigationResult>>,
}

#[derive(Clone)]
struct NavigationResult {
    tab_index: usize,
    success: bool,
    content: Option<String>,
    dom: Option<Document>,
    title: Option<String>,
    error: Option<String>,
}

impl BrowserApp {
    fn new() -> Self {
        let (sender, receiver) = mpsc::channel();
        let mut app = Self {
            tabs: Vec::new(),
            next_tab_id: 0,
            active_tab_index: 0,
            runtime: None,
            http_client: None,
            _html_parser: HtmlParser::new(), // Mark as intentionally unused for now
            navigation_queue: Vec::new(),
            result_sender: Some(sender),
            result_receiver: Some(receiver),
        };
        
        // Initialize async runtime
        app.runtime = Some(Runtime::new().unwrap());
        
        // Initialize HTTP client
        if let Some(rt) = &app.runtime {
            app.http_client = rt.block_on(async {
                HttpClient::new()
            }).ok();
        }
        
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
            content: None,
            dom: None,
            loading: false,
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
    
    
    
    fn process_navigation_queue(&mut self) {
        // Process all queued navigation requests
        while let Some(request) = self.navigation_queue.pop() {
            if let Some(rt) = &self.runtime {
                let url = request.url.clone();
                let tab_index = request.tab_index;
                
                // Update tab loading state
                if let Some(tab) = self.tabs.get_mut(tab_index) {
                    tab.loading = true;
                    tab.title = url.clone();
                }
                
                // Process navigation asynchronously
                let sender = self.result_sender.clone();
                rt.block_on(async {
                    // Create a new HTTP client for this request
                    if let Ok(client) = HttpClient::new() {
                        match client.get(&url).await {
                            Ok(response) => {
                                if response.status.is_success() {
                                    if let Ok(html_content) = response.text() {
                                        info!("Received HTML content: {} bytes", html_content.len());
                                        
                                        // Parse HTML into DOM
                                        let html_parser = HtmlParser::new();
                                        match html_parser.parse_html(&html_content) {
                                            Ok(document) => {
                                                info!("Successfully parsed HTML into DOM");
                                                
                                                // Send success result through channel
                                                if let Some(sender) = sender {
                                                    let title = extract_title_from_html(&html_content);
                                                    let _ = sender.send(NavigationResult {
                                                        tab_index,
                                                        success: true,
                                                        content: Some(html_content),
                                                        dom: Some(document),
                                                        title: Some(title),
                                                        error: None,
                                                    });
                                                }
                                            }
                                            Err(e) => {
                                                error!("Failed to parse HTML: {:?}", e);
                                                
                                                // Send error result through channel
                                                if let Some(sender) = sender {
                                                    let _ = sender.send(NavigationResult {
                                                        tab_index,
                                                        success: false,
                                                        content: Some(format!("Error parsing HTML: {:?}", e)),
                                                        dom: None,
                                                        title: None,
                                                        error: Some(format!("HTML parsing failed: {:?}", e)),
                                                    });
                                                }
                                            }
                                        }
                                    } else {
                                        error!("Failed to decode response as text");
                                        
                                        // Send error result through channel
                                        if let Some(sender) = sender {
                                            let _ = sender.send(NavigationResult {
                                                tab_index,
                                                success: false,
                                                content: Some("Error: Failed to decode response as text".to_string()),
                                                dom: None,
                                                title: None,
                                                error: Some("Failed to decode response as text".to_string()),
                                            });
                                        }
                                    }
                                } else {
                                    error!("HTTP request failed: {} {}", response.status.code, response.status.reason);
                                    
                                    // Send error result through channel
                                    if let Some(sender) = sender {
                                        let _ = sender.send(NavigationResult {
                                            tab_index,
                                            success: false,
                                            content: Some(format!("HTTP Error: {} {}", response.status.code, response.status.reason)),
                                            dom: None,
                                            title: None,
                                            error: Some(format!("HTTP request failed: {} {}", response.status.code, response.status.reason)),
                                        });
                                    }
                                }
                            }
                            Err(e) => {
                                error!("Request failed: {:?}", e);
                                
                                // Send error result through channel
                                if let Some(sender) = sender {
                                    let _ = sender.send(NavigationResult {
                                        tab_index,
                                        success: false,
                                        content: Some(format!("Request Error: {:?}", e)),
                                        dom: None,
                                        title: None,
                                        error: Some(format!("Request failed: {:?}", e)),
                                    });
                                }
                            }
                        }
                    } else {
                        error!("Failed to create HTTP client");
                        
                        // Send error result through channel
                        if let Some(sender) = sender {
                            let _ = sender.send(NavigationResult {
                                tab_index,
                                success: false,
                                content: Some("Error: Failed to create HTTP client".to_string()),
                                dom: None,
                                title: None,
                                error: Some("Failed to create HTTP client".to_string()),
                            });
                        }
                    }
                });
                
                // Update tab loading state
                if let Some(tab) = self.tabs.get_mut(tab_index) {
                    tab.loading = false;
                }
            }
        }
    }
    
    fn process_navigation_results(&mut self) {
        // Process all available navigation results
        if let Some(receiver) = &self.result_receiver {
            while let Ok(result) = receiver.try_recv() {
                if let Some(tab) = self.tabs.get_mut(result.tab_index) {
                    tab.loading = false;
                    
                    if result.success {
                        if let Some(content) = result.content {
                            tab.content = Some(content);
                        }
                        if let Some(dom) = result.dom {
                            tab.dom = Some(dom);
                        }
                        if let Some(title) = result.title {
                            if !title.is_empty() {
                                tab.title = title;
                            }
                        }
                    } else {
                        if let Some(content) = result.content {
                            tab.content = Some(content);
                        }
                        if let Some(error) = result.error {
                            error!("Navigation failed: {}", error);
                        }
                    }
                }
            }
        }
    }
    
    fn render_dom_content(&self, ui: &mut egui::Ui, document: &Document) {
        if let Some(root_node) = document.get_dom_tree().get_root() {
            self.render_node(ui, root_node);
        }
    }
    
    fn render_node(&self, ui: &mut egui::Ui, node: &Node) {
        match &node.node_type {
            NodeType::Element => {
                let tag_name = &node.node_name;
                
                // Render different element types
                match tag_name.as_str() {
                    "h1" | "h2" | "h3" | "h4" | "h5" | "h6" => {
                        if let Some(text) = &node.node_value {
                            ui.heading(text);
                        }
                    }
                    "p" => {
                        if let Some(text) = &node.node_value {
                            ui.label(text);
                        }
                    }
                    "div" => {
                        ui.group(|ui| {
                            if let Some(text) = &node.node_value {
                                ui.label(text);
                            }
                            // Render children
                            for &child_id in &node.child_ids {
                                // TODO: Get child node from DOM tree and render it
                                // For now, just show child count
                                ui.label(format!("Child node: {}", child_id.0));
                            }
                        });
                    }
                    "a" => {
                        if let Some(text) = &node.node_value {
                            if ui.link(text).clicked() {
                                // TODO: Handle link clicks
                                info!("Link clicked: {}", text);
                            }
                        }
                    }
                    _ => {
                        // Generic element rendering
                        if let Some(text) = &node.node_value {
                            ui.label(text);
                        }
                    }
                }
                
                // Render children
                for &child_id in &node.child_ids {
                    // TODO: Get child node from DOM tree and render it
                    // For now, just show child count
                    ui.label(format!("Child node: {}", child_id.0));
                }
            }
            NodeType::Text => {
                if let Some(text) = &node.node_value {
                    ui.label(text);
                }
            }
            _ => {
                // Other node types
                if let Some(text) = &node.node_value {
                    ui.label(text);
                }
            }
        }
    }
}

// Helper function to extract title from HTML content
fn extract_title_from_html(html: &str) -> String {
    if let Some(title_start) = html.find("<title>") {
        if let Some(title_end) = html.find("</title>") {
            if title_end > title_start {
                let title = &html[title_start + 7..title_end];
                return title.trim().to_string();
            }
        }
    }
    
    // Fallback: extract from first h1 or use URL
    if let Some(h1_start) = html.find("<h1>") {
        if let Some(h1_end) = html.find("</h1>") {
            if h1_end > h1_start {
                let h1 = &html[h1_start + 4..h1_end];
                return h1.trim().to_string();
            }
        }
    }
    
    "Untitled".to_string()
}

impl eframe::App for BrowserApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Actions vector for collecting UI actions
            let mut actions = Vec::new();
            
            // Tab bar
            ui.horizontal(|ui| {
                // Tab list
                for (index, tab) in self.tabs.iter().enumerate() {
                    let is_active = index == self.active_tab_index;
                    
                    ui.horizontal(|ui| {
                        // Tab button
                        if ui.selectable_label(is_active, &tab.title).clicked() {
                            actions.push(Action::Switch(index));
                        }
                        
                        // Close button (only show if more than one tab)
                        if self.tabs.len() > 1 && ui.button("×").clicked() {
                            actions.push(Action::Close(index));
                        }
                    });
                }
                
                // New tab button
                if ui.button("+").clicked() {
                    actions.push(Action::New);
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
                    if (ui.button("Go").clicked() || url_response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter))) 
                        && !active_tab.url.is_empty() {
                        
                        // Add navigation action
                        actions.push(Action::Navigate(active_tab.url.clone()));
                    }
                });
            }
            
            // Execute actions after the UI loop
            for action in actions {
                match action {
                    Action::Switch(index) => self.active_tab_index = index,
                    Action::Close(index) => self.close_tab(index),
                    Action::New => self.add_new_tab(),
                    Action::Navigate(url) => {
                        // Queue the navigation request to avoid borrowing issues
                        self.navigation_queue.push(NavigationRequest {
                            url: url.clone(),
                            tab_index: self.active_tab_index,
                        });
                        info!("Navigation queued to: {}", url);
                    }
                }
            }
            
            // Process navigation requests after the UI loop
            self.process_navigation_queue();
            
            // Process navigation results
            self.process_navigation_results();
            
            ui.separator();
            
            // Content area
            if let Some(active_tab) = self.get_active_tab() {
                ui.allocate_ui(ui.available_size(), |ui| {
                    if active_tab.loading {
                        ui.centered_and_justified(|ui| {
                            ui.label("Loading...");
                            ui.label(format!("Fetching: {}", active_tab.url));
                        });
                    } else if let Some(content) = &active_tab.content {
                        if let Some(document) = &active_tab.dom {
                            // Render DOM content
                            self.render_dom_content(ui, document);
                        } else {
                            // Fallback to raw content display
                            ui.label("Raw HTML Content:");
                            ui.separator();
                            ui.text_edit_multiline(&mut content.clone());
                        }
                    } else {
                        ui.centered_and_justified(|ui| {
                            ui.label("Enter a URL and click Go to load a webpage");
                            ui.label(format!("Current URL: {}", active_tab.url));
                            ui.label(format!("Active Tab: {}", active_tab.title));
                            ui.label(format!("Tab {} of {}", self.active_tab_index + 1, self.tabs.len()));
                        });
                    }
                });
            }
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    // Initialize logging
    env_logger::init();
    
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