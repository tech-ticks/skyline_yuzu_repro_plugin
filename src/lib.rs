use skyline::nn;
use skyline::{hook, install_hook};
use std::os::raw::c_char;

static mut HOOK_INSTALLED: bool = false;

#[hook(replace = nn::fs::OpenFile)]
unsafe fn hook_open_file(handle: *mut nn::fs::FileHandle, path: *const c_char, mode: i32) -> i32 {
    let path_str = std::ffi::CStr::from_ptr(path).to_str().unwrap();
    println!("[repro_plugin] !!! nn::fs::OpenFile called with path {}", path_str);
    call_original!(handle, path, mode)
}

#[hook(offset = 0x182050)]
unsafe fn hook_ground_manager_update() {
    if !HOOK_INSTALLED {
        // This function is called in every frame, but we only want to install the hook once
        HOOK_INSTALLED = true;
        println!("[repro_plugin] Inside GroundManager_Update, installing hook for SDK function 'nn::fs::OpenFile'");
        install_hook!(hook_open_file);
    }
}

#[skyline::main(name = "yuzu_repro")]
pub fn main() {
    println!("[repro_plugin] main() in yuzu repro plugin. Installing hook for game function 0x182050 (GroundManager_Update)..");
    // Internally calls the function A64HookFunction exported by Skyline (subsdk9)
    install_hook!(hook_ground_manager_update);

    // Installing the SDK hook inside the game function hook causes issues - installing the SDK hook directly here would work:
    // install_hook!(hook_open_file);
}
