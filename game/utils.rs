use std::vec;
use std::str;

/// Converts a zero terminated character buffer into an owned
/// string.  This is useful for a few OpenGL functionality where
/// a buffer is allocated to hold strings coming from OpenGL
/// functions (usually error messages).
pub unsafe fn charbuf_to_str(buf: &[u8]) -> ~str {
    str::raw::from_c_str(vec::raw::to_ptr(buf) as *i8)
}
