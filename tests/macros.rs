#![macro_escape]

pub macro_rules! wxApp(
    ($f: ident) => (
        fn main() {
            use libc::c_void;

            use wx::base::Closure;
            use wx::core::RustApp;

            let nullptr: *mut c_void = 0 as *mut c_void;

            let closure = Closure::new($f as *mut c_void, nullptr);
            let args: Vec<*mut i32> = Vec::new();
            RustApp::initializeC(&closure, args.len() as i32, args.as_ptr() as *mut *mut i8);
        }
    )
);
