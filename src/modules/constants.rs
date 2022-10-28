pub mod constants {
    pub struct Constants{
        pub nodeversion: String,
        pub default_chunk: i32,
        // These bytes are carefully selected to make the base58 and base32 representations of different CID types
        // easy to distinguish and not collide with anything on https://github.com/multiformats/multicodec
        pub cid_type_raw: i32,
        pub cid_type_metadata_file: i32,
        pub cid_type_metadata_dir: i32,
        pub cid_type_resolver: i32,
        
        pub registry_s5_magic_byte: i32,
        pub metadata_magic_byte: i32,
        
        // types for metadata files
        pub metadata_type_file: i32,
        pub metadata_type_chunked_file: i32,
        pub metadata_type_directory: i32,
        
        pub registry_max_data_size: i32,
        
        pub mhash_blake3: [i32;2],
        
        pub mkey_ed25519: u8,
        
        //  Use this for protocol updates
        pub protocol_method_handshake_open: i32,
        pub protocol_method_handshake_done: i32,
        pub protocol_methods_signed_message: i32,
        pub protocol_method_hash_query_response: i32,
        pub protocol_method_hash_query: i32,
        pub protocol_method_annouce_peers: i32,
        pub protocol_method_registry_update: i32,
        pub protocol_method_registry_query: i32,
    }
    impl Constants{
        pub fn get_constants() -> Constants {
            return Constants{
                nodeversion: "0.3.0".to_owned(),
                default_chunk: 1024 * 1024,
                cid_type_raw: 0x26,
                cid_type_metadata_file: 0x2d,
                cid_type_metadata_dir: 0x59,
                cid_type_resolver: 0x25,
                registry_s5_magic_byte: 0x5a,
                metadata_magic_byte: 0x5f,
                metadata_type_file: 0x01,
                metadata_type_chunked_file: 0x02,
                metadata_type_directory: 0x03,
                registry_max_data_size: 48,
                mhash_blake3: [0x1e, 0x20],
                mkey_ed25519: 0xed,
                protocol_method_handshake_open: 1,
                protocol_method_handshake_done: 2,
                protocol_methods_signed_message: 10,
                protocol_method_hash_query_response: 5,
                protocol_method_hash_query: 4,
                protocol_method_annouce_peers: 7,
                protocol_method_registry_update: 12,
                protocol_method_registry_query: 13,
            }
        }
    }
}