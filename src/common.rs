/*
 * client and server common functionality
 */

/*
 * socket related functions
 */
mod sock {
    use std::{net::TcpStream, io::{Write, Read}};


    trait GameProtocol {

        // sends a message
        fn send_message(&mut self, kind: u8, data: &[u8]);

        // recieves a message
        fn recieve_message(&mut self) -> (u8, Vec<u8>);
    }

    /*
     * this implements the protocol for the standard TcpStream
     */
    impl GameProtocol for TcpStream {
        fn send_message(&mut self, kind: u8, data: &[u8]) {
            // send kind
            self.write_all(&[kind]).unwrap();

            // send data length
            self.write_all(&(data.len() as u32).to_be_bytes()).unwrap();

            // send data
            self.write_all(data).unwrap();
        }

        fn recieve_message(&mut self) -> (u8, Vec<u8>) {
            
            // recieve kind
            let mut kind_buffer = [0u8];
            self.read_exact(&mut kind_buffer).unwrap();
            let kind = kind_buffer[0];

            // recieve length
            let mut length_buffer = [0u8; 4];
            self.read_exact(&mut length_buffer).unwrap();
            let length = u32::from_be_bytes(length_buffer);

            // recieve actual data
            let mut data_buffer: Vec<u8> = Vec::with_capacity(length as usize);
            self.read_exact(&mut data_buffer).unwrap();

            (kind, data_buffer)
        }
    }
}
