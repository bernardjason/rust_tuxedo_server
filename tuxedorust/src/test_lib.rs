#[cfg(test)]
mod tests {
    use crate::{ xmltofml, Fbfr32,  Fldid32,  Fget32, _tmget_tperrno_addr };
    use std::os::raw::c_char;
    use std::ffi::{CStr, CString};

    macro_rules! c_str {
    ($x:expr) => (CString::new($x).unwrap().as_ptr() as *mut i8;)
    }
    macro_rules! c_char_to_rust {
    ($x:expr) => ( CStr::from_ptr(($x).as_ptr()).to_str().unwrap() ; )
    }

    #[test]
    fn xml_works() {
        unsafe {
            let back = xmltofml();
            let data = back as *mut Fbfr32;

            let fml_id_message = Fldid32(c_str!("MESSAGE"));
            let mut fml_c_char: [i8; 256] = [0 as c_char; 256];
            let mut fml_max_len: u32 = fml_c_char.len() as u32;

            if -1 == Fget32(data, fml_id_message, 0, fml_c_char.as_mut_ptr(), &mut fml_max_len) {
                println!("Fget32 failed status {}",*_tmget_tperrno_addr());
            } else {
                println!("FML value {}",c_char_to_rust!(fml_c_char));
            }
            assert_eq!( c_char_to_rust!(fml_c_char),"test lib");
        }
    }
}
