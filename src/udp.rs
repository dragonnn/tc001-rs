use alloc::boxed::Box;

use embassy_net::{
    dns::DnsQueryType,
    udp::{PacketMetadata, UdpSocket},
    IpEndpoint,
};

pub struct UdpBuffers {
    rx_meta: Box<[PacketMetadata]>,
    rx_buffer: Box<[u8]>,
    tx_meta: Box<[PacketMetadata]>,
    tx_buffer: Box<[u8]>,
}

impl UdpBuffers {
    pub fn new() -> Self {
        Self {
            rx_meta: Box::new([PacketMetadata::EMPTY; 16]),
            rx_buffer: Box::new([0; 128]),
            tx_meta: Box::new([PacketMetadata::EMPTY; 16]),
            tx_buffer: Box::new([0; 128]),
        }
    }

    pub fn as_mut<'a>(
        &'a mut self,
    ) -> (&'a mut [PacketMetadata], &'a mut [u8], &'a mut [PacketMetadata], &'a mut [u8]) {
        (self.rx_meta.as_mut(), self.rx_buffer.as_mut(), self.tx_meta.as_mut(), self.tx_buffer.as_mut())
    }
}
