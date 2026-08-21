use crate::ui::models::TrackObject;
use gtk4::prelude::*;
use gtk4::{
    gio, Box as GtkBox, Image, Label, ListView, Orientation, SignalListItemFactory, SingleSelection,
};

pub struct QueueView {
    pub container: gtk4::ScrolledWindow,
    pub store: gio::ListStore,
    pub selection_model: SingleSelection,
    #[allow(dead_code)]
    pub list_view: ListView,
}

impl QueueView {
    pub fn new() -> Self {
        let store = gio::ListStore::new::<TrackObject>();
        let selection_model = SingleSelection::new(Some(store.clone()));

        let factory = SignalListItemFactory::new();

        factory.connect_setup(move |_, list_item| {
            let list_item = list_item.downcast_ref::<gtk4::ListItem>().unwrap();

            let hbox = GtkBox::builder()
                .orientation(Orientation::Horizontal)
                .spacing(12)
                .margin_start(12)
                .margin_end(12)
                .margin_top(6)
                .margin_bottom(6)
                .build();

            let icon = Image::builder()
                .icon_name("audio-x-generic-symbolic")
                .pixel_size(48)
                .build();

            let vbox = GtkBox::builder()
                .orientation(Orientation::Vertical)
                .valign(gtk4::Align::Center)
                .build();

            let title_label = Label::builder()
                .halign(gtk4::Align::Start)
                .ellipsize(gtk4::pango::EllipsizeMode::End)
                .build();
            title_label.add_css_class("heading");

            let artist_label = Label::builder()
                .halign(gtk4::Align::Start)
                .ellipsize(gtk4::pango::EllipsizeMode::End)
                .build();
            artist_label.add_css_class("dim-label");

            vbox.append(&title_label);
            vbox.append(&artist_label);

            hbox.append(&icon);
            hbox.append(&vbox);

            list_item.set_child(Some(&hbox));
        });

        factory.connect_bind(move |_, list_item| {
            let list_item = list_item.downcast_ref::<gtk4::ListItem>().unwrap();
            let track_obj = list_item.item().and_downcast::<TrackObject>().unwrap();
            let track = track_obj.get_track();

            let hbox = list_item.child().and_downcast::<GtkBox>().unwrap();
            let vbox = hbox.last_child().and_downcast::<GtkBox>().unwrap();

            let title_label = vbox.first_child().and_downcast::<Label>().unwrap();
            let artist_label = vbox.last_child().and_downcast::<Label>().unwrap();

            let title = track.metadata.title.unwrap_or_else(|| {
                track
                    .path
                    .file_name()
                    .map(|f| f.to_string_lossy().to_string())
                    .unwrap_or_else(|| "Unknown".to_string())
            });
            let artist = track
                .metadata
                .artist
                .unwrap_or_else(|| "Unknown Artist".to_string());

            title_label.set_label(&title);
            artist_label.set_label(&artist);
        });

        let list_view = ListView::new(Some(selection_model.clone()), Some(factory));
        list_view.add_css_class("navigation-sidebar");

        let scrolled_window = gtk4::ScrolledWindow::builder()
            .child(&list_view)
            .hscrollbar_policy(gtk4::PolicyType::Never)
            .build();

        Self {
            container: scrolled_window,
            store,
            selection_model,
            list_view,
        }
    }
}
