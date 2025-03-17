use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box, Button, Entry, Label, Orientation, Switch, ComboBoxText};

fn main() {
    let app = Application::builder()
        .application_id("com.example.morsecode")
        .build();

    app.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Morse Code Settings")
            .default_width(350)
            .default_height(250)
            .build();

        let vbox = Box::new(Orientation::Vertical, 10);
        vbox.set_margin_top(20);
        vbox.set_margin_bottom(20);
        vbox.set_margin_start(20);
        vbox.set_margin_end(20);

        // Timeout input
        let timeout_label = Label::new(Some("Timeout (ms):"));
        let timeout_entry = Entry::new();
        timeout_entry.set_placeholder_text(Some("Enter timeout"));

        // Single or double key mode switch
        let mode_label = Label::new(Some("Single Key Mode:"));
        let mode_switch = Switch::new();

        // Input method dropdown
        let input_label = Label::new(Some("Input Method:"));
        let input_dropdown = ComboBoxText::new();
        input_dropdown.append(Some("keyboard"), "Keyboard");
        input_dropdown.append(Some("mouse"), "Mouse");
        input_dropdown.append(Some("external"), "External Device");
        input_dropdown.set_active_id(Some("keyboard"));

        // Save button
        let save_button = Button::with_label("Save Settings");

        // Clone variables for closure
        let timeout_entry_clone = timeout_entry.clone();
        let mode_switch_clone = mode_switch.clone();
        let input_dropdown_clone = input_dropdown.clone();

        save_button.connect_clicked(move |_| {
            let timeout = timeout_entry_clone.text();
            let mode = if mode_switch_clone.is_active() { "Single Key" } else { "Double Key" };
            let input_method = input_dropdown_clone.active_id().unwrap_or_default();
            
            println!("Settings Saved:");
            println!("Timeout: {} ms", timeout);
            println!("Mode: {}", mode);
            println!("Input Method: {}", input_method);
        });

        // Packing UI elements
        vbox.append(&timeout_label);
        vbox.append(&timeout_entry);
        vbox.append(&mode_label);
        vbox.append(&mode_switch);
        vbox.append(&input_label);
        vbox.append(&input_dropdown);
        vbox.append(&save_button);

        window.set_child(Some(&vbox));
        window.present();
    });

    app.run();
}
