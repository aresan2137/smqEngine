use std::fmt::Debug;
use std::panic;

pub fn init() {
    #[cfg(target_arch = "wasm32")]
    {
        panic::set_hook(Box::new(console_error_panic_hook::hook));

        let mut target_level = log::Level::Info;

        if let Some(window) = web_sys::window() {
            if let Ok(search) = window.location().search() {
                let search_lower = search.to_lowercase();
                if search_lower.contains("log=trace") {
                    target_level = log::Level::Trace;
                } else if search_lower.contains("log=debug") {
                    target_level = log::Level::Debug;
                } else if search_lower.contains("log=warn") {
                    target_level = log::Level::Warn;
                } else if search_lower.contains("log=error") {
                    target_level = log::Level::Error;
                } else if search_lower.contains("log=info") {
                    target_level = log::Level::Info;
                }
            }
        }

        console_log::init_with_level(target_level).expect("failed to init logger");
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        panic::set_hook(Box::new(|panic_info| {
            let (file, line) = if let Some(location) = panic_info.location() {
                (location.file(), location.line())
            } else {
                ("unknown file", 0)
            };

            let msg = if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
                *s
            } else if let Some(s) = panic_info.payload().downcast_ref::<String>() {
                s.as_str()
            } else {
                "unknown critical"
            };

            log::error!("[PANIC] {}:{} -> {}", file, line, msg);
        }));

        env_logger::init();
    }
}

pub trait LogExpect<T> {
    fn log_expect(self, msg: &str) -> T;
}

impl<T, E: Debug> LogExpect<T> for Result<T, E> {
    #[track_caller]
    fn log_expect(self, msg: &str) -> T {
        match self {
            Ok(val) => val,
            Err(e) => {
                let location = std::panic::Location::caller();
                let file = location.file();
                let line = location.line();

                log::error!("[FATAL] {}:{} -> {}: {:?}", file, line, msg, e);

                panic!("{}: {:?}", msg, e);
            }
        }
    }
}

impl<T> LogExpect<T> for Option<T> {
    #[track_caller]
    fn log_expect(self, msg: &str) -> T {
        match self {
            Some(val) => val,
            None => {
                let location = std::panic::Location::caller();
                let file = location.file();
                let line = location.line();

                log::error!("[FATAL] {}:{} -> null: {}", file, line, msg);

                panic!("{}", msg);
            }
        }
    }
}