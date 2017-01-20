extern crate user32;
extern crate winapi;

pub fn create(title:&str, content:&str, btn_text:&str) {
    use std::ffi::CString;
    use std::ptr::null_mut;
    use self::user32::MessageBoxA;
    use self::winapi::winuser::{MB_OK, MB_ICONINFORMATION};

    fn main() {
        let lp_text = CString::new("text").unwrap();
        let lp_caption = CString::new("title").unwrap();

        unsafe {
            MessageBoxA(
                null_mut(),
                lp_text.as_ptr(),
                lp_caption.as_ptr(),
                MB_OK | MB_ICONINFORMATION
            );
        }
    }
}
