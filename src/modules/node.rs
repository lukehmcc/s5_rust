pub mod node {

    use crate::{modules::{Config, P2PService}};
    pub struct Node{
        pub config: Config,
    }
    impl Node {
        pub async fn start_node(self) {
            // init p2p service
            let _sled = sled::open(&self.config.database.path).expect("Open sled");
            let p2p: P2PService = P2PService{
                config: self.config.clone(),
                sled: _sled.clone(),
            };
            p2p.init().await;
            
            // TODO implement cache cleaner
            let _cache_path = &self.config.cache.path;
            
            // TODO implement store
            if self.config.store.expose {
                // do the store things
            }
            
            let p2p: P2PService = P2PService{
                config: self.config.clone(),
                sled: _sled.clone(),
            };
            p2p.start().await;
        }
    }  
}
