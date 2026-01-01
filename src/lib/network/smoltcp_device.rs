use smoltcp_nal::smoltcp;
use w5500::bus::Bus;
use w5500::raw_device::RawDevice;

const TRANSMISSION:usize=1500;

pub enum NetworkDevice<T:Bus>
{
    W5500(RawDevice<T>),
}

impl <T:Bus> smoltcp::phy::Device for NetworkDevice<T> {
    type RxToken<'a> = RxToken where Self: 'a;
    type TxToken<'a> = TxToken<'a, T> where Self: 'a;

    fn capabilities(&self) -> smoltcp::phy::DeviceCapabilities {
        let mut caps = smoltcp::phy::DeviceCapabilities::default();
        crate::println!("This max transmission unit may be incorrect.");
        caps.max_transmission_unit = TRANSMISSION;
        caps.medium = smoltcp::phy::Medium::Ethernet;

        caps
    }

    fn receive(
        &mut self,
        _timestamp: smoltcp::time::Instant,
    ) -> Option<(Self::RxToken<'_>, Self::TxToken<'_>)> {
        let mut buffer = [0u8; TRANSMISSION];
        let len = match self {
            NetworkDevice::W5500(w5500) => w5500.read_frame(&mut buffer[..]).unwrap(),
        };

        if len != 0 {
            Some((
                RxToken {
                    frame_buffer: buffer,
                    length: len,
                },
                TxToken { mac: self },
            ))
        } else {
            None
        }
    }

    fn transmit(&mut self, _timestamp: smoltcp::time::Instant) -> Option<Self::TxToken<'_>> {
        Some(TxToken { mac: self })
    }
}

pub struct RxToken {
    frame_buffer: [u8; TRANSMISSION],
    length: usize,
}

impl smoltcp::phy::RxToken for RxToken {
    fn consume<R, F>(self, f: F) -> R
        where
        F: FnOnce(&[u8]) -> R
    {
        f(&self.frame_buffer[..self.length])
    }
}

pub struct TxToken<'a, T:Bus> {
    mac: &'a mut NetworkDevice<T>,
}

impl<'a,T:Bus> smoltcp::phy::TxToken for TxToken<'a, T> {
    fn consume<R, F>(self, len: usize, f: F) -> R
    where
        F: FnOnce(&mut [u8]) -> R,
    {
        let mut buffer = [0u8; TRANSMISSION];
        let result = f(&mut buffer[..len]);
        match self.mac {
            NetworkDevice::W5500(mac) => {
                mac.write_frame(&buffer[..len]).unwrap();
            }
        }

        result
    }
}