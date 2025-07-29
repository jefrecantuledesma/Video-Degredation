use gtk4::prelude::*;
use gtk4::{
    glib, Application, ApplicationWindow, Box, Button, FileChooserAction, FileChooserDialog,
    FileFilter, Label, Orientation, ResponseType,
};
use std::path::PathBuf;

fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id("com.retro.video.editor")
        .build();

    app.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Retro Video Editor")
            .default_width(800)
            .default_height(600)
            .build();

        // Create a vertical box
        let main_box = Box::new(Orientation::Vertical, 10);
        main_box.set_margin_start(20);
        main_box.set_margin_end(20);
        main_box.set_margin_top(20);
        main_box.set_margin_bottom(20);

        // Video area placeholder
        let video_label = Label::new(Some("Video preview area"));
        video_label.set_vexpand(true);
        video_label.set_size_request(640, 480);

        // Status label
        let status_label = Label::new(Some("No video loaded"));
        status_label.set_margin_top(10);

        // Button container
        let button_box = Box::new(Orientation::Horizontal, 10);
        button_box.set_halign(gtk4::Align::Center);
        button_box.set_margin_top(10);

        // Create buttons
        let load_button = Button::with_label("Load Video");
        let apply_button = Button::with_label("Apply Retro Effect");
        let export_button = Button::with_label("Export Video");

        // Initially disable effect and export buttons
        apply_button.set_sensitive(false);
        export_button.set_sensitive(false);

        // Connect button handlers
        let apply_button_clone = apply_button.clone();
        let status_label_clone = status_label.clone();
        let video_label_clone = video_label.clone();
        let window_clone = window.clone();
        load_button.connect_clicked(move |_| {
            let apply_button = apply_button_clone.clone();
            let status_label = status_label_clone.clone();
            let video_label = video_label_clone.clone();

            // Create native GTK file dialog
            let dialog = FileChooserDialog::new(
                Some("Select MP4 Video"),
                Some(&window_clone),
                FileChooserAction::Open,
                &[
                    ("Cancel", ResponseType::Cancel),
                    ("Open", ResponseType::Accept),
                ],
            );

            // Add file filter for MP4 files
            let filter = FileFilter::new();
            filter.set_name(Some("MP4 Video"));
            filter.add_pattern("*.mp4");
            filter.add_pattern("*.MP4");
            dialog.add_filter(&filter);

            let all_filter = FileFilter::new();
            all_filter.set_name(Some("All Files"));
            all_filter.add_pattern("*");
            dialog.add_filter(&all_filter);

            dialog.connect_response(move |dialog, response| {
                if response == ResponseType::Accept {
                    if let Some(file) = dialog.file() {
                        if let Some(path) = file.path() {
                            let filename = path
                                .file_name()
                                .and_then(|n| n.to_str())
                                .unwrap_or("unknown");
                            status_label.set_text(&format!("Loaded: {}", filename));
                            video_label.set_text(&format!("Video: {}", filename));
                            apply_button.set_sensitive(true);
                            println!("Loaded video: {:?}", path);
                        }
                    }
                }
                dialog.close();
            });

            dialog.show();
        });

        let export_button_clone = export_button.clone();
        let status_label_clone = status_label.clone();
        apply_button.connect_clicked(move |_| {
            println!("Apply effect clicked!");
            status_label_clone.set_text("Effect applied!");
            export_button_clone.set_sensitive(true);
        });

        let status_label_clone = status_label.clone();
        let window_clone = window.clone();
        export_button.connect_clicked(move |_| {
            let status_label = status_label_clone.clone();

            // Create native GTK save dialog
            let dialog = FileChooserDialog::new(
                Some("Save Retro Video As"),
                Some(&window_clone),
                FileChooserAction::Save,
                &[
                    ("Cancel", ResponseType::Cancel),
                    ("Save", ResponseType::Accept),
                ],
            );

            // Set default filename
            dialog.set_current_name("retro_video.mp4");

            // Add file filter
            let filter = FileFilter::new();
            filter.set_name(Some("MP4 Video"));
            filter.add_pattern("*.mp4");
            dialog.add_filter(&filter);

            dialog.connect_response(move |dialog, response| {
                if response == ResponseType::Accept {
                    if let Some(file) = dialog.file() {
                        if let Some(path) = file.path() {
                            status_label.set_text(&format!("Exported to: {}", path.display()));
                            println!("Export path: {:?}", path);
                        }
                    }
                }
                dialog.close();
            });

            dialog.show();
        });

        // Pack buttons
        button_box.append(&load_button);
        button_box.append(&apply_button);
        button_box.append(&export_button);

        // Pack everything
        main_box.append(&video_label);
        main_box.append(&status_label);
        main_box.append(&button_box);

        window.set_child(Some(&main_box));
        window.present();
    });

    app.run()
}
