use crate::{errors::MeterfeederErr, bindings::{MF_Initialize, MF_Shutdown, MF_Reset, MF_Clear, MF_GetNumberGenerators, MF_GetListGenerators, MF_GetBytes, MF_GetByte, MF_RandInt32, MF_RandUniform, MF_RandNormal}};

use std::ffi::{CString, CStr};

const EMPTY_ERR_BUFF: [i8; 256] = [0i8; 256];

pub struct MeterFeederInstance;

impl MeterFeederInstance {
    pub fn new() -> Result<MeterFeeder, MeterfeederErr> {
        MeterFeederInstance::init()
    }
    pub fn init()->Result<MeterFeeder, MeterfeederErr>{
        let mut mf_error = [0i8; 256];
        unsafe{
            MF_Initialize(mf_error.as_mut_ptr());
            if mf_error == EMPTY_ERR_BUFF {
                Ok(MeterFeeder)
            }else{
                let str_err = CStr::from_ptr(mf_error.as_ptr());
                Err(MeterfeederErr::GenericError(str_err.to_string_lossy().to_string()))
            }
        }
    }
}

pub struct MeterFeeder;

impl MeterFeeder {
    /// This api ensures that the same instance isn't reused if shutdown
    /// a new instance should be created of meterfeeder
    #[allow(unused_mut)]
    pub fn shutdown(mut self){
        unsafe {
            MF_Shutdown();
        }
    }
    
    pub fn reset(&mut self) -> Result<i32, MeterfeederErr> {
        let mut mf_error = [0i8; 256];
        unsafe{
            let returned = MF_Reset(mf_error.as_mut_ptr());
            if mf_error == EMPTY_ERR_BUFF {
                Ok(returned)
            }else{
                let str_err = CStr::from_ptr(mf_error.as_ptr());
                Err(MeterfeederErr::GenericError(str_err.to_string_lossy().to_string()))
            }
        }
    }
    
    pub fn clear(&mut self, generator_serial_number: &str) -> Result<bool, MeterfeederErr> {
        let mut mf_error = [0i8; 256];
        let c_generator_serial_number = CString::new(generator_serial_number).unwrap();
        unsafe{
            let returned = MF_Clear(c_generator_serial_number.as_ptr(),mf_error.as_mut_ptr());
            if mf_error == EMPTY_ERR_BUFF {
                Ok(returned)
            }else{
                let str_err = CStr::from_ptr(mf_error.as_ptr());
                Err(MeterfeederErr::GenericError(str_err.to_string_lossy().to_string()))
            }
        }
    }
    
    pub fn get_number_generators(&mut self) -> i32 {
        unsafe {
            MF_GetNumberGenerators()
        }
    }

    pub fn list_generators(&mut self) -> Vec<String> {
        // initialize a vector with 58 items long char* with length `get_number_generators`
        let mut generators_list = vec![ [0i8; 58].as_mut_ptr(); self.get_number_generators().try_into().unwrap()];
        unsafe{
            MF_GetListGenerators(generators_list.as_mut_ptr());
            // Convert CStr to String
            generators_list
            .into_iter()
            // Make sure no null pointers get through
            .filter(|generator| !generator.is_null())
            .map(
                |generator| 
                    CStr::from_ptr(generator).to_string_lossy().to_string()
            )
            .collect::<Vec<String>>()
        }
    }

    pub fn get_bytes(&mut self, length: i32, generator_serial_number: &str) -> Result<Vec<u8>, MeterfeederErr> {
        let mut buffer:Vec<u8> = vec![0; length.try_into().unwrap()];
        let c_generator_serial_number = CString::new(generator_serial_number).unwrap();
        let mut mf_error = [0i8; 256];
        
        unsafe {
            MF_GetBytes(length, buffer.as_mut_ptr(), c_generator_serial_number.as_ptr(), mf_error.as_mut_ptr());
            if mf_error == EMPTY_ERR_BUFF {
                Ok(buffer)
            }else{
                let str_err = CStr::from_ptr(mf_error.as_ptr());
                Err(MeterfeederErr::GenericError(str_err.to_string_lossy().to_string()))
            }
        }
    }

    pub fn get_byte(&mut self, generator_serial_number: &str) -> Result<u8, MeterfeederErr> {
        let c_generator_serial_number = CString::new(generator_serial_number).unwrap();
        let mut mf_error = [0i8; 256];
        
        unsafe {
            let returned = MF_GetByte(c_generator_serial_number.as_ptr(), mf_error.as_mut_ptr());
            if mf_error == EMPTY_ERR_BUFF {
                Ok(returned)
            }else{
                let str_err = CStr::from_ptr(mf_error.as_ptr());
                Err(MeterfeederErr::GenericError(str_err.to_string_lossy().to_string()))
            }
        }
    }

    pub fn rand_int_32(&mut self, generator_serial_number: &str) -> Result<i32, MeterfeederErr> {
        let c_generator_serial_number = CString::new(generator_serial_number).unwrap();
        let mut mf_error = [0i8; 256];
        
        unsafe {
            let returned = MF_RandInt32(c_generator_serial_number.as_ptr(), mf_error.as_mut_ptr());
            if mf_error == EMPTY_ERR_BUFF {
                Ok(returned)
            }else{
                let str_err = CStr::from_ptr(mf_error.as_ptr());
                Err(MeterfeederErr::GenericError(str_err.to_string_lossy().to_string()))
            }
        }
    }

    pub fn rand_uniform(&mut self, generator_serial_number: &str) -> Result<f64, MeterfeederErr> {
        let c_generator_serial_number = CString::new(generator_serial_number).unwrap();
        let mut mf_error = [0i8; 256];
        
        unsafe {
            let returned = MF_RandUniform(c_generator_serial_number.as_ptr(), mf_error.as_mut_ptr());
            if mf_error == EMPTY_ERR_BUFF {
                Ok(returned)
            }else{
                let str_err = CStr::from_ptr(mf_error.as_ptr());
                Err(MeterfeederErr::GenericError(str_err.to_string_lossy().to_string()))
            }
        }
    }

    pub fn rand_normal(&mut self, generator_serial_number: &str) -> Result<f64, MeterfeederErr> {
        let c_generator_serial_number = CString::new(generator_serial_number).unwrap();
        let mut mf_error = [0i8; 256];
        
        unsafe {
            let returned = MF_RandNormal(c_generator_serial_number.as_ptr(), mf_error.as_mut_ptr());
            if mf_error == EMPTY_ERR_BUFF {
                Ok(returned)
            }else{
                let str_err = CStr::from_ptr(mf_error.as_ptr());
                Err(MeterfeederErr::GenericError(str_err.to_string_lossy().to_string()))
            }
        }
    }

}
