use std::env::args_os;
use std::ffi::CStr;
use std::marker::Sized;
use std::mem::uninitialized;
use std::ops::{Deref, DerefMut};
use std::os::raw::{c_char, c_int, c_void};
use std::os::unix::ffi::OsStrExt;
use std::process::exit;
use std::ptr::null_mut;
use tizen_sys::{
    app_control_h, dlog_print, log_priority_DLOG_ERROR, ui_app_lifecycle_callback_s, ui_app_main,
    Eo, Evas_Coord,
};

fn main() {
    unsafe {
        dlog_print(
            log_priority_DLOG_ERROR,
            CStr::from_bytes_with_nul_unchecked(TAG.as_bytes()).as_ptr(),
            CStr::from_bytes_with_nul_unchecked("app started\0".as_bytes()).as_ptr(),
        );
    }

    let mut app = HueApp {
        mainWin: None,
        nodes: vec![],
    };
    exit(app.main())
}

const PACKAGE: &str = "com.andersondanilo.tizen-example\0";
const TAG: &str = "ANDERSON\0";

struct HueApp {
    mainWin: Option<ElmWin>,
    nodes: Vec<*mut Eo>,
}

impl UIApp for HueApp {
    fn create(&mut self) -> bool {
        let package = unsafe { CStr::from_bytes_with_nul_unchecked(PACKAGE.as_bytes()) };
        let mut win = match ElmWin::standard_add(package, package) {
            Some(win) => {
                unsafe {
                    dlog_print(
                        log_priority_DLOG_ERROR,
                        CStr::from_bytes_with_nul_unchecked(TAG.as_bytes()).as_ptr(),
                        CStr::from_bytes_with_nul_unchecked("window created!\0".as_bytes())
                            .as_ptr(),
                    );
                }

                win
            }
            None => {
                unsafe {
                    dlog_print(
                        log_priority_DLOG_ERROR,
                        CStr::from_bytes_with_nul_unchecked(TAG.as_bytes()).as_ptr(),
                        CStr::from_bytes_with_nul_unchecked("cannot create window\0".as_bytes())
                            .as_ptr(),
                    );
                }

                return false;
            }
        };
        win.autodel_set(true);
        if win.wm_rotation_supported_get() {
            // TODO
        }

        unsafe {
            let conform = tizen_sys::elm_conformant_add(win.eo());
            tizen_sys::elm_win_indicator_mode_set(
                win.eo(),
                tizen_sys::Elm_Win_Indicator_Mode_ELM_WIN_INDICATOR_SHOW,
            );
            tizen_sys::elm_win_indicator_opacity_set(
                win.eo(),
                tizen_sys::Elm_Win_Indicator_Opacity_Mode_ELM_WIN_INDICATOR_OPAQUE,
            );
            tizen_sys::evas_object_size_hint_weight_set(
                win.eo(),
                tizen_sys::EVAS_HINT_EXPAND,
                tizen_sys::EVAS_HINT_EXPAND,
            );
            tizen_sys::elm_win_resize_object_add(win.eo(), conform);
            tizen_sys::evas_object_show(conform);

            /* Label */
            /* Create an actual view of the base gui.
            Modify this part to change the view. */
            let label = tizen_sys::elm_label_add(conform);
            tizen_sys::elm_object_part_text_set(
                label,
                std::ptr::null(),
                CStr::from_bytes_with_nul_unchecked("<align=center>ANDERSON</align>\0".as_bytes())
                    .as_ptr(),
            );
            tizen_sys::evas_object_size_hint_weight_set(
                label,
                tizen_sys::EVAS_HINT_EXPAND,
                tizen_sys::EVAS_HINT_EXPAND,
            );
            tizen_sys::elm_object_part_content_set(conform, std::ptr::null(), label);

            self.nodes.push(conform);
            self.nodes.push(label);
        }

        win.show();

        self.mainWin = Some(win);
        true
    }
    fn terminate(&mut self) {}
    fn pause(&mut self) {}
    fn resume(&mut self) {}
    fn control(&mut self, _app_control: app_control_h) {}
}

pub trait UIApp: Sized {
    fn create(&mut self) -> bool;
    fn terminate(&mut self);
    fn pause(&mut self);
    fn resume(&mut self);
    fn control(&mut self, app_control: app_control_h);

    fn main(&mut self) -> c_int {
        let args = args_os().collect::<Vec<_>>();
        let mut argv: Vec<*mut i8> = Vec::new();
        for mut i in args {
            i.push("\0");
            argv.push(i.as_bytes().as_ptr() as *mut c_char);
        }
        argv.push(null_mut());

        let mut event_callback = ui_app_lifecycle_callback_s {
            create: Some(app_create::<Self>),
            terminate: Some(app_terminate::<Self>),
            pause: Some(app_pause::<Self>),
            resume: Some(app_resume::<Self>),
            app_control: Some(app_control::<Self>),
        };

        unsafe {
            ui_app_main(
                argv.len() as c_int,
                argv.as_mut_slice().as_mut_ptr(),
                &mut event_callback,
                self as *mut _ as *mut c_void,
            )
        }
    }
}

extern "C" fn app_create<T: UIApp>(data: *mut c_void) -> bool {
    let app = unsafe { &mut *(data as *mut T) };
    app.create()
}

extern "C" fn app_terminate<T: UIApp>(data: *mut c_void) {
    let app = unsafe { &mut *(data as *mut T) };
    app.terminate()
}

extern "C" fn app_pause<T: UIApp>(data: *mut c_void) {
    let app = unsafe { &mut *(data as *mut T) };
    app.pause()
}

extern "C" fn app_resume<T: UIApp>(data: *mut c_void) {
    let app = unsafe { &mut *(data as *mut T) };
    app.resume()
}

extern "C" fn app_control<T: UIApp>(app_control: app_control_h, data: *mut c_void) {
    let app = unsafe { &mut *(data as *mut T) };
    app.control(app_control)
}

pub struct EvasObject {
    obj: *mut tizen_sys::Eo,
    //smart_callbacks: HashMap<'static CStr, Box<Fn<(&mut EvasObject,
    //                                               *mut c_void)>>>;
}

impl EvasObject {
    fn eo(&mut self) -> *mut tizen_sys::Eo {
        self.obj
    }

    fn geometry_get(&mut self) -> (Evas_Coord, Evas_Coord, Evas_Coord, Evas_Coord) {
        unsafe {
            let mut x = uninitialized();
            let mut y = uninitialized();
            let mut w = uninitialized();
            let mut h = uninitialized();
            tizen_sys::evas_object_geometry_get(self.eo(), &mut x, &mut y, &mut w, &mut h);
            (x, y, w, h)
        }
    }

    pub fn geometry_set(&mut self, x: Evas_Coord, y: Evas_Coord, w: Evas_Coord, h: Evas_Coord) {
        unsafe { tizen_sys::evas_object_geometry_set(self.eo(), x, y, w, h) }
    }

    pub fn show(&mut self) {
        unsafe { tizen_sys::evas_object_show(self.eo()) }
    }

    pub fn hide(&mut self) {
        unsafe { tizen_sys::evas_object_hide(self.eo()) }
    }

    pub fn size_hint_weight_set(&mut self, x: f64, y: f64) {
        unsafe { tizen_sys::evas_object_size_hint_weight_set(self.eo(), x, y) }
    }

    /*
    pub fn smart_callback_add(&mut self,
                              event: &'static CStr,
                                     func: fn(&mut EvasObject, &'a T),
                                     data: &'a T) {
        unsafe { evas_object_smart_callback_add(self.eo(), event.as_ptr(),
                                                smart_callback_wrapper
    }*/

    /*
    pub fn evas_object_smart_callback_add(obj: *mut Evas_Object,
                                          event:
                                              *const ::std::os::raw::c_char,
                                          func: Evas_Smart_Cb,
                                          data:
                                             *const ::std::os::raw::c_void);
    */
}

/*
extern fn smart_callback_wrapper(data: c_void, obj: *mut tizen_sys::Eo, event_info: *mut c_void) {
}
    ::std::option::Option<unsafe extern "C" fn(data:
                                                   *mut ::std::os::raw::c_void,
                                               obj: *mut Evas_Object,
                                               event_info:
                                                   *mut ::std::os::raw::c_void)>;
*/

pub struct ElmWin(EvasObject);

impl Deref for ElmWin {
    type Target = EvasObject;

    fn deref(&self) -> &EvasObject {
        &self.0
    }
}

impl DerefMut for ElmWin {
    fn deref_mut(&mut self) -> &mut EvasObject {
        &mut self.0
    }
}

impl ElmWin {
    pub fn standard_add(name: &'static CStr, title: &'static CStr) -> Option<Self> {
        let win = unsafe { tizen_sys::elm_win_util_standard_add(name.as_ptr(), title.as_ptr()) };
        if win.is_null() {
            None
        } else {
            Some(ElmWin(EvasObject { obj: win }))
        }
    }

    pub fn autodel_set(&mut self, autodel: bool) {
        unsafe {
            tizen_sys::elm_win_autodel_set(self.eo(), if autodel { 1 } else { 0 });
        }
    }

    pub fn autodel_get(&mut self) -> bool {
        unsafe { tizen_sys::elm_win_autodel_get(self.eo()) != 0 }
    }

    pub fn lower(&mut self) {
        unsafe { tizen_sys::elm_win_lower(self.eo()) }
    }

    pub fn wm_rotation_supported_get(&self) -> bool {
        unsafe { tizen_sys::elm_win_wm_rotation_supported_get(self.obj) != 0 }
    }
}
