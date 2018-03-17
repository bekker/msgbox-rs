use icon::IconType;

pub fn create(title:&str, content:&str, icon_type:IconType) {
    use std::ffi::CString;
    use std::ptr::null_mut;
    use ::user32::MessageBoxA;
    use ::winapi::winuser::{MB_OK, MB_ICONINFORMATION, MB_ICONERROR, MB_SYSTEMMODAL};

    let lp_text = CString::new(content).unwrap();
    let lp_caption = CString::new(title).unwrap();

    let window_type = match icon_type {
        IconType::ERROR => MB_OK | MB_ICONERROR | MB_SYSTEMMODAL,
        IconType::INFO => MB_OK | MB_ICONINFORMATION | MB_SYSTEMMODAL,
        IconType::NONE => MB_OK | MB_SYSTEMMODAL,
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
