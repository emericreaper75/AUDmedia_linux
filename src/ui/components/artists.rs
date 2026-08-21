use crate::ui::models::ItemObject;
use gtk4::prelude::*;
use gtk4::{gio, Box as GtkBox, Image, Label, ListView, Orientation, SingleSelection};

pub struct ArtistsView {
    pub container: gtk4::ScrolledWindow,
    pub store: gio::ListStore,
}

impl ArtistsView {
    pub fn new() -> Self {
        let store = gio::ListStore::new::<ItemObject>();
        let selection_model = SingleSelection::new(Some(store.clone()));

        let factory = gtk4::SignalListItemFactory::new();

        factory.connect_setup(move |_, list_item| {
            let list_item = list_item.downcast_ref::<gtk4::ListItem>().unwrap();

            let hbox = GtkBox::builder()
                .orientation(Orientation::Horizontal)
                .spacing(16)
                .margin_start(16)
                .margin_end(16)
                .margin_top(8)
                .margin_bottom(8)
                .build();

            let icon = Image::builder()
                .icon_name("avatar-default-symbolic")
                .pixel_size(48)
                .build();

            let title_label = Label::builder()
                .halign(gtk4::Align::Start)
                .valign(gtk4::Align::Center)
                .ellipsize(gtk4::pango::EllipsizeMode::End)
                .build();
            title_label.add_css_class("heading");

            hbox.append(&icon);
            hbox.append(&title_label);

            list_item.set_child(Some(&hbox));
        });

        factory.connect_bind(move |_, list_item| {
            let list_item = list_item.downcast_ref::<gtk4::ListItem>().unwrap();
            let item_obj = list_item.item().and_downcast::<ItemObject>().unwrap();

            let hbox = list_item.child().and_downcast::<GtkBox>().unwrap();
            let title_label = hbox.last_child().unwrap().downcast::<Label>().unwrap();

            title_label.set_label(&item_obj.title());
        });

        let list_view = ListView::new(Some(selection_model), Some(factory));
        list_view.add_css_class("navigation-sidebar");

        let scrolled_window = gtk4::ScrolledWindow::builder()
            .child(&list_view)
            .hscrollbar_policy(gtk4::PolicyType::Never)
            .build();

        Self {
            container: scrolled_window,
            store,
        }
    }
}
