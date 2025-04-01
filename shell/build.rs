use cfg_aliases::cfg_aliases;

fn main() {
    // Setup cfg aliases
    cfg_aliases! {
        // Systems.
        macos_platform: { target_os = "macos" },
        ios_platform: { target_os = "ios" },
        windows_platform: { target_os = "windows" },
        apple: { any(target_os = "ios", target_os = "macos") },
        free_unix: { all(unix, not(apple)) },
        redox: { target_os = "redox" },

        // Native displays.
        x11_platform: { all(feature = "x11", free_unix, not(redox)) },
        wayland_platform: { all(feature = "wayland", free_unix, not(redox)) },
        orbital_platform: { redox },
    }
}
