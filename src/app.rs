use crate::constants::{APP_ID, LOG_TAG};
pub use rutin_tizen::app::UIApp;
use rutin_tizen::efl::elm::Conformant;
use rutin_tizen::efl::elm::{IndicatorMode, IndicatorOpacityMode, Win};
pub use rutin_tizen::efl::evas::Object;
use rutin_tizen::efl::evas::SizeHint;
use rutin_tizen::system::{dlog, dlog::Priority};

pub struct App<'a> {
    _owned: Vec<Box<dyn Object<'a>>>,
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
        win.set_autodel(true);
        win.set_indicator_mode(IndicatorMode::Show);
        win.set_indicator_opacity(IndicatorOpacityMode::Opaque);
        win.set_size_hint_weight(SizeHint::Expand, SizeHint::Expand);

        let conformant = Conformant::new(&mut win);

        if conformant.is_none() {
            return false;
        }

        let mut conformant = conformant.unwrap();

        self._owned.push(Box::new(win));

        true
    }

    fn terminate(&mut self) {}
    fn pause(&mut self) {}
    fn resume(&mut self) {}
}
