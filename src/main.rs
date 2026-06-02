use igd::{search_gateway, PortMappingProtocol};

use std::net::{SocketAddrV4, Ipv4Addr, UdpSocket};

use std::process::{Child, Command};

use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

use std::thread;
use std::time::Duration;

use std::fs::{OpenOptions, read_to_string};
use std::io::Write;

fn get_local_ip() -> Option<Ipv4Addr> {
    let socket = UdpSocket::bind("0.0.0.0:0").ok()?;
    socket.connect("8.8.8.8:80").ok()?;

    match socket.local_addr().ok()?.ip() {
        std::net::IpAddr::V4(ip) => Some(ip),
        _ => None,
    }
}

// 📝 Fonction pour ajouter la résolution locale temporaire
fn add_hosts_entry(domain: &str, ip: &str) {
    #[cfg(target_os = "windows")]
    let hosts_path = r"C:\Windows\System32\drivers\etc\hosts";
    #[cfg(not(target_os = "windows"))]
    let hosts_path = "/etc/hosts";

    // On vérifie si l'entrée n'existe pas déjà
    if let Ok(content) = read_to_string(hosts_path) {
        if content.contains(domain) {
            return; 
        }
    }

    if let Ok(mut file) = OpenOptions::new().append(true).open(hosts_path) {
        let entry = format!("\n{} {} # VSS-TEMP\n", ip, domain);
        if let Err(e) = file.write_all(entry.as_bytes()) {
            eprintln!("[VSS] Erreur d'écriture dans hosts (droits admin requis) : {:?}", e);
        } else {
            println!("[VSS] DNS local configuré : {} ➔ {}", domain, ip);
        }
    } else {
        eprintln!("[VSS] ⚠️ Impossible d'ouvrir le fichier hosts. Lance le programme en ROOT / ADMIN !");
    }
}

// 🧹 Fonction pour nettoyer le fichier hosts à la fermeture
fn remove_hosts_entry(domain: &str) {
    #[cfg(target_os = "windows")]
    let hosts_path = r"C:\Windows\System32\drivers\etc\hosts";
    #[cfg(not(target_os = "windows"))]
    let hosts_path = "/etc/hosts";

    if let Ok(content) = read_to_string(hosts_path) {
        let mut lines: Vec<String> = content.lines().map(|l| l.to_string()).collect();
        // On filtre pour enlever la ligne contenant notre domaine temporaire
        lines.retain(|line| !line.contains(domain));

        if let Ok(mut file) = OpenOptions::new().write(true).truncate(true).open(hosts_path) {
            for line in lines {
                let _ = writeln!(file, "{}", line);
            }
            println!("[VSS] Nettoyage du DNS local effectué.");
        }
    }
}

fn start_mediamtx(binary: &str, config: &str) -> Option<Child> {
    match Command::new(binary)
        .arg(config)
        .spawn()
    {
        Ok(child) => {
            println!("[VSS] MediaMTX démarré");
            Some(child)
        }
        Err(e) => {
            eprintln!("[VSS] Impossible de démarrer MediaMTX: {e}");
            None
        }
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

    let wan_ip = if let Some(ref gw) = gateway {
        match gw.get_external_ip() {
            Ok(ip) => {
                println!("IP externe de la box: {}", ip);
                Some(ip)
            }
            Err(e) => {
                eprintln!("Erreur WAN IP: {:?}", e);
                None
            }
        }
    } else {
        None
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
    if let Some(ref gw) = gateway {
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
                println!("[VSS] ⚠️ Pas de redirection automatique, ouvrez le port {} manuellement!!! ⚠️", port);
            }
        }
    } else {
        println!("[VSS] ⚠️ Pas de redirection automatique, ouvrez le port {} manuellement!!! ⚠️", port);
    }

    // 🌐 Définition du domaine sslip public
    let temp_domain = match wan_ip {
        Some(ip) => format!("{}.sslip.io", ip),
        None => format!("{}.sslip.io", local_ip), // Fallback
    };

    // Setup Host
    add_hosts_entry(&temp_domain, "127.0.0.1");

    println!(
        "\n[VSS] OBS Url: rtmp://localhost/vss\n[VSS] VRC Url: rtspt://{}:{}/vss\n",
        temp_domain, port
    );

    #[cfg(target_os = "windows")]
    let mediamtx = "libs/mediamtx/mediamtx.exe";

    #[cfg(not(target_os = "windows"))] 
    let mediamtx = "libs/mediamtx/mediamtx";
    
    let config = "libs/mediamtx/mediamtx.yml";

    let _ = Command::new(mediamtx)
        .arg("--upgrade")
        .status();

    thread::sleep(Duration::from_secs(3));

    let running = Arc::new(AtomicBool::new(true));

    {
        let running = running.clone();

        ctrlc::set_handler(move || {
            println!("\n[VSS] Arrêt demandé...");
            running.store(false, Ordering::SeqCst);
        })
        .expect("Erreur Ctrl+C");
    }


    let mut child = start_mediamtx(mediamtx, config);

    while running.load(Ordering::SeqCst) {
        match child.as_mut() {
            Some(process) => {
                match process.try_wait() {
                    Ok(Some(status)) => {
                        println!(
                            "[VSS] MediaMTX s'est arrêté ({status}), redémarrage dans 5s..."
                        );

                        thread::sleep(Duration::from_secs(5));

                        if running.load(Ordering::SeqCst) {
                            child = start_mediamtx(mediamtx, config);
                        }
                    }

                    Ok(None) => {
                        thread::sleep(Duration::from_secs(1));
                    }

                    Err(e) => {
                        eprintln!("[VSS] Erreur supervision: {e}");
                        thread::sleep(Duration::from_secs(5));
                    }
                }
            }

            None => {
                thread::sleep(Duration::from_secs(5));

                if running.load(Ordering::SeqCst) {
                    child = start_mediamtx(mediamtx, config);
                }
            }
        }
    }

    println!("[VSS] Arrêt de MediaMTX...");

    if let Some(mut process) = child {
        if let Err(e) = process.kill() {
        eprintln!("[VSS] Kill MediaMTX: {e}");
        }
    }

    // 🧹 Nettoyage DNS local avant de quitter
    remove_hosts_entry(&temp_domain);

    // fermeture UPnP si dispo
    if let Some(gw) = gateway {
        println!("Fermeture UPnP...");
        let _ = gw.remove_port(PortMappingProtocol::TCP, port);
    }

    println!("Entrée pour fermer...");
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input);
}
