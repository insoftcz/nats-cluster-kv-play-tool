use async_nats::jetstream::stream::No;
use clap::{Parser, ValueEnum};
use serde;
use serde::Serialize;
use bytes::{Bytes};
use futures::StreamExt;


fn cluster_port_map(cluster_name: &ClusterName) -> u64 {
    match cluster_name {
        ClusterName::A => 4222,
        ClusterName::B => 4223,
        ClusterName::C => 4224,
    }
}

#[derive(ValueEnum, Clone, Default, Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
enum ClusterName {
    #[default]
    A,
    B,
    C,
}

#[derive(ValueEnum, Clone, Default, Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
enum Cmd {
    /// reads value of "foo" key in bucket "kv"
    #[default]
    Read,
    /// writes some data to key "foo" in bucket "kv"
    Write,
    /// listens for changes of "foo" in bucket "kv"
    ReadSubscribe,
}

/// NATS cluster POC tool
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t, value_enum)]
    cluster_name: ClusterName,

    #[arg(long, default_value_t, value_enum)]
    cmd: Cmd,
}


#[tokio::main]
async fn main() {
    let args = Args::parse();

    let port = cluster_port_map(&args.cluster_name);


    let nc = async_nats::connect(format!("nats://localhost:{}", port)).await.expect("to connect");
    let js = async_nats::jetstream::new(nc);

    let kv = js.create_key_value(async_nats::jetstream::kv::Config {
        bucket: "kv".to_string(),
        ..Default::default()
    }).await.expect("to create bucket");

    match args.cmd {
        Cmd::Read => {
            let result = kv.get("foo").await;
            println!("{:?}", result);
        },
        Cmd::Write => {
            let result = kv.put("foo", Bytes::from("hello world")).await;
            println!("{:?}", result);
        }
        Cmd::ReadSubscribe => {
            let mut watch = kv.watch("foo").await.expect("to watch foo");
            loop {
                let update = watch.next().await;
                if update.is_none() {
                    break;
                }
                println!("{:?}", update)
            }
        }
    }
}