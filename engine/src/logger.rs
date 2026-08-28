use std::fmt::Debug;
use std::panic;

pub fn init() {
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

        eprintln!("[PANIC] {}:{} -> {}", file, line, msg);
    }));
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

                eprintln!("[FATAL] {}:{} -> {}: {:?}", file, line, msg, e);
                
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

                eprintln!("[FATAL] {}:{} -> null: {}", file, line, msg);
                
                panic!("{}", msg);
            }
        }
    }
}