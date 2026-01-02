use w5500_mqtt::{
    hl::Hostname, ll::{
        net::{Ipv4Addr, SocketAddrV4}, Registers, Sn
    }, Event
};

use crate::network::w5500;

const HOST: SocketAddrV4 = SocketAddrV4::new(Ipv4Addr::new(10, 0, 0, 4), 8883);

pub fn test()
{
    let port:u16;

    let client = w5500_mqtt::Client::new(
        Sn::Sn2, //This is what's in the example...
        w5500_mqtt::SRC_PORT,
        HOST
    );

    //TODO
    //w5500_mqtt::Client is well documented, maybe start there.

    loop {
        match client.process(&mut w5500, monotonic_secs(start)) {
            Ok(Event::CallAfter(_)) => (),
            Ok(Event::Publish(mut reader)) => {
                let mut payload_buf: [u8; 128] = [0; 128];
                let payload_len: u16 = reader
                    .read_payload(&mut payload_buf)
                    .expect("failed to read payload");
                let mut topic_buf: [u8; 128] = [0; 128];
                let topic_len: u16 = reader
                    .read_topic(&mut topic_buf)
                    .expect("failed to read payload");

                match (
                    from_utf8(&topic_buf[..topic_len.into()]),
                    from_utf8(&payload_buf[..payload_len.into()]),
                ) {
                    (Ok(topic), Ok(payload)) => log::info!("{topic} {payload}"),
                    _ => log::info!("payload and topic are not valid UTF-8"),
                }

                reader.done().unwrap();
            }
            // This does not handle failures
            Ok(Event::SubAck(ack)) => log::info!("{ack:?}"),
            // should never occur - we never unsubscribe
            Ok(Event::UnSubAck(ack)) => log::warn!("{ack:?}"),
            Ok(Event::ConnAck) => {
                client
                    .subscribe(&mut w5500, "#")
                    .expect("failed to send SUBSCRIBE");
            }
            Ok(Event::None) => sleep(Duration::from_millis(10)),
            Err(e) => panic!("Error occured: {e:?}"),
        }
    }
}
