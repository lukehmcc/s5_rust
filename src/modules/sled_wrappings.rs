pub mod sled_wrappings {
    use sled::Db;
    pub struct SledWrappings{}
    impl SledWrappings{
        
        pub async fn set_string(key: String, val: String, sled: Db) {
            let bin_val: Vec<u8> = bincode::serialize(&val).unwrap();
            let res = sled.insert(&key, bin_val);
            if res.is_ok() {
                println!("{} was set to {}", key, val);
            }
        }
        pub async fn get_string(key: String, sled: Db) -> String{
                let res = sled.get(&key);
                if res.is_ok() {
                    let byte_vec = res.unwrap().unwrap().to_vec();
                    let stringy: String = bincode::deserialize(&byte_vec[..]).unwrap();
                    return stringy;
                } else {
                    return "".to_owned();
                }
        }
        pub async fn set_i32(key: String, val: i32, sled: Db) {
            let bin_val: Vec<u8> = bincode::serialize(&val).unwrap();
            let res = sled.insert(&key, bin_val);
            if res.is_ok() {
                println!("{} was set to {}", key, val);
            }
        }
        pub async fn get_i32(key: String, sled: Db) -> i32{
                let res = sled.get(&key);
                if res.is_ok() {
                    let byte_vec = res.unwrap().unwrap().to_vec();
                    let stringy: i32 = bincode::deserialize(&byte_vec[..]).unwrap();
                    return stringy;
                } else {
                    return -1;
                }
        }
    }
}