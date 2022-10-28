pub mod p2p {
    use concat_arrays::concat_arrays;
    use sha2::{Digest, Sha512};
    use sled::Db;
    use tokio::{net::TcpStream, io::{AsyncWriteExt, Interest, self}};
    use url::Url;
    use rand::{thread_rng, Rng};
    
    use crate::modules::{Config, SledWrappings, Constants};
    
    #[derive(Debug, Clone)]
    pub struct P2PService {
        pub config: Config,
        pub sled: Db,
    }
    
    #[derive(Debug)]
    pub struct Peer {
        pub id: String,
        pub id_bin: Vec<u8>,
        pub socket: TcpStream,
        pub connection_uri: Url,
        pub connected_peers: Vec<String>,
        pub challenge: Vec<u8>, // This should be 32 bytes in length
    }
    
    impl P2PService {
        //  Use this for protocol updates
        pub async fn init(self){
            // Generate Sha256 pubkey based on seed and set it to the node_id
            let mut hasher = Sha512::new();
            hasher.update(self.config.keypair.seed.as_bytes());
            let hashed_seed = hasher.finalize();
            let pk:ed25519_dalek::Keypair = ed25519_dalek::Keypair::from_bytes(&hashed_seed).expect("Seed should be available");
            let constants: Constants = Constants::get_constants();
            let prefix: [u8;1] = [constants.mkey_ed25519];
            let node_id_bin: [u8; 33] = concat_arrays!(prefix, pk.public.to_bytes());
            let node_id_bs58: String = P2PService::encode_node_id(node_id_bin.to_vec());
            println!("Local node id: {}", node_id_bs58);
            self.sled.insert("local_node_id", node_id_bin.to_vec()).expect("");
        }
        pub async fn start(self){
            // TODO add section for if p2p.self is configured
            if self.config.p2p.self_id != None {
                
            }
            let init_peers: Vec<String> = self.config.clone().p2p.peers.initial;
            for peer in init_peers {
                tokio::task::spawn(P2PService::connect_to_node(self.clone(), peer));
            }
        }
        pub async fn on_new_peer(addr: String, _connection_uri: Url) {
            let stream_res = TcpStream::connect(&addr).await;
            if stream_res.is_ok(){
                println!("connecting to: {}", &addr);
                let mut stream = stream_res.expect("failed to unwrap stream");
                let challenge: Vec<u8> = (0..32).map(|_| {
                    thread_rng().gen::<u8>()
                }).collect();
                let constants: Constants = Constants::get_constants();
                let mut initial_auth_payload_packer: Vec<u8> = Vec::new();
                rmp::encode::write_u8(&mut initial_auth_payload_packer, constants.protocol_method_handshake_open).expect("Failed to pack");
                rmp::encode::write_bin(&mut initial_auth_payload_packer, &challenge[..]).expect("failed to pack");
                match stream.write_all(&initial_auth_payload_packer[..]).await {
                    Ok(back) => {println!("{:?}", back)},
                    Err(err ) => {println!("{}", err);return;},
                }
                // now loop until the heck out of it
                loop {
                    dbg!("looping");
                    let ready = stream.ready(Interest::READABLE).await;
                    if ready.is_ok(){
                    let ready_ok = ready.unwrap();
                    if ready_ok.is_readable() {
                        let mut data = vec![0; 1024];
                        // Try to read data, this may still fail with `WouldBlock`
                        // if the readiness event is a false positive.
                        match stream.try_read(&mut data) {
                            Ok(n) => {
                                println!("read {} bytes", n);        
                            }
                            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                                continue;
                            }
                            Err(e) => {
                                dbg!(e);
                                return;
                            }
                        }
                    }                                
                }
            }            
        } else {
            println!("failed to conenct to: {}", &addr);
            return;
        }}
        pub fn encode_node_id(node_id_bytes: Vec<u8>) -> String {
            return format!("{}{}", "z".to_owned(), bs58::encode(node_id_bytes).into_string());
        }
        pub async fn connect_to_node(self, peer: String) {
            let parsed_peer: Option<Url> = match Url::parse(&peer) {
              Ok(parsed) => Some(parsed),
              Err(_) => None, 
            };
            if parsed_peer == None {
                return;
            }else{                
                let peer_uri = parsed_peer.unwrap();
                let protocol = peer_uri.scheme();
                let id = peer_uri.username();
                let ip = peer_uri.host().expect("Failed to unwrap hostname").to_owned().to_string();
                let port = peer_uri.port().expect("Failed to unwrap port");
                if protocol != "tcp" {
                    print!("Protocol {} is not supported from uri: {}\n", protocol, peer);
                    return;
                }
                let local_id  = self.sled.get(&"local_node_id").expect("Failed to get local node id from sled");
                let local_id_str: String = P2PService::encode_node_id(local_id.unwrap().to_vec());
                if local_id_str == id {
                    return;
                }
                let mut reconnect_delay_path: String = id.clone().to_owned();
                reconnect_delay_path.push_str("-rcd");
                let mut reconnect_delay: i32 = SledWrappings::get_i32(reconnect_delay_path.clone(), self.sled.clone()).await;
                if reconnect_delay == -1 {
                    reconnect_delay = 1;
                    SledWrappings::set_i32(reconnect_delay_path, reconnect_delay, self.sled.clone()).await;
                }
                let mut addr = ip;
                addr.push_str(":");
                addr.push_str(&port.to_string());
                tokio::task::spawn(P2PService::on_new_peer(addr, peer_uri));
                // let stream = TcpStream::connect(addr.clone());
                // if stream.await.is.is_ok() {
                //     println!("connecting to: {:?}", addr.clone());
                //     println!("Connected to the server!");
                //     tokio::task::spawn(P2PService::on_new_peer(listener.unwrap(), peer_uri));
                // } else {
                //     println!("connecting to: {:?}", addr);
                //     println!("Couldn't connect to server...");
                // }
            }
        }
    }
}