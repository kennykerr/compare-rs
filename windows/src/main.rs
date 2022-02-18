use windows::{
    core::*, Win32::Foundation::*, Win32::System::Threading::*, Win32::UI::WindowsAndMessaging::*,
};

fn main() -> Result<()> {
    unsafe {
        let event = CreateEventW(std::ptr::null(), true, false, None);
        SetEvent(event).ok()?;
        WaitForSingleObject(event, 0);
        CloseHandle(event).ok()?;
        MessageBoxA(None, "Text", "Caption", MB_OK);
        Ok(())
    }
}
