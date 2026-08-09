use raw_socket::{Domain, Protocol, RawSocket, Type};
use std::net::{SocketAddr};


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
    
    //Make socket bind to localhost
    let addr = SocketAddr::from(([127, 0, 0, 1], 7878));
    let listener = raw_socket.bind(addr);
    println!("Binding socket to {} ",addr);

    let _binding_result = match listener {
        Ok(b_result) => b_result,
        Err(error) => {
            panic!("Socket binding error: {}",error);
        }
    };

    println!("Socket bound to: {}",addr);

    let mut buffer:[u8;8] = [0,0,0,0,0,0,0,0];

    //Recieve data from Socket
    let data = raw_socket.recv_from(&mut buffer);

    let (bytes_recvd,src_addr) = match data {
        Ok((bytes,src_addr)) => (bytes,src_addr),
        Err(error) => {panic!("Error: {}",error)}
    };

    println!("Recieved: {} from: {}",bytes_recvd,src_addr);
    
}
