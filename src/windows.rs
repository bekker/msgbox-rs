use icon::IconType;

pub fn create(title:&str, content:&str, icon_type:IconType) {
    use std::iter::once;
    use std::ptr::null_mut;
    use ::user32::MessageBoxW;
    use ::winapi::winuser::{MB_OK, MB_ICONINFORMATION, MB_ICONERROR, MB_SYSTEMMODAL};

    let lp_text: Vec<u16> = content.encode_utf16().chain(once(0)).collect();
    let lp_caption: Vec<u16> = title.encode_utf16().chain(once(0)).collect();

    let window_type = match icon_type {
        IconType::Error => MB_OK | MB_ICONERROR | MB_SYSTEMMODAL,
        IconType::Info => MB_OK | MB_ICONINFORMATION | MB_SYSTEMMODAL,
        IconType::None => MB_OK | MB_SYSTEMMODAL,
    };

    unsafe {
        MessageBoxW(
            null_mut(),
            lp_text.as_ptr(),
            lp_caption.as_ptr(),
            window_type
        );
    }
}
