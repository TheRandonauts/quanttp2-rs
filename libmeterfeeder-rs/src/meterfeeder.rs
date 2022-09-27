use crate::{
    bindings::{
        BYTES_LENGTH_LIMIT, MF_Clear, MF_GetByte, MF_GetBytes, MF_GetListGenerators,
        MF_GetNumberGenerators, MF_Initialize, MF_RandInt32, MF_RandNormal, MF_RandUniform,
        MF_Reset, MF_Shutdown, BYTES_LENGTH_LIMIT_I32,
    },
    errors::MeterfeederErr,
};

use std::ffi::{CStr, CString};

const ERR_BUFF_SIZE: usize = 256;
const EMPTY_ERR_BUFF: [i8; ERR_BUFF_SIZE] = [0i8; ERR_BUFF_SIZE];
const GENERATOR_BUFF_SIZE: usize = 58;
const EMPTY_GENERATOR_BUFF: [i8; GENERATOR_BUFF_SIZE] = [0i8; GENERATOR_BUFF_SIZE];

pub struct MeterFeederInstance;

impl MeterFeederInstance {
    pub fn new() -> Result<MeterFeeder, MeterfeederErr> {
        MeterFeederInstance::init()
    }
    pub fn init() -> Result<MeterFeeder, MeterfeederErr> {
        let mut mf_error = [0i8; ERR_BUFF_SIZE];
        unsafe {
            MF_Initialize(mf_error.as_mut_ptr());
            if mf_error == EMPTY_ERR_BUFF {
                Ok(MeterFeeder)
            } else {
                let str_err = CStr::from_ptr(mf_error.as_ptr());
                Err(MeterfeederErr::GenericError(
                    str_err.to_string_lossy().to_string(),
                ))
            }
        }
    }
}

pub struct MeterFeeder;

impl MeterFeeder {
    /// This api ensures that the same instance isn't reused if shutdown
    /// a new instance should be created of meterfeeder
    #[allow(unused_mut)]
    pub fn shutdown(mut self) {
        unsafe {
            MF_Shutdown();
        }
    }

    pub fn reset(&mut self) -> Result<i32, MeterfeederErr> {
        let mut mf_error = [0i8; ERR_BUFF_SIZE];
        unsafe {
            let returned = MF_Reset(mf_error.as_mut_ptr());
            if mf_error == EMPTY_ERR_BUFF {
                Ok(returned)
            } else {
                let str_err = CStr::from_ptr(mf_error.as_ptr());
                Err(MeterfeederErr::GenericError(
                    str_err.to_string_lossy().to_string(),
                ))
            }
        }
    }

    pub fn clear(&mut self, generator_serial_number: &str) -> Result<bool, MeterfeederErr> {
        let mut mf_error = [0i8; ERR_BUFF_SIZE];
        let c_generator_serial_number = CString::new(generator_serial_number).unwrap();
        unsafe {
            let returned = MF_Clear(c_generator_serial_number.as_ptr(), mf_error.as_mut_ptr());
            if mf_error == EMPTY_ERR_BUFF {
                Ok(returned)
            } else {
                let str_err = CStr::from_ptr(mf_error.as_ptr());
                Err(MeterfeederErr::GenericError(
                    str_err.to_string_lossy().to_string(),
                ))
            }
        }
    }

    pub fn get_number_generators(&self) -> i32 {
        unsafe { MF_GetNumberGenerators() }
    }

    pub fn list_generators(&self) -> Vec<String> {
        // initialize a vector with 58 items long char* with length `get_number_generators`
        let mut generators_list =
            vec![[0i8; GENERATOR_BUFF_SIZE]; self.get_number_generators().try_into().unwrap()];
        unsafe {
            let mut generators_pointers_list: Vec<*mut i8> = generators_list
                .iter_mut()
                .map(|generator| generator.as_mut_ptr())
                .collect();
            MF_GetListGenerators(generators_pointers_list.as_mut_ptr());
            // Convert CStr to String
            generators_pointers_list
                .into_iter()
                // Make sure only added strings come through
                .filter(|generator| {
                    core::slice::from_raw_parts(*generator, GENERATOR_BUFF_SIZE)
                        != EMPTY_GENERATOR_BUFF
                })
                .map(|generator| CStr::from_ptr(generator).to_str())
                .filter_map(Result::ok)
                .map(|generator| generator[..8].to_string())
                .collect::<Vec<String>>()
        }
    }

    pub fn get_bytes(
        &mut self,
        length_i32: i32,
        generator_serial_number: &str,
    ) -> Result<Vec<u8>, MeterfeederErr> {
        let length:usize = length_i32.try_into().unwrap();
        let split_request = length > BYTES_LENGTH_LIMIT;
        let c_generator_serial_number = CString::new(generator_serial_number).unwrap();

        if split_request {
            let mut mf_error = EMPTY_ERR_BUFF.clone();

            // Perform ceiling division
            let mut iterations = (length+BYTES_LENGTH_LIMIT-1) / BYTES_LENGTH_LIMIT;
            let mut complete_buffer: Vec<u8> = Vec::with_capacity(length);
            while iterations != 0 {
                let mut split_buffer: Vec<u8> = vec![0; BYTES_LENGTH_LIMIT];
                unsafe {
                    MF_GetBytes(
                        BYTES_LENGTH_LIMIT_I32,
                        split_buffer.as_mut_ptr(),
                        c_generator_serial_number.as_ptr(),
                        mf_error.as_mut_ptr(),
                    );
                    iterations -= 1;
                    complete_buffer.extend_from_slice(&split_buffer);
                    mf_error = EMPTY_ERR_BUFF.clone();
                    if mf_error != EMPTY_ERR_BUFF {
                        let str_err = CStr::from_ptr(mf_error.as_ptr());
                        return Err(MeterfeederErr::GenericError(
                            str_err.to_string_lossy().to_string(),
                        ));
                    }
                }
            }
            // since splitting is just greedy final buffer will be larger and thus needs to be shrunk
            complete_buffer.resize(length, 0);
            Ok(complete_buffer)
        } else {
            let mut mf_error = EMPTY_ERR_BUFF.clone();
            let mut buffer: Vec<u8> = vec![0; length];
            unsafe {
                MF_GetBytes(
                    length_i32,
                    buffer.as_mut_ptr(),
                    c_generator_serial_number.as_ptr(),
                    mf_error.as_mut_ptr(),
                );
                if mf_error == EMPTY_ERR_BUFF {
                    Ok(buffer)
                } else {
                    let str_err = CStr::from_ptr(mf_error.as_ptr());
                    Err(MeterfeederErr::GenericError(
                        str_err.to_string_lossy().to_string(),
                    ))
                }
            }
        }
    }

    pub fn get_byte(&mut self, generator_serial_number: &str) -> Result<u8, MeterfeederErr> {
        let c_generator_serial_number = CString::new(generator_serial_number).unwrap();
        let mut mf_error = [0i8; ERR_BUFF_SIZE];

        unsafe {
            let returned = MF_GetByte(c_generator_serial_number.as_ptr(), mf_error.as_mut_ptr());
            if mf_error == EMPTY_ERR_BUFF {
                Ok(returned)
            } else {
                let str_err = CStr::from_ptr(mf_error.as_ptr());
                Err(MeterfeederErr::GenericError(
                    str_err.to_string_lossy().to_string(),
                ))
            }
        }
    }

    pub fn rand_int_32(&mut self, generator_serial_number: &str) -> Result<i32, MeterfeederErr> {
        let c_generator_serial_number = CString::new(generator_serial_number).unwrap();
        let mut mf_error = [0i8; ERR_BUFF_SIZE];

        unsafe {
            let returned = MF_RandInt32(c_generator_serial_number.as_ptr(), mf_error.as_mut_ptr());
            if mf_error == EMPTY_ERR_BUFF {
                Ok(returned)
            } else {
                let str_err = CStr::from_ptr(mf_error.as_ptr());
                Err(MeterfeederErr::GenericError(
                    str_err.to_string_lossy().to_string(),
                ))
            }
        }
    }

    pub fn rand_uniform(&mut self, generator_serial_number: &str) -> Result<f64, MeterfeederErr> {
        let c_generator_serial_number = CString::new(generator_serial_number).unwrap();
        let mut mf_error = [0i8; ERR_BUFF_SIZE];

        unsafe {
            let returned =
                MF_RandUniform(c_generator_serial_number.as_ptr(), mf_error.as_mut_ptr());
            if mf_error == EMPTY_ERR_BUFF {
                Ok(returned)
            } else {
                let str_err = CStr::from_ptr(mf_error.as_ptr());
                Err(MeterfeederErr::GenericError(
                    str_err.to_string_lossy().to_string(),
                ))
            }
        }
    }

    pub fn rand_normal(&mut self, generator_serial_number: &str) -> Result<f64, MeterfeederErr> {
        let c_generator_serial_number = CString::new(generator_serial_number).unwrap();
        let mut mf_error = [0i8; ERR_BUFF_SIZE];

        unsafe {
            let returned = MF_RandNormal(c_generator_serial_number.as_ptr(), mf_error.as_mut_ptr());
            if mf_error == EMPTY_ERR_BUFF {
                Ok(returned)
            } else {
                let str_err = CStr::from_ptr(mf_error.as_ptr());
                Err(MeterfeederErr::GenericError(
                    str_err.to_string_lossy().to_string(),
                ))
            }
        }
    }
}
