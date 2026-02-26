use core::{str::from_utf8, time::Duration};

use arduino_hal::{pac::SPI, Spi};
use w5500_mqtt::{
    hl::Hostname, ll::{
        eh1::{self, fdm::W5500}, net::{Ipv4Addr, SocketAddrV4}, Registers, Sn
    }, Event
};

use crate::{network::w5500, println};
use embedded_hal::spi;

//This is supposed to be the mqtt server it connects to, not the IP for the device!
const HOST: SocketAddrV4 = SocketAddrV4::new(Ipv4Addr::new(10, 10, 10, 199), 8883);

struct CounterClock
{
    count:u32
}

impl CounterClock
{
    pub fn new()->CounterClock
    {
        CounterClock{count:0}
    }

    pub fn tick(&mut self)->u32
    {
        self.count+=1;
        self.count
    }
}

pub struct InstantiatedW5500<'a>
{
    clock:CounterClock,
    w5500:W5500<arduino_hal::spi::Spi>,
    client:w5500_mqtt::Client<'a>
}

impl <'a> InstantiatedW5500<'a>
{
    pub fn process<'b>(&mut self)->Result<(),&'b str>
    {
        let result = match self.client.process(&mut self.w5500, self.clock.tick()) {
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
                    (Ok(topic), Ok(payload)) => println!("{} {}",topic,payload),
                    _ => println!("payload and topic are not valid UTF-8"),
                }

                reader.done().unwrap();
            }
            // This does not handle failures
            Ok(Event::SubAck(_ack)) => println!("Suback"),
            // should never occur - we never unsubscribe
            Ok(Event::UnSubAck(_ack)) => println!("Unsuback"),
            Ok(Event::ConnAck) => {
                self.client
                    .subscribe(&mut self.w5500, "#")
                    .expect("failed to send SUBSCRIBE");
            }
            Ok(Event::None) => (),
            Err(e) => panic!("Error occured: {e:?}"),
        };

        Ok(result)
    }

    pub fn new<'b>(spi:Spi)->Result<InstantiatedW5500<'a>,&'b str>
    {
        let mut client = w5500_mqtt::Client::new(
            Sn::Sn2, //This is what's in the example...
            w5500_mqtt::SRC_PORT,
            HOST
        );

        let id_str = "SecurityController"; //letters and numbers ony
        let id = match w5500_mqtt::ClientId::new(id_str)
        {
            None=>{
                return Err("Couldn't create w5500_mqtt client id.");
            },
            Some(id)=>id
        };

        client.set_client_id(id);

        
        let w5500= W5500::new(spi);
        let clock=CounterClock::new();

        Ok(InstantiatedW5500
        {
            clock,
            w5500,
            client
        })
    }

}