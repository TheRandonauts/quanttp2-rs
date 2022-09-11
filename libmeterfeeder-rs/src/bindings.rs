use libc::{ c_char, c_uchar, c_double};

// https://stackoverflow.com/a/40834178/5775114
// #[link(name = ":./libmeterfeeder.so")]
extern {
    pub fn MF_Initialize(pErrorReason: *mut c_char) -> i32;
    pub fn MF_Shutdown();
    pub fn MF_Reset(pErrorReason: *mut c_char)->i32;
    pub fn MF_Clear(generatorSerialNumber: *const c_char, pErrorReason: *mut c_char)->bool;
    pub fn MF_GetNumberGenerators()->i32;
    pub fn MF_GetListGenerators(pGenerators: *mut *mut c_char);
    pub fn MF_GetSerialListGenerators(pGenerators: *mut *mut c_char);
    pub fn MF_GetBytes(length: i32, buffer: *mut c_uchar, generatorSerialNumber: *const c_char, pErrorReason: *mut c_char);
    pub fn MF_GetByte(generatorSerialNumber: *const c_char, pErrorReason: *mut c_char)->c_uchar;
    pub fn MF_RandInt32(generatorSerialNumber: *const c_char, pErrorReason: *mut c_char)->i32;
    pub fn MF_RandUniform(generatorSerialNumber: *const c_char, pErrorReason: *mut c_char)->c_double;
    pub fn MF_RandNormal(generatorSerialNumber: *const c_char, pErrorReason: *mut c_char)->c_double;
}   