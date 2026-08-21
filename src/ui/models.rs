use crate::library::Track;
use gtk4::glib;
use gtk4::glib::subclass::prelude::*;
use std::cell::RefCell;

mod imp {
    use super::*;

    #[derive(Default)]
    pub struct TrackObject {
        pub track: RefCell<Option<Track>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for TrackObject {
        const NAME: &'static str = "AudmediaTrackObject";
        type Type = super::TrackObject;
        type ParentType = glib::Object;
    }

    impl ObjectImpl for TrackObject {}
}

glib::wrapper! {
    pub struct TrackObject(ObjectSubclass<imp::TrackObject>);
}

impl TrackObject {
    pub fn new(track: Track) -> Self {
        let obj: Self = glib::Object::builder().build();
        obj.imp().track.replace(Some(track));
        obj
    }

    pub fn get_track(&self) -> Track {
        self.imp().track.borrow().clone().unwrap()
    }
}

mod item_imp {
    use super::*;

    #[derive(Default)]
    pub struct ItemObject {
        pub title: RefCell<String>,
        pub subtitle: RefCell<String>,
        pub artwork_path: RefCell<Option<String>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ItemObject {
        const NAME: &'static str = "AudmediaItemObject";
        type Type = super::ItemObject;
        type ParentType = glib::Object;
    }

    impl ObjectImpl for ItemObject {}
}

glib::wrapper! {
    pub struct ItemObject(ObjectSubclass<item_imp::ItemObject>);
}

impl ItemObject {
    pub fn new(title: &str, subtitle: &str, artwork_path: Option<String>) -> Self {
        let obj: Self = glib::Object::builder().build();
        obj.imp().title.replace(title.to_string());
        obj.imp().subtitle.replace(subtitle.to_string());
        obj.imp().artwork_path.replace(artwork_path);
        obj
    }

    pub fn title(&self) -> String {
        self.imp().title.borrow().clone()
    }

    pub fn subtitle(&self) -> String {
        self.imp().subtitle.borrow().clone()
    }

    pub fn artwork_path(&self) -> Option<String> {
        self.imp().artwork_path.borrow().clone()
    }
}
