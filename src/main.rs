use igd::{search_gateway, PortMappingProtocol};
use std::net::{SocketAddrV4, Ipv4Addr, UdpSocket};
use std::thread;
use std::time::Duration;
use std::process::Command;

fn get_local_ip() -> Ipv4Addr {
    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
    socket.connect("8.8.8.8:80").unwrap();

    match socket.local_addr().unwrap().ip() {
        std::net::IpAddr::V4(ip) => ip,
        _ => panic!("IPv6 non supporté 😿"),
    }
}

fn main() {
    // 🔎 Recupere la box
    println!("Recherche de box...");
    let gateway = search_gateway(Default::default()).unwrap();

    // 🌍 Récupérer WAN IP
    let wan_ip = match gateway.get_external_ip() {
        Ok(ip) => {
            println!("IP externe de la box: {}", ip);
            ip
        }
        Err(e) => {
            eprintln!("Erreur WAN IP: {:?}", e);
            return;
        }
    };

    // 🌐 UPnP settings
    let local_ip = get_local_ip();
    println!("IP locale: {}", local_ip);

    let port = 8888; // HLS default port
    let duree = 86400; // 24H

    let addr = SocketAddrV4::new(local_ip, port);

    // 🚪 UPnP port mapping
    match gateway.add_port(
        PortMappingProtocol::TCP,
        port,
        addr,
        duree,
        "VSS - VrchatStreamServer",
    ) {
        Ok(_) => println!("Port {} ouvert", port),
        Err(e) => eprintln!("Erreur UPnP: {:?}", e),
    }

    // 📡 OBS
    let obsurl = "rtmp://127.0.0.1:1935/live";
    let obskey = "vss";

    println!(
        "Url OBS: {}\nKey: {}", 
        obsurl, obskey
    );

    // 🌐 VRC URL
    println!(
        "Url VRC: http://{}:{}/live/{}/index.m3u8",
        wan_ip, port, obskey
    );
    
    let mediamtx = "libs/mediamtx/mediamtx.exe";
    let config = "libs/mediamtx/mediamtx.yml";

    // 🎥 update MediaMTX
    Command::new(mediamtx)
        .arg("--upgrade")
        .status()
        .unwrap();
    
    // 😴 Pause
    thread::sleep(Duration::from_secs(3));

    // 🎥 Start MediaMTX
    Command::new(mediamtx)
        .arg(config)
        .status()
        .unwrap();

    // ⏳ Fin programme
    println!("Entrée pour fermer...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
}