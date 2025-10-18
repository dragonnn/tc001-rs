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
            rx_buffer: Box::new([0; 2048]),
            tx_meta: Box::new([PacketMetadata::EMPTY; 16]),
            tx_buffer: Box::new([0; 2048]),
        }
    }

    pub fn as_static_mut(
        &mut self,
    ) -> (&'static mut [PacketMetadata], &'static mut [u8], &'static mut [PacketMetadata], &'static mut [u8]) {
        unsafe {
            core::mem::transmute((
                self.rx_meta.as_mut(),
                self.rx_buffer.as_mut(),
                self.tx_meta.as_mut(),
                self.tx_buffer.as_mut(),
            ))
        }
    }
}
