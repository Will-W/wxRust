#[macro_escape];

pub macro_rules! wxApp(
    ($f: ident) => (
        fn start(argc: int, argv: **u8) -> int {
            #[start];

            use std::libc::c_void;

            use wx::base::WxClosure;
            use wx::core::WxrApp;

            static nullptr: *mut c_void = 0 as *mut c_void;

            do native::start(argc, argv) {
                let closure = WxClosure::new($f as *mut c_void, nullptr);
                let args: ~[*i32] = ~[];
                WxrApp::initializeC(&closure, args.len() as i32, args.as_ptr() as *mut *mut i8);
            }
        }
    )
)
