use crate::constants::{APP_ID, LOG_TAG};
pub use rutin_tizen::app::UIApp;
pub use rutin_tizen::efl::elm::prelude::*;
use rutin_tizen::efl::elm::{Conformant, IndicatorMode, IndicatorOpacityMode, Label, Win};
use rutin_tizen::efl::evas::SizeHint;
use rutin_tizen::system::dlog;
use rutin_tizen::system::dlog::Priority;

pub struct App<'a> {
    _owned: Vec<Box<dyn Object<'a> + 'a>>,
}

impl<'a> App<'a> {
    pub fn new() -> App<'a> {
        App { _owned: vec![] }
    }
}

impl<'a> UIApp for App<'a> {
    fn create(&mut self) -> bool {
        dlog::print(Priority::Error, LOG_TAG, "App Started!");

        let win = Win::new(APP_ID, APP_ID);

        if win.is_none() {
            return false;
        }

        let mut win = win.unwrap();

        dlog::print(Priority::Error, LOG_TAG, "Win Started!");

        win.set_autodel(true);
        win.set_indicator_mode(IndicatorMode::Show);
        win.set_indicator_opacity(IndicatorOpacityMode::Opaque);
        win.set_size_hint_weight(SizeHint::Expand, SizeHint::Expand);

        dlog::print(Priority::Error, LOG_TAG, "Win Configured!");

        let conformant = Conformant::new(&mut win);

        if conformant.is_none() {
            return false;
        }

        let mut conformant = conformant.unwrap();

        dlog::print(Priority::Error, LOG_TAG, "Conformant Created!");

        win.add_resize_object(&mut conformant);
        conformant.show();

        let label = Label::new(&mut conformant);

        if label.is_none() {
            return false;
        }

        let mut label = label.unwrap();

        dlog::print(Priority::Error, LOG_TAG, "Label Created!");

        label.set_text("<align=center>Hello World</align>");
        label.set_size_hint_weight(SizeHint::Expand, SizeHint::Expand);

        conformant.set_content(&mut label);

        win.show();

        dlog::print(Priority::Error, LOG_TAG, "Win showed!");

        self._owned.push(Box::new(win));
        self._owned.push(Box::new(conformant));
        self._owned.push(Box::new(label));

        true
    }

    fn terminate(&mut self) {}
    fn pause(&mut self) {}
    fn resume(&mut self) {}
}
