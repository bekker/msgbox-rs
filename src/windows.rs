extern crate user32;
extern crate winapi;

use icon::IconType;

pub fn create(title:&str, content:&str, icon_type:IconType) {
    use std::ffi::CString;
    use std::ptr::null_mut;
    use self::user32::MessageBoxA;
    use self::winapi::winuser::{MB_OK, MB_ICONINFORMATION, MB_ICONERROR};

    let lp_text = CString::new(content).unwrap();
    let lp_caption = CString::new(title).unwrap();

    let window_type = match icon_type {
        IconType::ERROR => MB_OK | MB_ICONERROR,
        IconType::INFO => MB_OK | MB_ICONINFORMATION,
        IconType::NONE => MB_OK,
    };

    unsafe {
        MessageBoxA(
            null_mut(),
            lp_text.as_ptr(),
            lp_caption.as_ptr(),
            window_type
        );
    }
}
