cfg_if! {
    if #[cfg(target_os = "linux")] {
        mod linux;

        pub type Manager = linux::SysFsManager;
        pub type Iterator = linux::SysFsIterator;
        pub type Device = linux::SysFsDevice;
    } else if #[cfg(any(target_os = "macos", target_os = "ios"))] {
        mod darwin;

        pub type Manager = darwin::IoKitManager;
        pub type Iterator = darwin::IoKitIterator;
        pub type Device = darwin::IoKitDevice;
    } else if #[cfg(target_os = "windows")] {
        mod windows;

        pub type Manager = windows::PowerManager;
        pub type Iterator = windows::PowerIterator;
        pub type Device = windows::PowerDevice;
    } else if #[cfg(any(target_os = "dragonfly", target_os = "freebsd"))] {
        mod freebsd;

        pub type Manager = freebsd::IoCtlManager;
        pub type Iterator = freebsd::IoCtlIterator;
        pub type Device = freebsd::IoCtlDevice;
     } else if #[cfg(target_os = "netbsd")] {
        mod netbsd;

        pub type Manager = netbsd::SysMonManager;
        pub type Iterator = netbsd::SysMonIterator;
        pub type Device = netbsd::SysMonDevice;
    } else {
        mod fallback;

        pub type Manager = fallback::FallbackManager;
        pub type Iterator = fallback::FallbackIterator;
        pub type Device = fallback::FallbackDevice;
    }
}

pub mod traits;
