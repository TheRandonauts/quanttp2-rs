use libc::{ c_char, c_uchar, c_double};
#[allow(unused)]

extern {
    fn MF_Initialize(pErrorReason: *mut c_char) -> i32;
    fn MF_Shutdown()->();
    fn MF_Reset(pErrorReason: *mut c_char)->i32;
    fn MF_Clear(generatorSerialNumber: *const c_char, pErrorReason: *mut c_char)->bool;
    fn MF_GetNumberGenerators()->i32;
    fn MF_GetListGenerators(pGenerators: *mut *mut c_char)->();
    fn MF_GetBytes(length: i32, buffer: *mut c_uchar, generatorSerialNumber: *const c_uchar, pErrorReason: *mut c_char)->();
    fn MF_GetByte(generatorSerialNumber: *const c_uchar, pErrorReason: *mut c_char)->c_uchar;
    fn MF_RandInt32(generatorSerialNumber: *const c_uchar, pErrorReason: *mut c_char)->i32;
    fn MF_RandUniform(generatorSerialNumber: *const c_uchar, pErrorReason: *mut c_char)->c_double;
    fn MF_RandNormal(generatorSerialNumber: *const c_uchar, pErrorReason: *mut c_char)->c_double;
}


#[cfg(test)]
mod tests {
    use crate::MF_Initialize;
    use std::ffi::CString;
    #[test]
    fn it_works() {

        let c_string = CString::new("foo").expect("CString::new failed");
        let ptr = c_string.into_raw();

        unsafe{
            MF_Initialize(ptr);
            let c_string = CString::from_raw(ptr);
            println!("{:?}", c_string);
        }

    }
}
