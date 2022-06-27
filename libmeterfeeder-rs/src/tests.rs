
#[cfg(test)]
mod tests {
    use crate::{meterfeeder::MeterFeederInstance};

    #[test]
    fn wrapper_init_test(){
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
        const BUFFER_LENGTH:i32 = 100_000;
        const GENERATOR_INDEX: usize = 0;
        let generators = instance.list_generators();
        let generator = generators.get(GENERATOR_INDEX).expect("Failed to get generator");
        assert!(instance.get_bytes(BUFFER_LENGTH, generator).expect("Failed to get bytes") != vec![0;BUFFER_LENGTH.try_into().unwrap()], "No generators list returned");
    }

    #[test]
    fn wrapper_get_byte(){
        let mut instance= MeterFeederInstance::new().expect("Failed to create instance");
        const GENERATOR_INDEX:usize = 0;
        let generators = instance.list_generators();
        let generator = generators.get(GENERATOR_INDEX).expect("Failed to get generator");
        assert!(instance.get_byte(generator).expect("Failed to get bytes") != 0, "No generators list returned");
    }


    #[test]
    fn wrapper_get_rand_int_32(){
        let mut instance= MeterFeederInstance::new().expect("Failed to create instance");
        const GENERATOR_INDEX: usize = 0;
        let generators = instance.list_generators();
        let generator = generators.get(GENERATOR_INDEX).expect("Failed to get generator");
        assert!(instance.rand_int_32(generator).expect("Failed to get bytes") != 0, "No generators list returned");
    }

    #[test]
    fn wrapper_get_rand_uniform(){
        let mut instance= MeterFeederInstance::new().expect("Failed to create instance");
        const GENERATOR_INDEX: usize = 0;
        let generators = instance.list_generators();
        let generator = generators.get(GENERATOR_INDEX).expect("Failed to get generator");
        // Test is naive just checking against 0
        assert!(instance.rand_uniform(generator).expect("Failed to get bytes") > 0 as f64, "No generators list returned");
    }

    #[test]
    fn wrapper_get_rand_normal(){
        let mut instance= MeterFeederInstance::new().expect("Failed to create instance");
        const GENERATOR_INDEX: usize = 0;
        let generators = instance.list_generators();
        let generator = generators.get(GENERATOR_INDEX).expect("Failed to get generator");
        // Test is naive just checking against 0
        assert!(instance.rand_normal(generator).expect("Failed to get bytes") > 0 as f64, "No generators list returned");
    }


}
