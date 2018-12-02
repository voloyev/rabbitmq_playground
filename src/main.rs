#[macro_use]
extern crate log;
extern crate futures;
extern crate lapin_futures as lapin;
extern crate tokio;

use futures::future::Future;
use futures::Stream;
use lapin::channel::{BasicProperties, BasicPublishOptions, QueueDeclareOptions};
use lapin::client::ConnectionOptions;
use lapin::types::FieldTable;
use tokio::net::TcpStream;
use tokio::runtime::Runtime;

fn main() {
    let addr = "127.0.0.1:5672".parse().unwrap();

    Runtime::new()
        .unwrap()
        .block_on(
            TcpStream::connect(&addr)
                .and_then(|stream| {
                    // connect() returns a future of an AMQP Client
                    // that resolves once the handshake is done
                    lapin::client::Client::connect(stream, ConnectionOptions::default())
                })
                .and_then(|(client, _ /* heartbeat */)| {
                    // create_channel returns a future that is resolved
                    // once the channel is successfully created
                    client.create_channel()
                })
                .and_then(|channel| {
                    let id = channel.id;
                    info!("created channel with id: {}", id);

                    // we using a "move" closure to reuse the channel
                    // once the queue is declared. We could also clone
                    // the channel
                    channel
                        .queue_declare("hello", QueueDeclareOptions::default(), FieldTable::new())
                        .and_then(move |_| {
                            info!("channel {} declared queue {}", id, "hello");

                            channel.basic_publish(
                                "",
                                "hello",
                                b"hello from tokio".to_vec(),
                                BasicPublishOptions::default(),
                                BasicProperties::default(),
                            )
                        })
                }),
        )
        .expect("runtime failure");
}
