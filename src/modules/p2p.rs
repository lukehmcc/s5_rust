pub mod p2p {
    use concat_arrays::concat_arrays;
    use sha2::{Digest, Sha512};
    use sled::Db;

    use crate::modules::Config;
    
    #[derive(Debug, Clone)]
    pub struct P2PService {
        pub config: Config,
        pub sled: Db,
    }
    
    impl P2PService {
        pub async fn init(self){
            // Generate Sha256 pubkey based on seed and set it to the node_id
            let mut hasher = Sha512::new();
            hasher.update(self.config.keypair.seed.as_bytes());
            let hashed_seed = hasher.finalize();
            let pk:ed25519_dalek::Keypair = ed25519_dalek::Keypair::from_bytes(&hashed_seed).expect("Seed should be available");
            let prefix: [u8;1] = [0xed];
            let node_id_bin: [u8; 33] = concat_arrays!(prefix, pk.public.to_bytes());
            println!("Local node id: {}", String::from_utf8_lossy(&node_id_bin));
            self.sled.insert("local_node_id", node_id_bin.to_ascii_lowercase()).unwrap();
        }
        pub async fn start(self){
        }
    }
}