Here's a simple comparison of the [windows](https://github.com/microsoft/windows-rs), [windows-sys](https://github.com/microsoft/windows-rs), and [winapi](https://github.com/retep998/winapi-rs) crates, illustrating code features/differences and build time.

### `windows` crate

```rust
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
```

```
D:\git\compare-rs\windows>cargo build
   Compiling windows_x86_64_msvc v0.32.0 (D:\git\windows-rs\crates\targets\x86_64_msvc)
   Compiling windows v0.32.0 (D:\git\windows-rs\crates\libs\windows)
   Compiling windows v0.1.0 (D:\git\compare-rs\windows)
    Finished dev [unoptimized + debuginfo] target(s) in 7.31s
```

### `windows-sys` crate

```rust
fn main() {
    unsafe {
        let event = CreateEventW(std::ptr::null(), 1, 0, std::ptr::null());
        SetEvent(event);
        WaitForSingleObject(event, 0);
        CloseHandle(event);
        MessageBoxA(0, b"Text\0".as_ptr(), b"Caption\0".as_ptr(), MB_OK);
    }
}
```

```
D:\git\compare-rs\windows-sys>cargo build
   Compiling windows_x86_64_msvc v0.32.0 (D:\git\windows-rs\crates\targets\x86_64_msvc)
   Compiling windows-sys v0.32.0 (D:\git\windows-rs\crates\libs\sys)
   Compiling windows-sys v0.1.0 (D:\git\compare-rs\windows-sys)
    Finished dev [unoptimized + debuginfo] target(s) in 3.22s
```

### `winapi` crate

```rust
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
```

```
D:\git\compare-rs\winapi>cargo build
   Compiling winapi v0.3.9 (D:\git\winapi-rs)
   Compiling winapi v0.1.0 (D:\git\compare-rs\winapi)
    Finished dev [unoptimized + debuginfo] target(s) in 4.20s
```
