extern crate bitcoincore_rpc;
#[macro_use]
extern crate serde_derive;
extern crate toml;

use std::env;
use std::error::Error;
use std::fs;
use std::io::Read;
use std::process;

use bitcoincore_rpc::{Auth, Client, RpcApi};
use prometheus_exporter_base::{render_prometheus, MetricType, PrometheusMetric};

#[derive(Debug, Deserialize, Clone)]
struct Node {
    name: String,
    chaintype: String,
    test: bool,
    hostport: String,
    username: String,
    password: String,
}

#[derive(Debug, Deserialize, Clone)]
struct Config {
    node: Vec<Node>,
}

#[derive(Clone, Debug, Default)]
struct MyOptions {}

fn get_config(filename: String) -> Result<Config, Box<dyn Error>> {
    let mut file_path: std::path::PathBuf = std::path::PathBuf::new();
    file_path.push(std::env::current_dir().unwrap().as_path());
    file_path.push(filename);

    let mut configuration_file: fs::File = match fs::OpenOptions::new()
        .read(true)
        .open(file_path.as_path()) {
            Ok(configuration_file) => configuration_file,
            Err(error) => {
                return Err(Box::new(error))
            }
        };

    let mut contents = String::new();

    match configuration_file.read_to_string(&mut contents) {
        Ok(_) => {},
        Err(error) => {
            return Err(Box::new(error))
        }
    }

    let config: Config = match toml::from_str(&contents) {
        Ok(config) => config,
        Err(error) => {
            return Err(Box::new(error))
        }
    };

    return Ok(config);
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let mut filename : String = "bc-exporter.toml".to_string();

    if args.len() > 1 {
        filename = args[1].to_string();
    }

    let config = match get_config(filename) {
        Ok(config) => config,
        Err(error) => {
            eprintln!("Could not load configuration: {}", error);
            process::exit(1)
        }
    };

    let addr = ([0, 0, 0, 0], 32221).into();
    println!("starting exporter on {}", addr);

    render_prometheus(addr, MyOptions::default(), |_request, _options| async move {
        let pc = PrometheusMetric::new("block_count", MetricType::Counter, "Block Count");
        let mut s = pc.render_header();

        for node in &config.node {
            let user_pass = Auth::UserPass(node.username.to_string(), node.password.to_string());

            let rpc = match Client::new(node.hostport.to_string(), user_pass) {
                Ok(rpc) => rpc,
                Err(error) => {
                    eprintln!("Failed while doing {}: {}", node.name, error);
                    continue
                }
            };

            let block_count = match rpc.get_block_count() {
                Ok(block_count) => block_count,
                Err(error) => {
                    eprintln!("Failed while doing {}: {}", node.name, error);
                    continue
                }
            };

            let mut attributes = Vec::new();
            attributes.push(("name", node.name.as_str()));
            attributes.push(("node", node.hostport.as_str()));
            attributes.push(("chaintype", node.chaintype.as_str()));
            match node.test {
                true => attributes.push(("test", "true")),
                false => attributes.push(("test", "false")),
            }

            s.push_str(&pc.render_sample(Some(&attributes), block_count, None));
        }

        Ok(s)
    })
    .await;
}
