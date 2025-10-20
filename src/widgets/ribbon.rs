use gtk::prelude::*;
use gtk::{Box, Orientation, Button, Separator, MenuButton};
use gtk::gio;

pub fn create_ribbon_toolbar() -> Box {
    let ribbon = Box::new(Orientation::Horizontal, 8);
    ribbon.add_css_class("ribbon-toolbar");
    ribbon.set_margin_start(8);
    ribbon.set_margin_end(8);
    ribbon.set_margin_top(4);
    ribbon.set_margin_bottom(4);

    // New (dropdown)
    let new_menu = gio::Menu::new();
    new_menu.append(Some("New Folder"), Some("app.new-folder"));
    new_menu.append(Some("New File"), Some("app.new-file"));
    let new_menu_model = Some(&new_menu);
    let new_btn = MenuButton::new();
    new_btn.set_label("New");
    new_btn.set_menu_model(new_menu_model);

    // Divider
    let divider1 = Separator::new(Orientation::Vertical);

    // Core actions
    let cut_btn = Button::from_icon_name("edit-cut-symbolic");
    cut_btn.set_tooltip_text(Some("Cut"));
    let copy_btn = Button::from_icon_name("edit-copy-symbolic");
    copy_btn.set_tooltip_text(Some("Copy"));
    let paste_btn = Button::from_icon_name("edit-paste-symbolic");
    paste_btn.set_tooltip_text(Some("Paste"));
    let rename_btn = Button::from_icon_name("edit-rename-symbolic");
    rename_btn.set_tooltip_text(Some("Rename"));
    let share_btn = Button::from_icon_name("mail-send-symbolic");
    share_btn.set_tooltip_text(Some("Share"));
    let trash_btn = Button::from_icon_name("user-trash-symbolic");
    trash_btn.set_tooltip_text(Some("Move to Trash"));

    // Divider
    let divider2 = Separator::new(Orientation::Vertical);

    // Sort dropdown
    let sort_menu = gio::Menu::new();
    sort_menu.append(Some("Name"), Some("app.sort-name"));
    sort_menu.append(Some("Size"), Some("app.sort-size"));
    sort_menu.append(Some("Type"), Some("app.sort-type"));
    sort_menu.append(Some("Modified"), Some("app.sort-modified"));
    let sort_btn = MenuButton::new();
    sort_btn.set_label("Sort");
    sort_btn.set_menu_model(Some(&sort_menu));

    // View dropdown
    let view_menu = gio::Menu::new();
    view_menu.append(Some("List"), Some("app.view-list"));
    view_menu.append(Some("Grid"), Some("app.view-grid"));
    let view_btn = MenuButton::new();
    view_btn.set_label("View");
    view_btn.set_menu_model(Some(&view_menu));

    // Spacer
    let spacer = Box::new(Orientation::Horizontal, 0);
    spacer.set_hexpand(true);

    // Hamburger menu (right aligned)
    let more_menu = gio::Menu::new();
    more_menu.append(Some("Settings"), Some("app.settings"));
    more_menu.append(Some("About"), Some("app.about"));
    let more_btn = MenuButton::new();
    more_btn.set_icon_name("open-menu-symbolic");
    more_btn.set_menu_model(Some(&more_menu));

    ribbon.append(&new_btn);
    ribbon.append(&divider1);
    ribbon.append(&cut_btn);
    ribbon.append(&copy_btn);
    ribbon.append(&paste_btn);
    ribbon.append(&rename_btn);
    ribbon.append(&share_btn);
    ribbon.append(&trash_btn);
    ribbon.append(&divider2);
    ribbon.append(&sort_btn);
    ribbon.append(&view_btn);
    ribbon.append(&spacer);
    ribbon.append(&more_btn);

    ribbon
}


