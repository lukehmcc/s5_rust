pub mod config {
    use serde::Deserialize;
    
    #[derive(Deserialize, Debug, Clone)]
    pub struct Config {
        pub name: String,
        pub keypair: KeyPair,
        pub database: DataBase,
        pub cache: Cache,
        pub http: Http,
        pub network: Network,
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
     pub struct Network {
         pub peers: Peers,
     }

    #[derive(Deserialize, Debug, Clone)]
    pub struct Peers {
        pub initial: Vec<String>,
    }
}