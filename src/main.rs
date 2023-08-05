use std::net::{Ipv4Addr, UdpSocket};

use clap::Parser;
use mac_address::MacAddress;
use miette::Result;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
/// Simple tool to send Wake-on-LAN packets
struct WolArgs {
    #[arg(long, short, required = true)]
    /// MAC address of the machine you want to send WoL packets to
    mac_address: MacAddress,

    #[arg(long, short, default_value = "255.255.255.255")]
    /// The broadcast IP address
    ip_address: Ipv4Addr,
}

fn main() -> Result<()> {
    let args = WolArgs::parse();
    run_program(args.mac_address, args.ip_address)?;
    Ok(())
}

fn run_program(mac_address: MacAddress, ip_address: Ipv4Addr) -> Result<()> {
    let mac_address = mac_address.bytes();
    let wol_sock = UdpSocket::bind("0.0.0.0:0").expect("Cannot bind UDP socket!");
    wol_sock.set_broadcast(true).expect("Cannot broadcast!");

    let mut magic_packet = vec![255u8, 255u8, 255u8, 255u8, 255u8, 255u8];
    magic_packet.append(&mut mac_address.repeat(16));
    wol_sock
        .send_to(&magic_packet, (ip_address, 9))
        .expect("Cannot send Wake-on-LAN packet!");
    Ok(())
}
