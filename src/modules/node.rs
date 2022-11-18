pub mod node {

    use std::collections::HashMap;

    use state::Storage;

    use crate::modules::{Config, P2PService, p2p::p2p::Peer};
    pub struct Node{
        pub config: Config,
    }
    impl Node {
        pub async fn start_node(self) {
            // init p2p service
            
            // create mutable peer hashmap
            let peers_hm_store: Storage<HashMap<String, Peer>> = state::Storage::new();
            let peers_hm: HashMap<String, Peer> = HashMap::new();
            peers_hm_store.set(peers_hm);
            let sled = sled::open(&self.config.database.path).expect("Open sled");
            let p2p: P2PService = P2PService{
                peers: peers_hm_store.clone(),
                config: self.config.clone(),
                sled: sled.clone(),
            };
            p2p.init().await;
            
            // TODO implement cache cleaner
            let _cache_path = &self.config.cache.path;
            
            // TODO implement store
            if self.config.store.expose {
                // do the store things
            }
            
            let p2p: P2PService = P2PService{
                peers: peers_hm_store.clone(),
                config: self.config.clone(),
                sled: sled.clone(),
            };
            tokio::task::spawn(p2p.start());
            // keeps the program alive
            loop {}
        }
    }  
}
