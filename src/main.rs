use raw_socket::{Domain, Protocol, RawSocket, Type};
use std::net::{SocketAddr};


fn main() {
    //Create socket
    let socket = RawSocket::new(Domain::ipv4(), Type::stream(), Some(Protocol::tcp()));
    
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

    println!("Socket bound to: {}",addr)
    
}
