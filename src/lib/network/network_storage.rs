use smoltcp_nal::smoltcp;

/// The number of TCP sockets supported in the network stack.
const NUM_TCP_SOCKETS: usize = 1; //Probably only need one for this device to communicate with an mqtt server
const RX_STORAGE:usize = 1024; //Not sure how much is needed for mqtt client
const TX_STORAGE:usize = 4096; //Not sure how much is needed for mqtt client

/// Containers for smoltcp-related network configurations
/// 

pub struct NetStorage {
    // Note: There is an additional socket set item required for the DHCP socket.
    pub(crate) sockets: [smoltcp::iface::SocketStorage<'static>; NUM_TCP_SOCKETS + 1],
    pub(crate) tcp_socket_storage: [TcpSocketStorage; NUM_TCP_SOCKETS],
}

impl NetStorage {
    pub const fn new() -> Self {
        NetStorage {
            sockets: [smoltcp::iface::SocketStorage::EMPTY; NUM_TCP_SOCKETS + 1],
            tcp_socket_storage: [TcpSocketStorage::new(); NUM_TCP_SOCKETS],
        }
    }
}

#[derive(Copy, Clone)]
pub(crate) struct TcpSocketStorage {
    pub(crate) rx_storage: [u8; RX_STORAGE],
    pub(crate) tx_storage: [u8; TX_STORAGE],
}

impl TcpSocketStorage {
    const fn new() -> Self {
        Self {
            tx_storage: [0; TX_STORAGE],
            rx_storage: [0; RX_STORAGE],
        }
    }
}