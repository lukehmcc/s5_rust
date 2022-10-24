pub mod p2p {
    use concat_arrays::concat_arrays;
    use ed25519_dalek::PublicKey;
    use sha2::{Digest, Sha256};
    use sled::Db;

    use crate::modules::Config;
    
    pub struct P2PService {
        pub config: Config,
        pub sled: Db,
    }
    
    impl P2PService {
        pub async fn init(self){
            let mut hasher = Sha256::new();
            hasher.update(self.config.keypair.seed.as_bytes());
            let hashed_seed = hasher.finalize();
            let pk:PublicKey = ed25519_dalek::PublicKey::from_bytes(&hashed_seed).expect("Seed should be available");
            let prefix: [u8;1] = [0xed];
            let node_id_bin: [u8; 33] = concat_arrays!(prefix, pk.to_bytes());
            self.sled.insert("node_id", node_id_bin.to_ascii_lowercase()).unwrap();
        }
    }
}