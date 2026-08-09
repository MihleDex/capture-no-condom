use raw_socket::{Domain, Protocol, RawSocket, Type};



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
    
    let mut buffer:[u8;8] = [0,0,0,0,0,0,0,0];
    let mut counter = 0;
    loop {
        if counter == 7 {
            break;
        }
        counter = counter+1;

        //Recieve data from Socket
        let data = raw_socket.recv_from(&mut buffer);

        let (bytes_recvd,src_addr) = match data {
        Ok((bytes,src_addr)) => (bytes,src_addr),
        Err(error) => {panic!("Error: {}",error)}
    };

    println!("Recieved: {} from: {}",bytes_recvd,src_addr);
    }
    println!("Buffer: {:02X?}",buffer);
    
}
