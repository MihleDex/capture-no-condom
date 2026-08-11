use raw_socket::{Domain, Protocol, RawSocket, Type};

struct IPV4Header {
    version:u8, //Convert to nibble before storing
    ihl:u8,
    dscp:u8,
    ecn:u8,
    total_length:u16
}

fn build_header(version:u8,ihl:u8,dscp:u8,ecn:u8,total_length:u16) -> IPV4Header {
    IPV4Header { version, ihl, dscp, ecn, total_length }
}



fn main() {
    //Create socket
    let socket = RawSocket::new(Domain::ipv4(), Type::raw(), Some(Protocol::tcp()));
    
    let raw_socket = match socket {
        Ok(result) => result,
        Err(error) => {
            panic!("Error Creating socket: {}",error);
        }
    };
    println!("Socket Created ");
    
    let mut buffer:[u8;58] = [0;58];
 

    //Recieve data from Socket
    let data = raw_socket.recv_from(&mut buffer);

    let (_bytes_recvd,_src_addr) = match data {
    Ok((bytes,src_addr)) => (bytes,src_addr),
    Err(error) => {panic!("Error: {}",error)}
    };


    let tl1 :u16 = buffer[4].into();
    let tl2 :u16 = buffer[5].into();
    let tl :u16 = tl1 + tl2;
    let header =  build_header(buffer[0] >> 4, buffer[0] & 0x0F, buffer[2], buffer[3],tl);

    println!("------IPV4Header-------");
    println!("version: {:02x?}",header.version);
    println!("ihl: {:02x?}",header.ihl);
    println!("dscp: {:02x?}",header.dscp);
    println!("ecn: {:02x?}",header.ecn);
    println!("total_length: {:02x?}",header.total_length);
   
    
}
