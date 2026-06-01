use igd::{search_gateway, PortMappingProtocol};
use std::net::{SocketAddrV4, Ipv4Addr, UdpSocket};
use std::thread;
use std::time::Duration;
use std::process::Command;

fn get_local_ip() -> Option<Ipv4Addr> {
    let socket = UdpSocket::bind("0.0.0.0:0").ok()?;
    socket.connect("8.8.8.8:80").ok()?;

    match socket.local_addr().ok()?.ip() {
        std::net::IpAddr::V4(ip) => Some(ip),
        _ => None,
    }
}


fn main() {
    println!("[VSS] Recherche de box...");

    let gateway = match search_gateway(Default::default()) {
        Ok(gw) => {
            println!("[VSS] Box UPnP détecté");
            Some(gw)
        }
        Err(_) => {
            println!("[VSS] Erreur Box non compatible/indisponible");
            None
        }
    };

    let local_ip = match get_local_ip() {
        Some(ip) => {
            println!("[VSS] IP locale: {}", ip);
            ip
        } 
        None => {
            eprintln!("[VSS] Impossible de récupérer l'IP locale");
            return;
        }
    };


    let port = 8554;
    let addr = SocketAddrV4::new(local_ip, port);

    // 🚪 Port mapping seulement si box dispo
    if let Some(gw) = &gateway {
        match gw.add_port(
            PortMappingProtocol::TCP,
            port,
            addr,
            86400,
            "VSS - VrchatStreamServer",
        ) {
            Ok(_) => println!("[VSS] Port UPnP {} ouvert", port),
            Err(e) => {
                println!("[VSS] UPnP refusé: {:?}", e);
                println!("[VSS] ⚠️  ⚠️  ⚠️ Pas de redirection automatique, ouvrez le port {} manuellement!!! ⚠️  ⚠️  ⚠️", port);
            }
        }
    } else {
        println!("[VSS] ⚠️  ⚠️  ⚠️ Pas de redirection automatique, ouvrez le port {} manuellement!!! ⚠️  ⚠️  ⚠️", port);
    }

    println!(
        "\n[VSS] OBS Url: http://localhost:8889/vss/whip\n[VSS] VRC Url: rtsp://{}:{}/vss\n",
        local_ip, port
    );

    let mediamtx = "libs/mediamtx/mediamtx.exe";
    let config = "libs/mediamtx/mediamtx.yml";

    let _ = Command::new(mediamtx)
        .arg("--upgrade")
        .status();

    thread::sleep(Duration::from_secs(3));

    let _ = Command::new(mediamtx)
        .arg(config)
        .status();

    println!("Entrée pour fermer...");
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input);

    // fermeture UPnP si dispo
    if let Some(gw) = gateway {
        println!("Fermeture UPnP...");
        let _ = gw.remove_port(PortMappingProtocol::TCP, port);
    }
}