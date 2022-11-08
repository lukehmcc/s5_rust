pub mod node {

    use crate::modules::{Config, P2PService};
    pub struct Node{
        pub config: Config,
    }
    impl Node {
        pub async fn start_node(self) {
            // init p2p service
            let sled = sled::open(&self.config.database.path).expect("Open sled");
            let p2p: P2PService = P2PService{
                config: self.config.clone(),
                sled: sled.clone(),
            };
            p2p.init().await;
            // store the config in sled in bincode so functions without the node/p2pservice 
            // state can access it
            let config_encoded: Vec<u8> = bincode::serialize(&self.config.clone()).expect("config failed to serialize");
            sled.insert("config", config_encoded);
            
            // TODO implement cache cleaner
            let _cache_path = &self.config.cache.path;
            
            // TODO implement store
            if self.config.store.expose {
                // do the store things
            }
            
            let p2p: P2PService = P2PService{
                config: self.config.clone(),
                sled: sled.clone(),
            };
            tokio::task::spawn(p2p.start());
            // keeps the program alive
            loop {}
        }
    }  
}
