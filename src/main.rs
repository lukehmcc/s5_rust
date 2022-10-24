use std::{env, fs, process::exit};
use rand::{thread_rng, Rng};
use base58::{self, ToBase58};

mod modules;
use modules::{Config,Node};

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2{
        println!("Please specify a config file for this node.");
        exit(1);
    }
    println!("");
    println!("s5_rust v{}", env!("CARGO_PKG_VERSION"));
    println!("");
    let mut config = fs::read_to_string(&args[1]).expect("Failed to read config file");
    
    if config.contains("AUTOMATICALLY_GENERATED_ON_FIRST_START"){
        println!("Generating seed...");
        let mut rng = thread_rng();
        let numbers: Vec<u8> = (0..64).map(|_| {
            rng.gen()
        }).collect();
        config = config.replace("AUTOMATICALLY_GENERATED_ON_FIRST_START", &numbers.to_base58());
        fs::write(&args[1], config).expect("Failed to write seed to config");
        println!("Sucsessfully generated and inserted seed.")
    }
    let config = fs::read_to_string(&args[1]).expect("Failed to read config file");
    let config_toml: Config = toml::from_str(&config).unwrap();
    let node: Node = Node{
        config: config_toml,
    };
    node.start_node().await;
}