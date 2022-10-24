pub mod config {
    use serde::Deserialize;
    use std::collections::HashMap;

    
    #[derive(Deserialize, Debug, Clone)]
    pub struct Config {
        pub name: String,
        pub keypair: KeyPair,
        pub database: DataBase,
        pub cache: Cache,
        pub http: Http,
        pub p2p: P2P,
        pub store: Store,
        
    }

    #[derive(Deserialize, Debug, Clone)]
    pub struct KeyPair {
        pub seed: String,
    }
    
    #[derive(Deserialize, Debug, Clone)]
    pub struct DataBase {
        pub path: String,
    }

    #[derive(Deserialize, Debug, Clone)]
    pub struct Cache {
        pub path: String,
    }
    
    #[derive(Deserialize, Debug, Clone)]
    pub struct Http {
        pub api: API,
    }

    #[derive(Deserialize, Debug, Clone)]
    pub struct API {
        pub port: i64,
        pub delete: Delete,
    }

    #[derive(Deserialize, Debug, Clone)]
    pub struct Delete {
        pub enabled: bool,
    }
    
     #[derive(Deserialize, Debug, Clone)]
     pub struct P2P {
         pub peers: Peers,
     }

    #[derive(Deserialize, Debug, Clone)]
    pub struct Peers {
        pub initial: Vec<String>,
    }
    
    #[derive(Deserialize, Debug, Clone)]
    pub struct Store {
        pub expose: bool,
        pub s3: Option<HashMap<String, String>>,
        pub local: Option<HashMap<String, String>>,
        pub arweave: Option<HashMap<String, String>>,
    }
}