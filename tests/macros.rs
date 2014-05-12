#[macro_escape];

pub macro_rules! wxApp(
    ($f: ident) => (
        fn start(argc: int, argv: **u8) -> int {
            #[start];

            use libc::c_void;

            use wx::base::Closure;
            use wx::core::RustApp;

            static nullptr: *mut c_void = 0 as *mut c_void;

            do native::start(argc, argv) {
                let closure = Closure::new($f as *mut c_void, nullptr);
                let args: ~[*i32] = ~[];
                RustApp::initializeC(&closure, args.len() as i32, args.as_ptr() as *mut *mut i8);
            }
        }
    )
)
