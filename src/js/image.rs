
use libc;
use super::*;
use super::{
    get_prop_int,
    set_prop_int,
    set_prop_str,
    make_element_raw,
    get_element_raw,
    drop_ref,
    clone_ref,
};

pub struct Image {
    internal: libc::c_int,
}

impl Clone for Image {
    fn clone(&self) -> Image {
        Image {
            internal: unsafe { clone_ref(self.internal) },
        }
    }
}

unsafe impl JSObject for Image {
    unsafe fn get_internal(&self) -> libc::c_int { self.internal }
}

impl JSElement for Image {}

impl Drawable for Image {
    fn can_draw(&self) -> bool {
        self.is_complete()
    }
}

impl Image {
    pub fn new() -> Image {
        Image {
            internal: make_element_raw("img"),
        }

    }
    pub fn get_by_id(id: &str) -> Option<Image> {
        let id = get_element_raw(id);
        if id == -1 {
            None
        } else {
            Some(Image {
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

    pub fn set_source(&self, src: &str) {
        set_prop_str(self.internal, "src", src);
    }

    pub fn is_complete(&self) -> bool {
        get_prop_int(self.internal, "complete") != 0
    }
}

impl Drop for Image {
    fn drop(&mut self) {
        unsafe {
            drop_ref(self.internal);
        }
    }
}