use std::{
    io,
    net::{SocketAddr, ToSocketAddrs},
};

pub mod stream;
pub mod target_addr;

pub async fn lookup_host<T>(host: T) -> io::Result<impl Iterator<Item = SocketAddr>>
where
    T: ToSocketAddrs,
{
    // addr::to_socket_addrs(host).await
    host.to_socket_addrs()
}
