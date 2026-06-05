use std::{fs, path::Path};
use semver::Version;
use serde::Deserialize;

#[derive(Deserialize)]
struct Release { tag_name: String, assets: Vec<Asset> }
#[derive(Deserialize)]
struct Asset { name: String, browser_download_url: String }

const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");
const NEXT_DIR: &str = "./.vss_next";

#[cfg(target_os = "windows")]
const EXE_NAME: &str = "VSS.exe";
#[cfg(not(target_os = "windows"))]
const EXE_NAME: &str = "VSS";

fn zip_name() -> &'static str {
    if cfg!(target_os = "windows") { "vss-windows.zip" } else { "vss-linux.zip" }
}

fn download(url: &str, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut resp = reqwest::blocking::get(url)?;
    std::io::copy(&mut resp, &mut fs::File::create(path)?)?;
    Ok(())
}

fn extract_to_staging(zip_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut zip = zip::ZipArchive::new(fs::File::open(zip_path)?)?;
    let _ = fs::remove_dir_all(NEXT_DIR);

    for i in 0..zip.len() {
        let mut file = zip.by_index(i)?;
        let path = match file.enclosed_name() { Some(p) => Path::new(NEXT_DIR).join(p), None => continue };

        if file.name().ends_with('/') { fs::create_dir_all(&path)?; continue; }
        if let Some(p) = path.parent() { fs::create_dir_all(p)?; }
        std::io::copy(&mut file, &mut fs::File::create(&path)?)?;

        #[cfg(unix)]
        if path.file_name().unwrap_or_default() == EXE_NAME {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&path, fs::Permissions::from_mode(0o755))?;
        }
    }
    Ok(())
}

#[cfg(target_os = "windows")]
fn finalize_swap() -> Result<(), Box<dyn std::error::Error>> {
    let ps_script = format!(
r#"$proc = Get-Process -Id {pid} -ErrorAction SilentlyContinue
if ($proc) {{ $proc | Wait-Process }}
Robocopy "{next}\VSS" "." /E /MOVE /R:2 /W:1 | Out-Null
if ($LASTEXITCODE -ge 8) {{ exit 1 }}
Remove-Item -Recurse -Force "{next}" -ErrorAction SilentlyContinue
Start-Process "{exe}""#,
        pid = std::process::id(), next = NEXT_DIR, exe = EXE_NAME
    );
    fs::write("update.ps1", ps_script)?;
    std::process::Command::new("powershell")
        .args(["-WindowStyle", "Hidden", "-ExecutionPolicy", "Bypass", "-File", "update.ps1"]).spawn()?;
    std::process::exit(0);
}

#[cfg(not(target_os = "windows"))]
fn finalize_swap() -> Result<(), Box<dyn std::error::Error>> {
    // Le ZIP ayant un dossier racine "VSS/", le contenu extrait est dans .vss_next/VSS/
    let staging_vss = Path::new(NEXT_DIR).join("VSS");
    
    for entry in fs::read_dir(&staging_vss)? {
        let entry = entry?;
        let dest = Path::new(".").join(entry.file_name());
        if entry.path().is_dir() {
            let _ = fs::remove_dir_all(&dest);
            fs::create_dir_all(&dest)?;
            for sub in fs::read_dir(entry.path())? {
                let sub = sub?;
                fs::copy(sub.path(), dest.join(sub.file_name()))?;
            }
        } else {
            fs::copy(entry.path(), &dest)?;
        }
    }
    let _ = fs::remove_dir_all(NEXT_DIR);
    Ok(())
}

pub fn check_update() {
    let _ = fs::remove_dir_all(NEXT_DIR);
    #[cfg(target_os = "windows")]
    let _ = fs::remove_file("update.ps1");

    if let Ok(release) = reqwest::blocking::Client::new()
        .get("https://api.github.com/repos/Simaire/VSS/releases/latest")
        .header("User-Agent", "VSS").send().and_then(|r| r.json::<Release>()) 
    {
        if Version::parse(&release.tag_name.trim_start_matches('v')).unwrap_or(Version::new(0,0,0)) > Version::parse(CURRENT_VERSION).unwrap() {
            let zip = zip_name();
            if let Some(asset) = release.assets.iter().find(|a| a.name == zip) {
                if download(&asset.browser_download_url, zip).is_ok() && extract_to_staging(zip).is_ok() {
                    let _ = fs::remove_file(zip);
                    let _ = finalize_swap();
                }
            }
        }
    }
}