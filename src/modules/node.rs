pub mod node {
    use crate::{modules::{Config, P2PService}};
    
    pub struct Node{
        pub config: Config,
    }
    impl Node {
        pub async fn start_node(self) {
            let _tree = sled::open(&self.config.database.path).expect("Open sled");
            let p2p: P2PService = P2PService{
                config: self.config.clone()
            };
            p2p.init().await;
        }
    }  
}
