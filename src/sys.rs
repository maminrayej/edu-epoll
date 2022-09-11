macro_rules! syscall {
    ($fn: ident ( $($arg: expr),* )) => {{
        #[allow(unused_unsafe)]
        let result = unsafe { libc::$fn($($arg, )*) };

        if result == -1 {
            Err(std::io::Error::last_os_error())
        } else {
            Ok(result)
        }
    }};
}

pub(crate) use syscall;
