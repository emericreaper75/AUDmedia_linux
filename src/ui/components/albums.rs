use crate::ui::models::ItemObject;
use gtk4::prelude::*;
use gtk4::{gio, Box as GtkBox, GridView, Image, Label, Orientation, SingleSelection};

pub struct AlbumsView {
    pub container: gtk4::ScrolledWindow,
    pub store: gio::ListStore,
}

impl AlbumsView {
    pub fn new() -> Self {
        let store = gio::ListStore::new::<ItemObject>();
        let selection_model = SingleSelection::new(Some(store.clone()));

        let factory = gtk4::SignalListItemFactory::new();

        factory.connect_setup(move |_, list_item| {
            let list_item = list_item.downcast_ref::<gtk4::ListItem>().unwrap();

            let vbox = GtkBox::builder()
                .orientation(Orientation::Vertical)
                .spacing(4)
                .margin_start(8)
                .margin_end(8)
                .margin_top(8)
                .margin_bottom(8)
                .halign(gtk4::Align::Center)
                .build();

            let icon = Image::builder()
                .icon_name("media-optical-symbolic")
                .pixel_size(128)
                .build();
            icon.add_css_class("card");

            let title_label = Label::builder()
                .halign(gtk4::Align::Center)
                .ellipsize(gtk4::pango::EllipsizeMode::End)
                .max_width_chars(15)
                .build();
            title_label.add_css_class("heading");

            let subtitle_label = Label::builder()
                .halign(gtk4::Align::Center)
                .ellipsize(gtk4::pango::EllipsizeMode::End)
                .max_width_chars(15)
                .build();
            subtitle_label.add_css_class("dim-label");

            vbox.append(&icon);
            vbox.append(&title_label);
            vbox.append(&subtitle_label);

            list_item.set_child(Some(&vbox));
        });

        factory.connect_bind(move |_, list_item| {
            let list_item = list_item.downcast_ref::<gtk4::ListItem>().unwrap();
            let item_obj = list_item.item().and_downcast::<ItemObject>().unwrap();

            let vbox = list_item.child().and_downcast::<GtkBox>().unwrap();

            let icon = vbox.first_child().unwrap().downcast::<Image>().unwrap();
            let title_label = icon.next_sibling().unwrap().downcast::<Label>().unwrap();
            let subtitle_label = vbox.last_child().unwrap().downcast::<Label>().unwrap();

            title_label.set_label(&item_obj.title());
            subtitle_label.set_label(&item_obj.subtitle());

            if let Some(ref path) = item_obj.artwork_path() {
                icon.set_from_file(Some(path));
            } else {
                icon.set_icon_name(Some("media-optical-symbolic"));
            }
        });

        let grid_view = GridView::new(Some(selection_model), Some(factory));
        grid_view.set_max_columns(10);
        grid_view.set_min_columns(2);
        grid_view.set_enable_rubberband(true);

        let scrolled_window = gtk4::ScrolledWindow::builder()
            .child(&grid_view)
            .hscrollbar_policy(gtk4::PolicyType::Never)
            .build();

        Self {
            container: scrolled_window,
            store,
        }
    }
}
