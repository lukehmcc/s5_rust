pub mod p2p {
    use concat_arrays::concat_arrays;
    use ed25519_dalek::PublicKey;
    use hex::ToHex;
    use sha2::{Digest, Sha256};

    use crate::modules::Config;
    
    pub struct P2PService {
        pub config: Config,
    }
    
    impl P2PService {
        pub async fn init(self){
            let mut hasher = Sha256::new();
            hasher.update(self.config.keypair.seed.as_bytes());
            let hashed_seed = hasher.finalize();
            let pk:PublicKey = ed25519_dalek::PublicKey::from_bytes(&hashed_seed).expect("Seed should be available");
            let pk_byte: [u8; 32] = pk.to_bytes();
            let prefix: [u8; 1] = [0xed];
            let node_id_bin: [u8; 33] = concat_arrays!(prefix, pk_byte);
            let tree = sled::open(&self.config.database.path).expect("Open sled");
            tree.insert("node_id", node_id_bin.to_ascii_lowercase());
        }
    }
}