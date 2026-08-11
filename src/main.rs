use std::net::Ipv4Addr;

use raw_socket::{Domain, Protocol, RawSocket, Type};

struct IPV4Header {
    version:u8, //Convert to nibble before storing
    ihl:u8,
    dscp:u8,
    ecn:u8,
    total_length:u16,
    identification:u16,
    flags: u8,
    fragment_offset:u16,
    time_to_live: u8,
    protocol:u8,
    header_checksum:u16,
    source_address: Ipv4Addr,
    destination_address:Ipv4Addr
}

fn build_header(buffer:[u8;58]) -> IPV4Header {
    let version = buffer[0] >> 4; //nibling, lol
    let ihl = buffer[0] & 0x0F; //Mask with hexadecimal of 15 to get remaing bits
    let dscp = buffer[1] >> 6;
    let ecn = buffer[1] & 3; //Mask with 3
    let total_length = u16::from_be_bytes([buffer[2],buffer[3]]);
    let identification = u16::from_be_bytes([buffer[4],buffer[5]]);
    let flags = buffer[6] >> 3;
    let fragment_offset = u16::from_be_bytes([buffer[6] & 0x1F,buffer[7]]);
    let time_to_live = buffer[8];
    let protocol = buffer[9];
    let header_checksum = u16::from_be_bytes([buffer[10],buffer[11]]);
    let source_address = Ipv4Addr::from([buffer[12],buffer[13],buffer[14],buffer[15]]);
    let destination_address = Ipv4Addr::from([buffer[16],buffer[17],buffer[18],buffer[19]]);
    IPV4Header { version, ihl, dscp, ecn, total_length,identification, flags,fragment_offset,time_to_live,protocol,header_checksum,source_address,destination_address }
}

fn main() {
    //Create socket
    let socket: Result<RawSocket, std::io::Error> = RawSocket::new(Domain::ipv4(), Type::raw(), Some(Protocol::tcp()));
    let mut buffer:[u8;58] = [0;58];

    //Extract socket creation results
    let raw_socket: RawSocket = match socket {
        Ok(result) => result,
        Err(error) => {
            panic!("Error Creating socket: {}",error);
        }
    };
    println!("Socket Created ");

    //Recieve data from Socket
    let data: Result<(usize, std::net::SocketAddr), std::io::Error> = raw_socket.recv_from(&mut buffer);
    //Extract results
    let (_bytes_recvd,_src_addr) = match data {
    Ok((bytes,src_addr)) => (bytes,src_addr),
    Err(error) => {panic!("Error: {}",error)}
    };
    //Build Ipv header
    let header =  build_header(buffer);
    //Print Header
    println!("------IPV4Header-------");
    println!("version: {:02x?}",header.version);
    println!("ihl: {:02x?}",header.ihl);
    println!("dscp: {:02x?}",header.dscp);
    println!("ecn: {:02x?}",header.ecn);
    println!("total_length: {:02x?}",header.total_length);
    println!("identification: {:02x?}",header.identification);
    println!("flags: {:02x?}",header.flags);
    println!("fragment_offset: {:02x?}",header.fragment_offset);
    println!("time_to_live: {}",header.time_to_live);
    println!("protocol: {:02x?}",header.protocol);
    println!("header_checksum: {:02x?}",header.header_checksum);
    println!("source_address: {:02x?}",header.source_address);
    println!("destination_address: {:02x?}",header.destination_address);

}
