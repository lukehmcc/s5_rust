pub mod config {
    use serde::Deserialize;

    #[derive(Deserialize, Debug)]
    pub struct Config {
        pub name: String,
        pub keypair: KeyPair,
        pub cache: Cache,
        pub http_api: HttpAPI,
        pub http_api_delete: HttpApiDelete,
        pub network_peers: NetworkPeers,
    }

    #[derive(Deserialize, Debug)]
    pub struct KeyPair {
        pub seed: String,
    }

    #[derive(Deserialize, Debug)]
    pub struct Cache {
        pub path: String,
    }

    #[derive(Deserialize, Debug)]
    pub struct HttpAPI {
        pub port: i64,
    }

    #[derive(Deserialize, Debug)]
    pub struct HttpApiDelete {
        pub enabled: bool,
    }

    #[derive(Deserialize, Debug)]
    pub struct NetworkPeers {
        pub initial: Vec<String>,
    }
}