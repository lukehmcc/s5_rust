pub mod p2p {
    use std::{io::Cursor, collections::HashMap};

    use concat_arrays::concat_arrays;
    use ed25519_dalek::{Keypair, Signature, PublicKey, Verifier};
    use rmpv::Value;
    use state::Storage;
    use std::error::Error;
    use sha2::{Digest, Sha512};
    use sled::Db;
    use tokio::io::{AsyncWriteExt, Interest, self};
    use tokio::net::TcpStream;
    use url::Url;
    use rand::{thread_rng, Rng};
    use rmp::{encode,decode};
    
    use crate::modules::{Config, SledWrappings, Constants};
    
    #[derive(Debug, Clone)]
    pub struct P2PService {
        pub config: Config,
        pub sled: Db,
        pub peers: Storage<HashMap<String, Peer>>,
    }
    
    #[derive(Debug, Clone)]
    pub struct Peer {
        pub id: Option<String>,
        pub bin_id: Option<Vec<u8>>,
        pub connection_uris: Vec<Url>,
        pub is_connected: bool,
        pub connected_peers: Option<Vec<String>>,
        pub challenge: Option<[u8; 32]>,
        
    }
    impl Default for Peer {
        fn default() -> Peer {
            Peer {
                id: Default::default(),
                bin_id: Default::default(),
                connection_uris: Default::default(),
                is_connected: false,
                connected_peers: Default::default(),
                challenge: Default::default(),

            }
        }
    }
    
    #[derive(Debug, Clone)]
    pub struct SignedMessage {
        pub node_id: String,
        pub message: Vec<u8>,
    }
    
    #[derive(Debug)]
    pub struct StringError (String);
    
    impl P2PService {
        //  Use this for protocol updates
        pub async fn init(self){
            // Generate Sha256 pubkey based on seed and set it to the node_id
            let kp: Keypair = P2PService::get_keypair(self.clone());
            let constants: Constants = Constants::get_constants();
            let prefix: [u8;1] = [constants.mkey_ed25519];
            let node_id_bin: [u8; 33] = concat_arrays!(prefix, kp.public.to_bytes());
            let node_id_bs58: String = P2PService::encode_node_id(node_id_bin.to_vec());
            println!("Local node id: {}", node_id_bs58);
            self.sled.insert("local_node_id", node_id_bin.to_vec()).expect("");
        }
        pub fn get_keypair(p2p: P2PService) -> Keypair {
            let mut hasher = Sha512::new();
            hasher.update(p2p.config.keypair.seed.as_bytes());
            let hashed_seed = hasher.finalize();
            let kp:ed25519_dalek::Keypair = ed25519_dalek::Keypair::from_bytes(&hashed_seed).expect("Seed should be available");
            return kp;
        }
        pub async fn start(self){
            // TODO add section for if p2p.self is configured
            if self.config.p2p.self_id != None {
                
            }
            let init_peers: Vec<String> = self.config.clone().p2p.peers.initial;
            for peer in init_peers {
                tokio::task::spawn(P2PService::connect_to_node(self.clone(), vec![peer]));
            }
        }
        pub async fn on_new_peer(self, peer: Peer) {
            let mut peer: Peer = peer;
            let ip: String = peer.connection_uris[0].host().expect("Failed to unwrap hostname").to_owned().to_string();
            let port: u16 = peer.connection_uris[0].port().expect("Failed to unwrap port");
            let mut addr: String = ip;
            addr.push_str(":");
            addr.push_str(&port.to_string());
            let id = peer.connection_uris[0].username();
            peer.id = Some(id.to_owned());
            peer.bin_id = Some(P2PService::decode_node_id(id.to_owned()));
            let stream_res = TcpStream::connect(&addr).await;
            self.peers.get();
            if stream_res.is_ok(){
                // if the stream is okay we pack the handshake open call and the challenge then send it off to the peer
                println!("connecting to: {}", &addr);
                let mut stream = stream_res.expect("failed to unwrap stream");
                // the challenge here is a random value that is expected to be sent back by the peer to verify they are using
                // the same protocol
                let challenge: Vec<u8> = (0..32).map(|_| {
                    thread_rng().gen::<u8>()
                }).collect();
                let constants: Constants = Constants::get_constants();
                let mut initial_auth_payload_packer: Vec<u8> = Vec::new();
                rmp::encode::write_u8(&mut initial_auth_payload_packer, constants.protocol_method_handshake_open).expect("Failed to pack");
                rmp::encode::write_bin(&mut initial_auth_payload_packer, &challenge[..]).expect("failed to pack");
                match stream.write_all(&initial_auth_payload_packer[..]).await {
                    Ok(_) => {},
                    Err(err ) => {println!("writing failure: {}", err);return;},
                }
                loop {
                    let ready = stream.ready(Interest::READABLE).await;
                    if ready.is_ok(){
                    let ready_ok = ready.unwrap();
                    if ready_ok.is_readable() {
                        let mut data = vec![0; 1024];
                        // Try to read data, this may still fail with `WouldBlock`
                        // if the readiness event is a false positive.
                        match stream.try_read(&mut data) {
                            Ok(_n) => {
                                let (packed, peer_opt) = P2PService::handle_stream_data(self.clone(), data, peer.clone()).await.unwrap();
                                match stream.write_all(&packed[..]).await {
                                    Ok(_) => {},
                                    Err(err ) => {println!("writing failure: {}", err);return;},
                                }
                                if peer_opt.is_some() {peer = peer_opt.unwrap();}
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
                // TODO implement recconect on dropped connection
                
                // this is the equivalent of the onDone from dart
                if peer.id.is_some() {
                    let _peers: HashMap<String, Peer> = self.peers.get().to_owned();
                }
                
            }            
        } else {
            println!("failed to conenct to: {}", &addr);
            return;
        }}
        pub async fn handle_stream_data(self, data: Vec<u8>, peer: Peer) -> Result<(Vec<u8>, Option<Peer>), Box<dyn Error>> {
            let mut cursor = Cursor::new(data);
            let method: u8 = decode::read_int(&mut cursor).expect("failed to unwrap rmp decode");
            println!("method call: {}", method);
            let constants = Constants::get_constants();
            
            if method == constants.protocol_method_handshake_open {
                // when handshake open is received you send back "handshake_done", the challenge, 
                // the length of conenction uris, and each connection uri as a string
                // in our case we only support 1 connection uri
                let mut packer: Vec<u8> = Vec::new();
                encode::write_u8(&mut packer, constants.protocol_method_handshake_done).expect("packing const failed");
                let challenge_bytes_val = rmpv::decode::read_value(&mut cursor).unwrap().to_owned();
                let challenge_bytes_slice  = challenge_bytes_val.as_slice().expect("expected challege slice to exist");
                encode::write_bin(&mut packer, challenge_bytes_slice).expect("writing challnge failed");
                encode::write_u8(&mut packer, 1).expect("packing const failed"); // this is because only 1 url is supported per peer
                encode::write_str(&mut packer, &format!("tcp://{}",&peer.connection_uris[0])).expect("writing string to packer failed");
                return Ok((packer, None));
            } else if method == constants.protocol_method_registry_update {
                // TODO implement registry
            }
            if method == constants.protocol_methods_signed_message {
                let signed_message: SignedMessage = P2PService::unpack_and_verify_signature(self.clone(), cursor).await.unwrap();
                let mut sm_cursor = Cursor::new(signed_message.message);
                
                let method2: u8 = decode::read_int(&mut sm_cursor).expect("failed to unwrap rmp decode to int");
                if method2 == constants.protocol_method_handshake_done{
                    let challenge_bytes_val = rmpv::decode::read_value(&mut sm_cursor).unwrap().to_owned();
                    let _challenge_bytes_slice  = challenge_bytes_val.as_slice().expect("expected challege slice to exist");
                }
                
            } else if method == constants.protocol_method_hash_query {
                // TODO implement
            } else if method == constants.protocol_method_registry_query {
                // TODO implement
            }
            Ok((Vec::new(), None))
        }
        pub async fn unpack_and_verify_signature(self, message: Cursor<Vec<u8>>) -> Result<SignedMessage, Box<dyn Error>>{
            let mut message_mut: Cursor<Vec<u8>> = message.clone();
            let node_id_val: Value = rmpv::decode::read_value(&mut message_mut).expect("failed to unwrap value");
            let node_id: &[u8] = node_id_val.as_slice().expect("failed to extract slice");
            let node_pk: Vec<u8> = node_id[1..node_id.len()].to_owned();
            let signature_val: Value = rmpv::decode::read_value(&mut message_mut).expect("failed to unwrap value");
            let signature: &[u8] = signature_val.as_slice().expect("failed to extract slice");
            let message_val: Value = rmpv::decode::read_value(&mut message_mut).expect("failed to unwrap value");
            let message: &[u8] = message_val.as_slice().expect("failed to extract slice");
            
            // next verify signature and message
            let pk: PublicKey = ed25519_dalek::PublicKey::from_bytes(&node_pk)?;
            let sig: Signature = Signature::try_from(signature).expect("failed to cast to signature");
            pk.verify(message, &sig)?;
            return Ok(SignedMessage{
                message: message.to_owned(), 
                node_id: P2PService::encode_node_id(node_id.to_owned())
            }); 
        }
        pub fn encode_node_id(node_id_bytes: Vec<u8>) -> String {
            return format!("{}{}", "z".to_owned(), bs58::encode(node_id_bytes).into_string());
        }
        pub fn decode_node_id(node_id_string: String) -> Vec<u8> {
            let chopped: String = (&node_id_string[1..]).to_owned();
            return bs58::decode(chopped.into_bytes()).into_vec().expect("decoding bs58 failed");
            
        }
        pub async fn connect_to_node(self, connection_uris: Vec<String>) {
            // just gonna ignore the list of uris for now
            let peer: String = connection_uris[0].clone();
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
                
                let mut new_peer: Peer = Peer::default();
                new_peer.id = Some(peer_uri.username().to_owned());
                new_peer.connection_uris = vec![peer_uri];
                tokio::task::spawn(P2PService::on_new_peer(self.clone(), new_peer));
            }
        }
    }
}
    