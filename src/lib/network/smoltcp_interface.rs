use smoltcp_nal::smoltcp;

pub enum NetworkDevice
{
    W5500(w5500::raw_device::RawDevice
        <
            w5500::bus::FourWire
                <
                    embedded_hal_bus::spi::ExclusiveDevice
                    <
                        arduino_hal::hal::Spi,
                        arduino_hal::port::Pin<arduino_hal::port::mode::Output, arduino_hal::hal::port::PB2>,
                        arduino_hal::hal::delay::Delay<arduino_hal::clock::MHz16>
                    >
                >
        >
    ),
}

impl smoltcp::phy::Device for NetworkDevice {
    type RxToken<'a> = RxToken where Self: 'a;
    type TxToken<'a> = TxToken<'a> where Self: 'a;

    fn capabilities(&self) -> smoltcp::phy::DeviceCapabilities {
        let mut caps = smoltcp::phy::DeviceCapabilities::default();
        caps.max_transmission_unit = 1500;
        caps.medium = smoltcp::phy::Medium::Ethernet;
        caps
    }

    fn receive(
        &mut self,
        _timestamp: smoltcp::time::Instant,
    ) -> Option<(Self::RxToken<'_>, Self::TxToken<'_>)> {
        let mut buffer = [0u8; 1500];
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
    frame_buffer: [u8; 1500],
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

pub struct TxToken<'a> {
    mac: &'a mut NetworkDevice,
}

impl<'a> smoltcp::phy::TxToken for TxToken<'a> {
    fn consume<R, F>(self, len: usize, f: F) -> R
    where
        F: FnOnce(&mut [u8]) -> R,
    {
        let mut buffer = [0u8; 1500];
        let result = f(&mut buffer[..len]);
        match self.mac {
            NetworkDevice::W5500(mac) => {
                mac.write_frame(&buffer[..len]).unwrap();
            }
        }

        result
    }
}