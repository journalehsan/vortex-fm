use gtk::prelude::*;
use gtk::{Dialog, Label, Box, Orientation, Button, Entry, FileChooserDialog, FileChooserAction, ResponseType};
use std::path::PathBuf;
use crate::core::file_operations::*;
use crate::core::navigation::get_global_state;

pub fn show_rename_dialog(path: &PathBuf) {
    let dialog = Dialog::new();
    dialog.set_title(Some("Rename"));
    dialog.set_default_size(300, 150);
    dialog.set_modal(true);
    
    let content = dialog.content_area();
    let vbox = Box::new(Orientation::Vertical, 12);
    vbox.set_margin_start(20);
    vbox.set_margin_end(20);
    vbox.set_margin_top(20);
    vbox.set_margin_bottom(20);
    
    // Current name
    let current_name = path.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("Unknown")
        .to_string();
    
    let name_label = Label::new(Some("New name:"));
    vbox.append(&name_label);
    
    let name_entry = Entry::new();
    name_entry.set_text(&current_name);
    name_entry.set_hexpand(true);
    vbox.append(&name_entry);
    
    // Buttons
    let button_box = Box::new(Orientation::Horizontal, 8);
    button_box.set_halign(gtk::Align::End);
    
    let cancel_btn = Button::with_label("Cancel");
    let dialog_clone = dialog.clone();
    cancel_btn.connect_clicked(move |_| {
        dialog_clone.close();
    });
    
    let rename_btn = Button::with_label("Rename");
    rename_btn.add_css_class("suggested-action");
    let path_clone = path.clone();
    let dialog_clone2 = dialog.clone();
    rename_btn.connect_clicked(move |_| {
        let new_name = name_entry.text().to_string();
        if !new_name.is_empty() && new_name != current_name {
            if let Err(e) = rename_file(&path_clone, &new_name) {
                eprintln!("Error renaming file: {}", e);
            } else {
                // Refresh the file list
                if let Some(state_rc) = get_global_state() {
                    let state = state_rc.borrow();
                    state.refresh_ui();
                }
            }
        }
        dialog_clone2.close();
    });
    
    button_box.append(&cancel_btn);
    button_box.append(&rename_btn);
    vbox.append(&button_box);
    
    content.append(&vbox);
    dialog.present();
}

pub fn show_delete_confirmation(path: &PathBuf) {
    let dialog = Dialog::new();
    dialog.set_title(Some("Delete Confirmation"));
    dialog.set_default_size(300, 150);
    dialog.set_modal(true);
    
    let content = dialog.content_area();
    let vbox = Box::new(Orientation::Vertical, 12);
    vbox.set_margin_start(20);
    vbox.set_margin_end(20);
    vbox.set_margin_top(20);
    vbox.set_margin_bottom(20);
    
    let file_name = path.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("Unknown")
        .to_string();
    
    let message = if path.is_dir() {
        format!("Are you sure you want to delete the folder '{}' and all its contents?", file_name)
    } else {
        format!("Are you sure you want to delete the file '{}'?", file_name)
    };
    
    let message_label = Label::new(Some(&message));
    message_label.set_wrap(true);
    vbox.append(&message_label);
    
    // Buttons
    let button_box = Box::new(Orientation::Horizontal, 8);
    button_box.set_halign(gtk::Align::End);
    
    let cancel_btn = Button::with_label("Cancel");
    let dialog_clone = dialog.clone();
    cancel_btn.connect_clicked(move |_| {
        dialog_clone.close();
    });
    
    let delete_btn = Button::with_label("Delete");
    delete_btn.add_css_class("destructive-action");
    let path_clone = path.clone();
    let dialog_clone2 = dialog.clone();
    delete_btn.connect_clicked(move |_| {
        if let Err(e) = delete_file(&path_clone) {
            eprintln!("Error deleting file: {}", e);
        } else {
            // Refresh the file list
            if let Some(state_rc) = get_global_state() {
                let state = state_rc.borrow();
                state.refresh_ui();
            }
        }
        dialog_clone2.close();
    });
    
    button_box.append(&cancel_btn);
    button_box.append(&delete_btn);
    vbox.append(&button_box);
    
    content.append(&vbox);
    dialog.present();
}

pub fn show_copy_dialog(path: &PathBuf) {
    let dialog = FileChooserDialog::new(
        Some("Choose destination folder"),
        None::<&gtk::Window>,
        FileChooserAction::SelectFolder,
        &[("Cancel", ResponseType::Cancel), ("Copy", ResponseType::Accept)]
    );
    
    dialog.set_modal(true);
    
    let path_clone = path.clone();
    dialog.connect_response(move |dialog, response| {
        if response == ResponseType::Accept {
            if let Some(dest_path) = dialog.file() {
                if let Some(dest_path) = dest_path.path() {
                    if let Err(e) = copy_file(&path_clone, &dest_path) {
                        eprintln!("Error copying file: {}", e);
                    } else {
                        // Refresh the file list
                        if let Some(state_rc) = get_global_state() {
                            let state = state_rc.borrow();
                            state.refresh_ui();
                        }
                    }
                }
            }
        }
        dialog.close();
    });
    
    dialog.present();
}

pub fn show_move_dialog(path: &PathBuf) {
    let dialog = FileChooserDialog::new(
        Some("Choose destination folder"),
        None::<&gtk::Window>,
        FileChooserAction::SelectFolder,
        &[("Cancel", ResponseType::Cancel), ("Move", ResponseType::Accept)]
    );
    
    dialog.set_modal(true);
    
    let path_clone = path.clone();
    dialog.connect_response(move |dialog, response| {
        if response == ResponseType::Accept {
            if let Some(dest_path) = dialog.file() {
                if let Some(dest_path) = dest_path.path() {
                    if let Err(e) = move_file(&path_clone, &dest_path) {
                        eprintln!("Error moving file: {}", e);
                    } else {
                        // Refresh the file list
                        if let Some(state_rc) = get_global_state() {
                            let state = state_rc.borrow();
                            state.refresh_ui();
                        }
                    }
                }
            }
        }
        dialog.close();
    });
    
    dialog.present();
}
