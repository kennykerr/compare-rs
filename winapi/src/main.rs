use winapi::{um::handleapi::*, um::synchapi::*, um::winuser::*};

fn main() {
    unsafe {
        let event = CreateEventW(std::ptr::null_mut(), 1, 0, std::ptr::null_mut());
        SetEvent(event);
        WaitForSingleObject(event, 0);
        CloseHandle(event);

        MessageBoxA(
            std::ptr::null_mut(),
            b"Text\0".as_ptr() as _,
            b"Caption\0".as_ptr() as _,
            MB_OK,
        );
    }
}
