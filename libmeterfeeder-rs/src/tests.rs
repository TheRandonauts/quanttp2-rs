
#[cfg(test)]
mod tests {
    use crate::{meterfeeder::MeterFeederInstance};
    // #[test]
    // fn it_works() {
    //     let c_string = CString::new("foo").expect("CString::new failed");
    //     let ptr = c_string.into_raw();

    //     unsafe{
    //         MF_Initialize(ptr);
    //         let c_string = CString::from_raw(ptr);
    //         println!("{:?}", c_string);
    //     }
    // }

    #[test]
    fn wrapper_init_test(){
        println!("Here");
        MeterFeederInstance::new().expect("Failed to create instance");
    }

    #[test]
    fn wrapper_shutdown_test(){
        let instance= MeterFeederInstance::new().expect("Failed to create instance");
        instance.shutdown();
    }

    #[test]
    fn wrapper_reset_test(){
        let mut instance= MeterFeederInstance::new().expect("Failed to create instance");
        instance.reset().expect("Failed to reset devices");
    }


    #[test]
    fn wrapper_get_gen(){
        let mut instance= MeterFeederInstance::new().expect("Failed to create instance");
        assert!(instance.get_number_generators() > 0, "No generators found");
    }

    #[test]
    fn wrapper_list_gen(){
        let mut instance= MeterFeederInstance::new().expect("Failed to create instance");
        assert!(!instance.list_generators().is_empty(), "No generators list returned");
    }

    #[test]
    fn wrapper_get_bytes(){
        let mut instance= MeterFeederInstance::new().expect("Failed to create instance");
        let BUFFER_LENGTH = 100_000;
        let GENERATOR_INDEX = 0;
        let GENERATORS = instance.list_generators();
        let GENERATOR = GENERATORS.get(GENERATOR_INDEX).expect("Failed to get generator");
        assert!(instance.get_bytes(BUFFER_LENGTH, GENERATOR).expect("Failed to get bytes") != vec![0;BUFFER_LENGTH.try_into().unwrap()], "No generators list returned");
    }

    #[test]
    fn wrapper_get_byte(){
        let mut instance= MeterFeederInstance::new().expect("Failed to create instance");
        let GENERATOR_INDEX = 0;
        let GENERATORS = instance.list_generators();
        let GENERATOR = GENERATORS.get(GENERATOR_INDEX).expect("Failed to get generator");
        assert!(instance.get_byte(GENERATOR).expect("Failed to get bytes") != 0, "No generators list returned");
    }


    #[test]
    fn wrapper_get_rand_int_32(){
        let mut instance= MeterFeederInstance::new().expect("Failed to create instance");
        let GENERATOR_INDEX = 0;
        let GENERATORS = instance.list_generators();
        let GENERATOR = GENERATORS.get(GENERATOR_INDEX).expect("Failed to get generator");
        assert!(instance.rand_int_32(GENERATOR).expect("Failed to get bytes") != 0, "No generators list returned");
    }

    #[test]
    fn wrapper_get_rand_uniform(){
        let mut instance= MeterFeederInstance::new().expect("Failed to create instance");
        let GENERATOR_INDEX = 0;
        let GENERATORS = instance.list_generators();
        let GENERATOR = GENERATORS.get(GENERATOR_INDEX).expect("Failed to get generator");
        // Test is naive just checking against 0
        assert!(instance.rand_uniform(GENERATOR).expect("Failed to get bytes") > 0 as f64, "No generators list returned");
    }

    #[test]
    fn wrapper_get_rand_normal(){
        let mut instance= MeterFeederInstance::new().expect("Failed to create instance");
        let GENERATOR_INDEX = 0;
        let GENERATORS = instance.list_generators();
        let GENERATOR = GENERATORS.get(GENERATOR_INDEX).expect("Failed to get generator");
        // Test is naive just checking against 0
        assert!(instance.rand_normal(GENERATOR).expect("Failed to get bytes") > 0 as f64, "No generators list returned");
    }


}
