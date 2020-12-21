extern crate libc;

mod test_lib;

use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::fs;

include!("./bindings.rs");
include!("c_macros.rs");

#[no_mangle]
pub unsafe extern "C" fn rust_upper(rqst: &TPSVCINFO) {
    let fml_id_message = Fldid32(c_str!("MESSAGE"));
    let mut fml_c_char: [i8; 1024] = [0 as c_char; 1024];
    let mut fml_max_len: u32 = fml_c_char.len() as u32;
    let data: *mut FBFR32 = rqst.data as *mut FBFR32;

    if -1 == Fget32(data, fml_id_message, 0, fml_c_char.as_mut_ptr(), &mut fml_max_len) {
        userlog(c_str!(format!("Fget32 status {}",*_tmget_tperrno_addr())));
        tpreturn(TPFAIL as i32, 0, rqst.data, 0, 0);
    }

    println!("FML to stdout");
    Fprint32(data);
    println!("FML to stdout");

    userlog(c_str!(format!("input     {}", c_char_to_rust!(fml_c_char)     )));

    for i in 0..fml_max_len as usize {
        let mut c: char = char::from(fml_c_char[i] as u8);
        c.make_ascii_uppercase();
        fml_c_char[i] = c as i8;
    }

    userlog(c_str!(format!("uppercase {}", c_char_to_rust!(fml_c_char)     )));

    let fml_buffer_from_file = xmltofml();
    if *fml_buffer_from_file == 0 {
        tpreturn(TPFAIL as i32, 0, rqst.data, 0, 0);
    }

    Fadd32(fml_buffer_from_file as *mut Fbfr32, fml_id_message, fml_c_char.as_mut_ptr() as *mut i8, fml_max_len as u32);

    tpreturn(TPSUCCESS as i32, 0, fml_buffer_from_file, 0, 0);
}

pub fn xmltofml() -> *mut i8 {
    let filename = "fml.xml";

    unsafe {
        userlog(c_str!(format!("filename [{}]",filename)));
    }

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    unsafe {
        let tperrno = _tmget_tperrno_addr();

        let xml_buffer = tpalloc(c_str!("XML"), C_NULL!(), 10000 as i64);
        if tperrno.read() != 0 {
            userlog(c_str!(format!("tpalloc32 XML status {}",*tperrno)));
            return &mut 0i8;
        }

        xml_buffer.copy_from(contents.as_ptr() as *const i8, contents.len());

        let fml_buffer = tpalloc(c_str!("FML32"), C_NULL!(), 10000 as i64);
        if tperrno.read() != 0 {
            userlog(c_str!(format!("tpalloc32 FML32 status {}",*tperrno)));
            return &mut 0i8;
        }

        let return_status = tpxmltofml32(
            xml_buffer,
            C_NULL!(),
            point_point!(fml_buffer) as *mut *mut Fbfr32,
            C_NULL!() as *mut *mut i8, 0);

        userlog(c_str!(format!("FML TO XML status {}",return_status)));
        if return_status != 0 {
            userlog(c_str!(format!("FML TO XML tperrno {}",*tperrno)));
        }

        return fml_buffer;
    }
}

