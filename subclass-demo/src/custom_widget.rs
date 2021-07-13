use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gdk, glib, graphene};

mod imp {
    use super::*;
    use glib::{ParamSpec, Value};
    use once_cell::sync::Lazy;
    use std::cell::RefCell;

    pub struct CustomWidget {
        rgba: RefCell<Option<gdk::RGBA>>,
        label: gtk::Label,
    }

    impl Default for CustomWidget {
        fn default() -> Self {
            Self {
                rgba: RefCell::default(),
                label: gtk::Label::new(None),
            }
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for CustomWidget {
        const NAME: &'static str = "CustomWidget";
        type Type = super::CustomWidget;
        type ParentType = gtk::Widget;
    }

    impl ObjectImpl for CustomWidget {
        fn properties() -> &'static [ParamSpec] {
            static PROPS: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
                vec![ParamSpec::new_boxed(
                    "rgba",
                    "RGBA",
                    "Color RGBA",
                    gdk::RGBA::static_type(),
                    glib::ParamFlags::READWRITE,
                )]
            });
            PROPS.as_ref()
        }

        fn property(&self, _obj: &Self::Type, _id: usize, pspec: &ParamSpec) -> Value {
            match pspec.name() {
                "rgba" => self.rgba.borrow().to_value(),
                _ => unimplemented!(),
            }
        }

        fn set_property(&self, _obj: &Self::Type, _id: usize, value: &Value, pspec: &ParamSpec) {
            match pspec.name() {
                "rgba" => {
                    self.rgba.borrow_mut().replace(value.get().unwrap());
                }
                _ => unimplemented!(),
            }
        }

        fn constructed(&self, obj: &Self::Type) {
            self.label.set_parent(obj);
            self.label.add_css_class("title-1");

            obj.bind_property("rgba", &self.label, "label")
                .transform_to(move |_, val| {
                    let rgba = val.get::<gdk::RGBA>().unwrap();
                    Some(rgba.to_string().to_value())
                })
                .build();
        }

        fn dispose(&self, _obj: &Self::Type) {
            self.label.unparent();
        }
    }

    impl WidgetImpl for CustomWidget {
        fn measure(
            &self,
            _widget: &Self::Type,
            _orientation: gtk::Orientation,
            _for_size: i32,
        ) -> (i32, i32, i32, i32) {
            (250, 250, -1, -1)
        }

        fn size_allocate(&self, _widget: &Self::Type, width: i32, height: i32, baseline: i32) {
            self.label.size_allocate(
                &gtk::Allocation {
                    x: 0,
                    y: 0,
                    width,
                    height,
                },
                baseline,
            );
        }

        fn snapshot(&self, widget: &Self::Type, snapshot: &gtk::Snapshot) {
            let color = self.rgba.borrow().unwrap_or_else(|| gdk::RGBA::black());
            snapshot.append_color(
                &color,
                &graphene::Rect::new(0.0, 0.0, widget.width() as f32, widget.height() as f32),
            );
            widget.snapshot_child(&self.label, snapshot);
        }
    }
}

glib::wrapper! {
    pub struct CustomWidget(ObjectSubclass<imp::CustomWidget>) @extends gtk::Widget;
}

impl CustomWidget {
    pub fn new() -> Self {
        glib::Object::new(&[]).unwrap()
    }
}
