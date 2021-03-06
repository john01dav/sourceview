extern crate gio;
extern crate glib;
extern crate gtk;
extern crate sourceview;

use gio::prelude::*;
use glib::glib_object_wrapper;
use glib::subclass::types::ObjectSubclass;
use glib::translate::*;
use gtk::prelude::*;
use sourceview::prelude::*;
use sourceview::subclass::CompletionProviderImpl;

use glib::subclass::object::ObjectImpl;
use glib::GString;
use sourceview::{CompletionInfo, CompletionProvider};
use std::env;

fn main() {
    let uiapp = gtk::Application::new(None, gio::ApplicationFlags::FLAGS_NONE)
        .expect("Application::new failed");
    uiapp.connect_activate(|app| {
        let win = gtk::ApplicationWindow::new(app);
        win.set_default_size(320, 200);
        win.set_title("Basic example");

        let sourceview = sourceview::View::new();
        let completion = CustomAutocomplete::new();
        let _ = sourceview
            .get_completion()
            .expect("Sourceview has no completion")
            .add_provider(&completion);
        win.add(&sourceview);

        win.show_all();
    });
    uiapp.run(&env::args().collect::<Vec<_>>());
}

glib::glib_wrapper! {
    pub struct CustomAutocomplete(Object<glib::subclass::simple::InstanceStruct<imp::CustomAutocomplete>, glib::subclass::simple::ClassStruct<imp::CustomAutocomplete>, CustomAutocompleteClass>) @implements crate::CompletionProvider;

    match fn {
        get_type => || imp::CustomAutocomplete::get_type().to_glib(),
    }
}

impl CustomAutocomplete {
    fn new() -> CustomAutocomplete {
        glib::Object::new(Self::static_type(), &[])
            .expect("Failed to create CustomAutocomplete instance.")
            .downcast()
            .expect("Created CustomAutocomplete is of wrong type.")
    }
}

mod imp {
    use super::*;
    use sourceview::CompletionProposal;

    pub struct CustomAutocomplete;

    impl ObjectSubclass for CustomAutocomplete {
        const NAME: &'static str = "CustomAutocomplete";
        type ParentType = glib::Object;
        type Instance = glib::subclass::simple::InstanceStruct<Self>;
        type Class = glib::subclass::simple::ClassStruct<Self>;

        glib::glib_object_subclass!();

        fn type_init(type_: &mut glib::subclass::InitializingType<Self>) {
            type_.add_interface::<CompletionProvider>();
        }

        fn new() -> Self {
            CustomAutocomplete
        }
    }

    impl ObjectImpl for CustomAutocomplete {
        glib::glib_object_impl!();
    }

    impl CompletionProviderImpl for CustomAutocomplete {
        fn get_name(&self, _obj: &CompletionProvider) -> GString {
            GString::from("CustomAutocomplete")
        }

        fn get_icon(&self, _obj: &CompletionProvider) -> Option<gdk_pixbuf::Pixbuf> {
            None
        }

        fn get_icon_name(&self, _obj: &CompletionProvider) -> Option<GString> {
            None
        }

        fn get_gicon(&self, _obg: &CompletionProvider) -> Option<gio::Icon> {
            None
        }

        fn populate(&self, obg: &CompletionProvider, context: &sourceview::CompletionContext) {
            for i in 1..10 {
                let item = sourceview::CompletionItem::new(
                    &format!("Test {}", i),
                    &format!("Test {};", i),
                    None,
                    Some("This is a test proposal"),
                );
                context.add_proposals(obg, &[item.upcast::<CompletionProposal>()], i == 9);
            }
        }

        fn get_activation(&self, _obg: &CompletionProvider) -> sourceview::CompletionActivation {
            sourceview::CompletionActivation::all()
        }

        fn provide_match(
            &self,
            _obg: &CompletionProvider,
            _context: &sourceview::CompletionContext,
        ) -> bool {
            true
        }

        fn get_info_widget(
            &self,
            _obg: &CompletionProvider,
            _proposal: &sourceview::CompletionProposal,
        ) -> Option<gtk::Widget> {
            None
        }

        fn update_info(
            &self,
            _obg: &CompletionProvider,
            _proposal: &sourceview::CompletionProposal,
            _info: &CompletionInfo,
        ) {
        }

        fn get_start_iter(
            &self,
            _obg: &CompletionProvider,
            _context: &sourceview::CompletionContext,
            _proposal: &sourceview::CompletionProposal,
            _iter: &gtk::TextIter,
        ) -> bool {
            false
        }

        fn activate_proposal(
            &self,
            _obg: &CompletionProvider,
            _proposal: &sourceview::CompletionProposal,
            _iter: &gtk::TextIter,
        ) -> bool {
            false
        }

        fn get_interactive_delay(&self, _obj: &CompletionProvider) -> i32 {
            0
        }

        fn get_priority(&self, _obg: &CompletionProvider) -> i32 {
            1
        }
    }
}
