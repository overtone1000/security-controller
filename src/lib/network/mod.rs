use arduino_hal::hal;
use w5500::MacAddress;

use smoltcp_nal::smoltcp;
use smoltcp::iface::Interface;
use smoltcp::wire::{EthernetAddress, IpCidr, Ipv4Address, Ipv4Cidr};
use smoltcp::socket::dhcpv4;

//Trying to emulate https://github.com/cnmozzie/stm32-rust-demo/blob/main/examples/smoltcp-dhcp.rs
//There's a local copy in the example directory in this repo
pub fn test()
{
    let spi = {
        hal::spi::Spi::new(

        )
    };
    
    let w5500 = w5500::UninitializedDevice::new(
            w5500::bus::FourWire::new(spi)
        )
        .initialize_macraw(MacAddress::new(0, 1, 2, 3, 4, 5))
        .unwrap();
        
    let mut mac = Mac::W5500(w5500);
}