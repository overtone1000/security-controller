use arduino_hal::hal::delay::Delay;
use arduino_hal::hal::port::{PB2, PB3, PB4, PB5};
use arduino_hal::hal::{delay};
use arduino_hal::port::mode::{Input, Output, PullUp};
use arduino_hal::port::Pin;
use arduino_hal::spi::{self, DataOrder, SerialClockRate};

use embedded_hal_bus::spi::ExclusiveDevice;

use smoltcp_nal::smoltcp;
use smoltcp_nal::smoltcp::iface::SocketHandle;
use smoltcp_nal::smoltcp::socket::dhcpv4;
use smoltcp_nal::smoltcp::wire::{HardwareAddress, IpCidr, Ipv4Cidr};
use smoltcp::wire::Ipv4Address;

use w5500::MacAddress;

use crate::network::network_storage::NetStorage;
use crate::network::smoltcp_device::NetworkDevice;
use crate::println;

//Trying to emulate https://github.com/cnmozzie/stm32-rust-demo/blob/main/examples/smoltcp-dhcp.rs
//There's a local copy in the example directory in this repo


type RawDeviceType = 
    w5500::raw_device::RawDevice<
        w5500::bus::FourWire<
            ExclusiveDevice<
                spi::Spi,
                spi::ChipSelectPin<arduino_hal::hal::port::PB2>,
                Delay<arduino_hal::clock::MHz16>
            >
        >
    >
;

type BusDeviceType = w5500::bus::FourWire<
            ExclusiveDevice<
                spi::Spi,
                spi::ChipSelectPin<arduino_hal::hal::port::PB2>,
                Delay<arduino_hal::clock::MHz16>
            >
        >;


struct CounterClock
{
    loopcount:i64
}
impl CounterClock
{
    fn get_new_loop_timestamp(&mut self)->smoltcp::time::Instant
    {
        self.loopcount+=1;
        smoltcp::time::Instant::from_millis(self.loopcount)
    }
}

pub struct W5500Interface<'a>
{
    network_device:NetworkDevice<BusDeviceType>,
    network_interface:smoltcp::iface::Interface,
    //storage:&'static NetStorage,
    //storage:NetStorage<'a>,
    sockets:smoltcp::iface::SocketSet<'a>,
    tcp_handle:SocketHandle,
    dhcp_handle:SocketHandle,
    clock:CounterClock   
}

impl <'a> W5500Interface<'a> {

    pub fn new(
        spi_peripheral:arduino_hal::pac::SPI,
        cs: Pin<Output, PB2>,
        copi: Pin<Output, PB3>,
        cipo: Pin<Input<PullUp>, PB4>,
        sclk: Pin<Output, PB5>,
        storage:&'a mut NetStorage<'a>
    )->W5500Interface<'a>
    {
        println!("This data order and serial clock rate may be incorrect.");
        let settings = spi::Settings{
            data_order: DataOrder::LeastSignificantFirst,
            clock: SerialClockRate::OscfOver16,
            mode: embedded_hal::spi::Mode {
                polarity: embedded_hal::spi::Polarity::IdleLow,
                phase: embedded_hal::spi::Phase::CaptureOnFirstTransition,
            }
        };

        let (spi,cs) = {
            spi::Spi::new(
                spi_peripheral,
                sclk,
                copi,
                cipo,
                cs,
                settings
            )
        };

        let arduino_delay:Delay<arduino_hal::clock::MHz16>=delay::Delay::new();
        let exclusive_spi=ExclusiveDevice::new(spi,cs,arduino_delay);
        let fourwire = w5500::bus::FourWire::new(exclusive_spi);
        let uninitialized = w5500::UninitializedDevice::new(fourwire);

        // Recommended address spaces
        //  x2-xx-xx-xx-xx-xx
        //  x6-xx-xx-xx-xx-xx
        //  xA-xx-xx-xx-xx-xx
        //  xE-xx-xx-xx-xx-xx
        let mac= MacAddress::new(
            2,0, 1,2,3,5
        );
        
        let initialized = match uninitialized.initialize_macraw(mac)
        {
            Ok(res) => res,
            Err(_) => panic!(),
        };

        let mut network_device = NetworkDevice::W5500(initialized);
        let config = smoltcp::iface::Config::new(
            HardwareAddress::Ethernet(
                smoltcp::wire::EthernetAddress(
                    mac.octets)
            )
        );

        println!("Is ZERO correct here?");
        let mut network_interface = smoltcp::iface::Interface::new(config, &mut network_device, smoltcp::time::Instant::ZERO);

        println!("Default to 0.0.0.0");
        network_interface
            .routes_mut()
            .add_default_ipv4_route(Ipv4Address::new(0, 0, 0, 0))
            .unwrap();

        network_interface.update_ip_addrs(|addrs| addrs.push(IpCidr::new(smoltcp::wire::IpAddress::v4(0, 0, 0, 0), 0)).unwrap());

        // Create sockets
        let dhcp_socket = dhcpv4::Socket::new();

        //let mut storage = NetStorage::new();
        //let net_store = cortex_m::singleton!(: NetStorage = NetStorage::new()).unwrap();
        
        let mut sockets = smoltcp::iface::SocketSet::new(storage.sockets.as_mut_slice() );
        
        let tcp_socket = {
        let rx_buffer = smoltcp::socket::tcp::SocketBuffer::new( storage.tcp_socket_storage[0].rx_storage.as_mut_slice());
        let tx_buffer = smoltcp::socket::tcp::SocketBuffer::new( storage.tcp_socket_storage[0].tx_storage.as_mut_slice());

            smoltcp::socket::tcp::Socket::new(rx_buffer, tx_buffer)
        };

        let tcp_handle = sockets.add(tcp_socket);
        let dhcp_handle = sockets.add(dhcp_socket);

        let clock = CounterClock { loopcount: 0 };

        let retval = W5500Interface { 
            network_device,
            network_interface, 
            //storage,
            sockets,
            tcp_handle, 
            dhcp_handle, 
            clock 
        };

        retval
    }

    pub fn process_sockets(&mut self)
    {
        
        let timestamp = self.clock.get_new_loop_timestamp();

        

        self.network_interface.poll(
            timestamp,
            &mut self.network_device,
            &mut self.sockets
        );

/*        

        let event= self.sockets.get_mut::<dhcpv4::Socket>(self.dhcp_handle).poll();

        match event
        {
            None => {}
            Some(dhcpv4::Event::Configured(config)) => {
                
                println!("DHCP config acquired!");
                //println!("IP address is {}", config.address); //doesn't implement display
                set_ipv4_addr(&mut self.network_interface, config.address);

                if let Some(router) = config.router {
                    //println!("Default gateway: {}", router); //doesn't implement display
                    self.network_interface.routes_mut().add_default_ipv4_route(router).unwrap();
                } else {
                    println!("Default gateway: None");
                    self.network_interface.routes_mut().remove_default_ipv4_route();
                }
            }
            Some(dhcpv4::Event::Deconfigured) => {
                println!("DHCP lost config!");
                set_ipv4_addr(&mut self.network_interface,Ipv4Cidr::new(Ipv4Address::UNSPECIFIED, 0));
                self.network_interface.routes_mut().remove_default_ipv4_route();
            }
        }
        */
    }
}

fn set_ipv4_addr(network_interface:&mut smoltcp::iface::Interface, cidr: Ipv4Cidr) {
    network_interface.update_ip_addrs(
        |addrs| {
            let dest = addrs.iter_mut().next().unwrap();
            *dest = IpCidr::Ipv4(cidr);
        }
    );
}