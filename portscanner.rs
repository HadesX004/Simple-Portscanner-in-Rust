use std::net::{SocketAddr, ToSocketAddrs, TcpStream};      
use std::env;

fn main(){
    let args: Vec<String> = env::args().collect();

    if args.len() < 1 {
        println!("Modo de uso: ");
        println!("./portscanner 192.168.0.1");

    }else{
        println!("Scanneando as low ports do host: {}", args[1]);

        let lowPorts = [21, 22, 23, 53, 80, 443, 9050, 1337];
        
        for port in lowPorts{
            let ipPort: Vec<SocketAddr> = format!("{}:{}", args[1], port).to_socket_addrs().expect("").collect();

            if ipPort.len() == 0{
                println!("Não foi possível criar o socket");

            }else{
                if let Ok(_) = TcpStream::connect(&ipPort[0]){
                    println!("Port {}: Open", port);

                }
            }
        }
    }
}