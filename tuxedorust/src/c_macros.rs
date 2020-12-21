#[allow(unused_macros)]
macro_rules! point_point {
    ($x:expr) => ( Box::into_raw(Box::new($x)))
}
macro_rules! c_str {
    ($x:expr) => (CString::new($x).unwrap().as_ptr() as *mut i8;)
}
macro_rules! c_char_to_rust {
    ($x:expr) => ( CStr::from_ptr(($x).as_ptr()).to_str().unwrap() ; )
}

const __NULL__: i8 = 0;
#[allow(unused_macros)]
macro_rules! C_NULL {
    () => (__NULL__ as *mut i8)
}
