use std::ffi::{OsStr, OsString};
use std::os::unix::ffi::OsStringExt;
use std::path::{Path, PathBuf};

pub fn getcwd() -> PathBuf {
    let mut buf = [0u8; 4096];

    let res = unsafe {
        c::getcwd(&mut buf as *mut u8 as _, buf.len() as _)
    };

    if res.is_null() {
        let msg = &Errnum::errno().strerror();
        let msg = msg.to_string_lossy();
        panic!("getcwd failed: {}", msg);
    }

    os_string_of_bytes_with_nil(&buf).into()
}

#[derive(Clone, Copy)]
pub struct Errnum(c::c_int);

impl Errnum {
    pub fn errno() -> Self {
        Errnum(unsafe { c::errno })
    }

    pub fn reset_errno() {
        unsafe { c::errno = 0 };
    }

    pub fn strerror(self) -> OsString {
        let mut buf = [0u8; 4096];
        let buf_ptr = &mut buf as *mut u8 as *mut c::c_char;
        let buf_len = buf.len() as c::c_ulong;

        unsafe {
            let res = c::strerror_r(self.0, buf_ptr, buf_len);
            assert_eq!(res, 0);
            os_string_of_bytes_with_nil(&buf)
        }
    }
}

fn os_string_of_bytes_with_nil(buf: &[u8]) -> OsString {
    let ix_nil = buf.iter().position(|&b| b == 0)
        .expect("NUL terminator");
    OsString::from_vec(buf[.. ix_nil].to_owned())
}

mod c {
    pub use std::os::raw::{c_char, c_int, c_ulong};
    extern "C" {
        pub fn getcwd(buf: *mut c_char, size: c_ulong) -> *mut c_char;

        pub fn strerror_r(errnum: c_int, buf: *mut c_char, buflen: c_ulong) -> c_int;
        pub static mut errno: c_int;
    }
}
