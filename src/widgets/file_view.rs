use gtk::prelude::*;
use gtk::{Box as GtkBox, Orientation, Widget};

use crate::core::file_manager::FileManagerState;

// Adapter trait: each view implements this to provide its widget and lifecycle hooks
pub trait FileViewAdapter {
    fn build(&mut self, state: &FileManagerState) -> Widget;
    fn refresh(&mut self, state: &FileManagerState);
    fn set_icon_size(&mut self, _size: i32) { let _ = _size; }
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
        if let Some(adapter) = self.adapter.as_mut() { adapter.refresh(state); }
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
    fn build(&mut self, _state: &FileManagerState) -> Widget {
        let root = GtkBox::new(Orientation::Vertical, 0);
        root.set_css_classes(&["fileview-list"]);
        let w: Widget = root.clone().upcast();
        self.root = Some(root);
        w
    }

    fn refresh(&mut self, _state: &FileManagerState) {
        // no-op placeholder
    }
}

// Placeholder GridView adapter
pub struct GridViewAdapter {
    root: Option<GtkBox>,
    icon_size: i32,
}

impl GridViewAdapter { pub fn new() -> Self { Self { root: None, icon_size: 64 } } }

impl FileViewAdapter for GridViewAdapter {
    fn build(&mut self, _state: &FileManagerState) -> Widget {
        let root = GtkBox::new(Orientation::Vertical, 0);
        root.set_css_classes(&["fileview-grid"]);
        let w: Widget = root.clone().upcast();
        self.root = Some(root);
        w
    }

    fn refresh(&mut self, _state: &FileManagerState) { }

    fn set_icon_size(&mut self, size: i32) { self.icon_size = size; }
}


