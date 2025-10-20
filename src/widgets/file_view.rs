use gtk::prelude::*;
use gtk::{Box as GtkBox, Orientation, Widget};

use crate::core::file_manager::FileManagerState;
use crate::utils::search::{filter_files_in_directory, FileEntry};
use crate::utils::icon_manager::IconSize;
use std::fs;
use gtk::{ListBox, ListBoxRow, FlowBox, FlowBoxChild, Label, Grid, ScrolledWindow};

// Adapter trait: each view implements this to provide its widget and lifecycle hooks
pub trait FileViewAdapter {
    fn build(&mut self, state: &FileManagerState) -> Widget;
    fn refresh(&mut self, state: &FileManagerState);
    fn set_icon_size(&mut self, _size: i32) { let _ = _size; }
    fn update(&mut self, state: &FileManagerState) {
        // Default: do full refresh. Adapters can override for in-place updates.
        self.refresh(state);
    }
}

// Base FileView container that hosts the active adapter's widget
pub struct FileView {
    container: GtkBox,
    adapter: Option<std::boxed::Box<dyn FileViewAdapter>>,
}

impl FileView {
    pub fn new() -> Self {
        let container = GtkBox::new(Orientation::Vertical, 0);
        FileView { container, adapter: None }
    }

    pub fn widget(&self) -> &GtkBox { &self.container }

    pub fn set_adapter(&mut self, mut adapter: std::boxed::Box<dyn FileViewAdapter>, state: &FileManagerState) {
        if let Some(child) = self.container.first_child() { self.container.remove(&child); }
        let view = adapter.build(state);
        self.container.append(&view);
        self.adapter = Some(adapter);
    }

    pub fn refresh(&mut self, state: &FileManagerState) {
        if let Some(adapter) = self.adapter.as_mut() {
            // Rebuild the adapter view to reflect current directory/state
            if let Some(child) = self.container.first_child() { self.container.remove(&child); }
            let view = adapter.build(state);
            self.container.append(&view);
        }
    }

    pub fn update(&mut self, state: &FileManagerState) {
        if let Some(adapter) = self.adapter.as_mut() {
            adapter.update(state);
        }
    }

    pub fn set_icon_size(&mut self, size: i32) {
        if let Some(adapter) = self.adapter.as_mut() { adapter.set_icon_size(size); }
    }
}

// Placeholder ListView adapter
pub struct ListViewAdapter {
    root: Option<GtkBox>,
}

impl ListViewAdapter { pub fn new() -> Self { Self { root: None } } }

impl FileViewAdapter for ListViewAdapter {
    fn build(&mut self, state: &FileManagerState) -> Widget {
        let root = GtkBox::new(Orientation::Vertical, 0);
        root.set_css_classes(&["fileview-list"]);
        let list = ListBox::new();
        list.set_selection_mode(gtk::SelectionMode::None);

        // Use the search utility to get filtered files
        let files = filter_files_in_directory(&state.current_path(), &state.current_filter, &state.config);

        // Create simple rows with name labels (clicks handled by file_item in grid; here we keep minimal)
        for file_entry in files {
            let row = ListBoxRow::new();
            let row_box = GtkBox::new(Orientation::Horizontal, 8);
            let name_label = Label::new(Some(&file_entry.name));
            name_label.set_xalign(0.0);
            name_label.set_hexpand(true);
            row_box.append(&name_label);
            row.set_child(Some(&row_box));
            list.append(&row);
        }

        root.append(&list);
        let w: Widget = root.clone().upcast();
        self.root = Some(root);
        w
    }

    fn refresh(&mut self, _state: &FileManagerState) {
        // no-op placeholder
    }

    fn update(&mut self, state: &FileManagerState) {
        // In-place update: rebuild the list without replacing the entire widget
        if let Some(root) = &self.root {
            // Clear existing list
            while let Some(child) = root.first_child() {
                root.remove(&child);
            }
            
            let list = ListBox::new();
            list.set_selection_mode(gtk::SelectionMode::None);

            // Use the search utility to get filtered files
            let files = filter_files_in_directory(&state.current_path(), &state.current_filter, &state.config);

            for file_entry in files {
                let row = ListBoxRow::new();
                let row_box = GtkBox::new(Orientation::Horizontal, 8);
                let name_label = Label::new(Some(&file_entry.name));
                name_label.set_xalign(0.0);
                name_label.set_hexpand(true);
                row_box.append(&name_label);
                row.set_child(Some(&row_box));
                list.append(&row);
            }

            root.append(&list);
        }
    }
}

// Improved GridView adapter with fixed grid layout (like Qt HLayout/VLayout)
pub struct GridViewAdapter {
    root: Option<GtkBox>,
    grid: Option<Grid>,
    scrolled: Option<ScrolledWindow>,
    icon_size: IconSize,
}

impl GridViewAdapter { 
    pub fn new() -> Self { 
        Self { 
            root: None, 
            grid: None,
            scrolled: None,
            icon_size: IconSize::Medium 
        } 
    }
    
    pub fn with_icon_size(icon_size: IconSize) -> Self {
        Self { 
            root: None, 
            grid: None,
            scrolled: None,
            icon_size 
        }
    }
    
}

impl FileViewAdapter for GridViewAdapter {
    fn build(&mut self, state: &FileManagerState) -> Widget {
        let root = GtkBox::new(Orientation::Vertical, 0);
        root.set_css_classes(&["fileview-grid"]);
        
        // Create container for the scroll area (like Qt's approach)
        let scroll_container = GtkBox::new(Orientation::Vertical, 0);
        scroll_container.set_hexpand(true);
        scroll_container.set_vexpand(true);
        scroll_container.add_css_class("scroll-container");
        
        // Create scrolled window for the grid
        let scrolled = ScrolledWindow::new();
        scrolled.set_hexpand(true);
        scrolled.set_vexpand(true);
        // Set scroll policy after widget is fully constructed
        scrolled.set_policy(gtk::PolicyType::Never, gtk::PolicyType::Automatic);
        scrolled.set_propagate_natural_width(false); // Don't propagate natural width
        scrolled.set_propagate_natural_height(false);
        
        // Create fixed grid layout (like Qt's HLayout/VLayout)
        let grid = Grid::new();
        grid.set_hexpand(true);  // Fill available width
        grid.set_vexpand(true);  // Allow vertical expansion
        
        // Calculate dynamic spacing based on icon size
        let icon_pixels = self.icon_size.to_pixels();
        let spacing = (icon_pixels as f32 * 0.1) as i32; // 10% of icon size
        let margin = (icon_pixels as f32 * 0.15) as i32; // 15% of icon size
        
        // Calculate tile dimensions and grid layout
        let base_tile_width = 120;
        let scale_factor = (icon_pixels as f32 / 64.0).max(0.5).min(2.0);
        let tile_width = (base_tile_width as f32 * scale_factor) as i32;
        let total_item_width = tile_width + spacing;
        
        // Calculate responsive items per row based on available width
        // Use a reasonable minimum width for calculation (600px as fallback)
        let available_width = 600; // This should be dynamically calculated, but we'll use a reasonable default
        let items_per_row = (available_width / total_item_width).max(1);
        
        // Calculate grid width to fit exactly the number of items per row
        let grid_width = items_per_row * total_item_width - spacing + 2 * margin;
        
        grid.set_row_spacing(spacing as u32);
        grid.set_column_spacing(spacing as u32);
        grid.set_margin_start(margin);
        grid.set_margin_end(margin);
        // Set fixed width to prevent horizontal overflow
        grid.set_size_request(grid_width, -1);
        grid.set_margin_top(margin);
        grid.set_margin_bottom(margin);
        grid.set_halign(gtk::Align::Center); // Center the grid instead of filling
        grid.set_valign(gtk::Align::Start);
        
        // items_per_row is already calculated above
        
        // Use the search utility to get filtered files
        let files = filter_files_in_directory(&state.current_path(), &state.current_filter, &state.config);
        
        // Add files to grid with fixed positions
        for (index, file_entry) in files.iter().enumerate() {
            let row = (index as i32) / items_per_row;
            let col = (index as i32) % items_per_row;
            
            let btn = crate::widgets::file_item::create_file_item_with_size(
                &file_entry.icon, 
                &file_entry.name, 
                &file_entry.file_type, 
                file_entry.path.clone(), 
                &state.config,
                self.icon_size.to_pixels()
            );
            
            // Attach to grid at specific position
            grid.attach(&btn, col, row, 1, 1);
        }
        
        // Add empty placeholder items to fill the grid and prevent dynamic sizing
        let total_items = files.len() as i32;
        let total_rows = (total_items + items_per_row - 1) / items_per_row; // Ceiling division
        let total_cells = total_rows * items_per_row;
        let empty_items_needed = total_cells - total_items;
        
        for i in 0..empty_items_needed {
            let empty_item = self.create_empty_placeholder();
            let row = (total_items + i) / items_per_row;
            let col = (total_items + i) % items_per_row;
            grid.attach(&empty_item, col, row, 1, 1);
        }
        
        // GTK's built-in scroll behavior should work fine for vertical scrolling
        
        scrolled.set_child(Some(&grid));
        scroll_container.append(&scrolled);
        root.append(&scroll_container);
        
        let w: Widget = root.clone().upcast();
        self.root = Some(root);
        self.grid = Some(grid);
        self.scrolled = Some(scrolled);
        w
    }

    fn refresh(&mut self, _state: &FileManagerState) {
        // no-op placeholder
    }

    fn update(&mut self, state: &FileManagerState) {
        // In-place update: clear and rebuild grid without replacing entire widget
        if let Some(grid) = &self.grid {
            // Clear all children from grid
            while let Some(child) = grid.first_child() {
                grid.remove(&child);
            }
            
            // Calculate dynamic spacing and items per row based on icon size
            let icon_pixels = self.icon_size.to_pixels();
            let spacing = (icon_pixels as f32 * 0.1) as i32; // 10% of icon size
            let margin = (icon_pixels as f32 * 0.15) as i32; // 15% of icon size
            
            // Calculate tile dimensions and grid layout
            let base_tile_width = 120;
            let scale_factor = (icon_pixels as f32 / 64.0).max(0.5).min(2.0);
            let tile_width = (base_tile_width as f32 * scale_factor) as i32;
            let total_item_width = tile_width + spacing;
            
            // Calculate responsive items per row based on available width
            let available_width = 600; // Same default as in build()
            let items_per_row = (available_width / total_item_width).max(1);
            
            // Calculate grid width to fit exactly the number of items per row
            let grid_width = items_per_row * total_item_width - spacing + 2 * margin;
            
            grid.set_row_spacing(spacing as u32);
            grid.set_column_spacing(spacing as u32);
            grid.set_margin_start(margin);
            grid.set_margin_end(margin);
            // Set fixed width to prevent horizontal overflow
            grid.set_size_request(grid_width, -1);
            grid.set_margin_top(margin);
            grid.set_margin_bottom(margin);
            grid.set_halign(gtk::Align::Center); // Center the grid instead of filling
            grid.set_valign(gtk::Align::Start);
            
            // Use the search utility to get filtered files
            let files = filter_files_in_directory(&state.current_path(), &state.current_filter, &state.config);
            
            // Add files to grid with fixed positions
            for (index, file_entry) in files.iter().enumerate() {
                let row = (index as i32) / items_per_row;
                let col = (index as i32) % items_per_row;
                
                let btn = crate::widgets::file_item::create_file_item_with_size(
                    &file_entry.icon, 
                    &file_entry.name, 
                    &file_entry.file_type, 
                    file_entry.path.clone(), 
                    &state.config,
                    self.icon_size.to_pixels()
                );
                
                // Attach to grid at specific position
                grid.attach(&btn, col, row, 1, 1);
            }
            
            // Add empty placeholder items to fill the grid and prevent dynamic sizing
            let total_items = files.len() as i32;
            let total_rows = (total_items + items_per_row - 1) / items_per_row; // Ceiling division
            let total_cells = total_rows * items_per_row;
            let empty_items_needed = total_cells - total_items;
            
            for i in 0..empty_items_needed {
                let empty_item = self.create_empty_placeholder();
                let row = (total_items + i) / items_per_row;
                let col = (total_items + i) % items_per_row;
                grid.attach(&empty_item, col, row, 1, 1);
            }
            
            // GTK's built-in scroll behavior should work fine for vertical scrolling
        }
    }
    
    fn set_icon_size(&mut self, size: i32) {
        self.icon_size = IconSize::from_pixels(size);
    }
}

impl GridViewAdapter {
    /// Create an empty placeholder widget to fill grid cells and prevent dynamic sizing
    fn create_empty_placeholder(&self) -> gtk::Widget {
        let placeholder = gtk::Box::new(Orientation::Vertical, 0);
        
        // Calculate dynamic size based on icon size
        let icon_pixels = self.icon_size.to_pixels();
        let base_width = 120;
        let base_height = 100;
        let scale_factor = (icon_pixels as f32 / 64.0).max(0.5).min(2.0);
        let item_width = (base_width as f32 * scale_factor) as i32;
        let item_height = (base_height as f32 * scale_factor) as i32;
        
        placeholder.set_size_request(item_width, item_height);
        placeholder.add_css_class("empty-placeholder");
        
        // Make it invisible but still take up space
        placeholder.set_opacity(0.0);
        
        placeholder.upcast()
    }
}


