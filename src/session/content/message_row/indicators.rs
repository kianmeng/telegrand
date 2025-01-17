use super::indicators_model::MessageIndicatorsModel;

use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate};

mod imp {
    use super::*;
    use glib::clone;
    use once_cell::sync::Lazy;

    #[derive(Debug, Default, CompositeTemplate)]
    #[template(string = r#"
    <interface>
      <template class="MessageIndicators" parent="GtkWidget">
        <property name="layout-manager">
          <object class="GtkBoxLayout">
            <property name="spacing">3</property>
          </object>
        </property>
        <child>
          <object class="GtkLabel">
            <binding name="label">
              <lookup name="message-info">
                <lookup name="model">MessageIndicators</lookup>
              </lookup>
            </binding>
          </object>
        </child>
        <child>
          <object class="GtkImage" id="sending_state_icon">
            <binding name="icon-name">
              <lookup name="sending-state-icon-name">
                <lookup name="model">MessageIndicators</lookup>
              </lookup>
            </binding>
          </object>
        </child>
      </template>
    </interface>
    "#)]
    pub(crate) struct MessageIndicators {
        pub(super) model: MessageIndicatorsModel,
        #[template_child]
        pub(super) sending_state_icon: TemplateChild<gtk::Image>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for MessageIndicators {
        const NAME: &'static str = "MessageIndicators";
        type Type = super::MessageIndicators;
        type ParentType = gtk::Widget;

        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);
            klass.set_css_name("messageindicators");
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for MessageIndicators {
        fn properties() -> &'static [glib::ParamSpec] {
            static PROPERTIES: Lazy<Vec<glib::ParamSpec>> = Lazy::new(|| {
                vec![
                    glib::ParamSpecObject::new(
                        "message",
                        "Message",
                        "The message of the widget",
                        glib::Object::static_type(),
                        glib::ParamFlags::READWRITE | glib::ParamFlags::EXPLICIT_NOTIFY,
                    ),
                    glib::ParamSpecObject::new(
                        "model",
                        "Model",
                        "The model of the widget",
                        MessageIndicatorsModel::static_type(),
                        glib::ParamFlags::READABLE,
                    ),
                ]
            });
            PROPERTIES.as_ref()
        }

        fn set_property(
            &self,
            obj: &Self::Type,
            _id: usize,
            value: &glib::Value,
            pspec: &glib::ParamSpec,
        ) {
            match pspec.name() {
                "message" => obj.set_message(value.get().unwrap()),
                _ => unimplemented!(),
            }
        }

        fn property(&self, obj: &Self::Type, _id: usize, pspec: &glib::ParamSpec) -> glib::Value {
            match pspec.name() {
                "message" => obj.message().to_value(),
                "model" => obj.model().to_value(),
                _ => unimplemented!(),
            }
        }

        fn constructed(&self, obj: &Self::Type) {
            self.parent_constructed(obj);

            self.model.connect_notify_local(
                Some("message"),
                clone!(@weak obj => move |_, _| {
                    obj.notify("message");
                }),
            );

            self.sending_state_icon
                .connect_icon_name_notify(|sending_state_icon| {
                    sending_state_icon.set_visible(
                        sending_state_icon
                            .icon_name()
                            .map(|icon_name| !icon_name.is_empty())
                            .unwrap_or(false),
                    )
                });
        }

        fn dispose(&self, obj: &Self::Type) {
            let mut child = obj.first_child();
            while let Some(child_) = child {
                child = child_.next_sibling();
                child_.unparent();
            }
        }
    }

    impl WidgetImpl for MessageIndicators {}
}

glib::wrapper! {
    pub(crate) struct MessageIndicators(ObjectSubclass<imp::MessageIndicators>)
        @extends gtk::Widget;
}

impl MessageIndicators {
    pub(crate) fn message(&self) -> glib::Object {
        self.imp().model.message()
    }

    pub(crate) fn set_message(&self, message: glib::Object) {
        self.imp().model.set_message(message);
    }

    pub(crate) fn model(&self) -> &MessageIndicatorsModel {
        &self.imp().model
    }
}
