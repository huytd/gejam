
use libc::{c_int, c_double};
use super::*;

extern "C" {
    fn get_context_2d(cref: c_int)  -> c_int;
    fn ctx_fill_rect(cref: c_int, x: c_int, y: c_int, w: c_int, h: c_int);
    fn ctx_draw_image(cref: c_int, iref: c_int, dx: c_int, dy: c_int);
    fn ctx_draw_image_scaled(cref: c_int, iref: c_int, dx: c_int, dy: c_int, dw: c_int, dh: c_int);
    fn ctx_draw_image_part(cref: c_int, iref: c_int, sx: c_int, sy: c_int, sw: c_int, sh: c_int, dx: c_int, dy: c_int, dw: c_int, dh: c_int);
    fn ctx_fill_text(cref: c_int, text: *const libc::c_char, text_len: libc::c_int, x: c_int, y: c_int);
    fn ctx_save(cref: c_int);
    fn ctx_restore(cref: c_int);
    fn ctx_translate(cref: c_int, x: c_int, y: c_int);
    fn ctx_scale(cref: c_int, x: c_double, y: c_double);
    fn ctx_rotate(cref: c_int, angle: c_double);

    fn ctx_begin_path(cref: c_int);
    fn ctx_clip(cref: c_int);
    fn ctx_fill(cref: c_int);
    fn ctx_arc(cref: c_int, x: c_int, y: c_int, radius: c_int, start: c_double, end: c_double, anti: bool);

    fn ctx_create_radial_gradient(cref: c_int, x0: c_int, y0: c_int, r0: c_int, x1: c_int, y1: c_int, r1: c_int) -> c_int;
    fn gradient_add_color_stop(gref: c_int, offset: c_double, val: *const libc::c_char, val_len: libc::c_int);
}

pub trait Drawable: JSObject {
    fn can_draw(&self) -> bool {
        true
    }
}

pub struct Canvas {
    internal: c_int,
}

unsafe impl JSObject for Canvas {
    unsafe fn get_internal(&self) -> c_int { self.internal }
}

impl JSElement for Canvas {}

impl Drawable for Canvas {}

impl Canvas {
    pub fn new() -> Canvas {
        Canvas {
            internal: make_element_raw("canvas"),
        }

    }
    pub fn get_by_id(id: &str) -> Option<Canvas> {
        let id = get_element_raw(id);
        if id == -1 {
            None
        } else {
            Some(Canvas {
                internal: id
            })
        }
    }

    pub fn set_width(&self, width: i32) {
        set_prop_int(self.internal, "width", width);
    }

    pub fn set_height(&self, height: i32) {
        set_prop_int(self.internal, "height", height);
    }

    pub fn get_width(&self) -> i32 {
        get_prop_int(self.internal, "width")
    }

    pub fn get_height(&self) -> i32 {
        get_prop_int(self.internal, "height")
    }

    pub fn get_context_2d(&self) -> Context2d {
        unsafe {
            Context2d {
                internal: get_context_2d(self.internal),
            }
        }
    }
}

impl Drop for Canvas {
    fn drop(&mut self) {
        unsafe {
            drop_ref(self.internal);
        }
    }
}

pub struct Context2d {
    internal: c_int,
}

unsafe impl JSObject for Context2d {
    unsafe fn get_internal(&self) -> c_int { self.internal }
}

impl Context2d {

    pub fn set_image_smoothing(&self, enabled: bool) {
        set_prop_int(self.internal, "mozImageSmoothingEnabled", enabled as i32);
        set_prop_int(self.internal, "webkitImageSmoothingEnabled", enabled as i32);
        set_prop_int(self.internal, "msImageSmoothingEnabled", enabled as i32);
        set_prop_int(self.internal, "imageSmoothingEnabled", enabled as i32);
    }

    pub fn save(&self) {
        unsafe {
            ctx_save(self.internal);
        }
    }

    pub fn restore(&self) {
        unsafe {
            ctx_restore(self.internal);
        }
    }

    pub fn begin_path(&self) {
        unsafe {
            ctx_begin_path(self.internal);
        }
    }

    pub fn clip(&self) {
        unsafe {
            ctx_clip(self.internal);
        }
    }

    pub fn fill(&self) {
        unsafe {
            ctx_fill(self.internal);
        }
    }

    pub fn arc(&self, x: i32, y: i32, radius: i32, start: f64, end: f64) {
        unsafe {
            ctx_arc(self.internal, x, y, radius, start, end, false);
        }
    }

    pub fn set_global_alpha(&self, alpha: f64) {
        set_prop_float(self.internal, "globalAlpha", alpha);
    }

    pub fn set_font(&self, font: &str) {
        set_prop_str(self.internal, "font", font);
    }

    pub fn set_fill_style<C>(&self, style: &C)
        where C: Color + ?Sized
    {
        unsafe {
            style.set_to(self.internal, "fillStyle");
        }
    }

    pub fn translate(&self, x: i32, y: i32) {
        unsafe {
            ctx_translate(self.internal, x as _, y as _);
        }
    }

    pub fn scale(&self, x: f64, y: f64) {
        unsafe {
            ctx_scale(self.internal, x as _, y as _);
        }
    }

    pub fn rotate(&self, angle: f64) {
        unsafe {
            ctx_rotate(self.internal, angle as _);
        }
    }

    pub fn fill_rect(&self, x: i32, y: i32, w: i32, h: i32) {
        unsafe {
            ctx_fill_rect(self.internal, x as _, y as _, w as _, h as _);
        }
    }

    pub fn draw_image<I: Drawable>(&self, i: &I, dx: i32, dy: i32) {
        unsafe {
            if i.can_draw() {
                ctx_draw_image(self.internal, i.get_internal(), dx as _, dy as _);
            }
        }
    }

    pub fn draw_image_scaled<I: Drawable>(&self, i: &I, dx: i32, dy: i32, dw: i32, dh: i32) {
        unsafe {
            if i.can_draw() {
                ctx_draw_image_scaled(self.internal, i.get_internal(), dx as _, dy as _, dw as _, dh as _);
            }
        }
    }

    pub fn draw_image_part<I: Drawable>(&self, i: &I, sx: i32, sy: i32, sw: i32, sh: i32, dx: i32, dy: i32, dw: i32, dh: i32) {
        unsafe {
            if i.can_draw() {
                ctx_draw_image_part(self.internal, i.get_internal(), sx as _, sy as _, sw as _, sh as _, dx as _, dy as _, dw as _, dh as _);
            }
        }
    }

    pub fn fill_text(&self, text: &str, x: i32, y: i32) {
        unsafe {
            ctx_fill_text(self.internal, text.as_ptr() as *const _, text.as_bytes().len() as _, x, y);
        }
    }

    pub fn create_radial_gradient(&self, x0: i32, y0: i32, r0: i32, x1: i32, y1: i32, r1: i32) -> CanvasGradient {
        CanvasGradient {
            internal: unsafe {
                ctx_create_radial_gradient(self.internal, x0, y0, r0, x1, y1, r1)
            }
        }
    }
}

impl Drop for Context2d {
    fn drop(&mut self) {
        unsafe {
            drop_ref(self.internal);
        }
    }
}

pub struct CanvasGradient {
    internal: c_int,
}

unsafe impl JSObject for CanvasGradient {
    unsafe fn get_internal(&self) -> c_int { self.internal }
}

impl CanvasGradient {
    pub fn add_color_stop(&self, offset: f64, style: &str) {
        unsafe {
            gradient_add_color_stop(
                self.internal, offset,
                style.as_ptr() as *const _, style.as_bytes().len() as _,
            );
        }
    }
}

impl Drop for CanvasGradient {
    fn drop(&mut self) {
        unsafe {
            drop_ref(self.internal);
        }
    }
}


pub trait Color {
    unsafe fn set_to(&self, cref: c_int, key: &str);
}

impl Color for str {
    unsafe fn set_to(&self, cref: c_int, key: &str) {
        set_prop_str(cref, key, self)
    }
}

impl Color for String {
    unsafe fn set_to(&self, cref: c_int, key: &str) {
        set_prop_str(cref, key, self)
    }
}

impl Color for CanvasGradient {
    unsafe fn set_to(&self, cref: c_int, key: &str) {
        set_prop_ref(cref, key, self.internal)
    }
}