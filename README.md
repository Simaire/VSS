<div align="left" style="position: relative;">
<img src="https://raw.githubusercontent.com/PKief/vscode-material-icon-theme/ec559a9f6bfd399b82bb44393651661b08aaf7ba/icons/folder-markdown-open.svg" align="right" width="30%" style="margin: -20px 0 0 20px;">
<h1>VSS - Vrchat Streaming Server</h1>
<p align="left">
	<em><code>FR ❯ VSS est un utilitaire permetant de mettre en place un server MediaMTX (https://mediamtx.org/) et de l'exposer sur internet, permetant la conversion d'un flux RTMP (OBS) vers RTSP (VRChat)

</code></em>
<em><code>EN ❯ VSS is a utility that allows you to set up a MediaMTX server (https://mediamtx.org/) and expose it on the internet, allowing the conversion of an RTMP stream (OBS) to RTSP (VRChat)
</code></em>
</p>
<p align="left">
	<img src="https://img.shields.io/github/license/Simaire/VSS?style=default&logo=opensourceinitiative&logoColor=white&color=0080ff" alt="license">
	<img src="https://img.shields.io/github/last-commit/Simaire/VSS?style=default&logo=git&logoColor=white&color=0080ff" alt="last-commit">
	<img src="https://img.shields.io/github/languages/top/Simaire/VSS?style=default&color=0080ff" alt="repo-top-language">
	<img src="https://img.shields.io/github/languages/count/Simaire/VSS?style=default&color=0080ff" alt="repo-language-count">
</p>
<p align="left"><!-- default option, no dependency badges. -->
</p>
<p align="left">
	<!-- default option, no dependency badges. -->
</p>
</div>
<br clear="right">

## 🔗 Table of Contents

- [📍 Overview](#-overview)
- [👾 Features](#-features)
- [📁 Project Structure](#-project-structure)
- [🚀 Getting Started](#-getting-started)
  - [☑️ Prerequisites](#-prerequisites)
  - [⚙️ Installation](#-installation)
  - [🤖 Usage](#🤖-usage)
  - [OBS](#OBS-Settings)
- [🔰 Contributing](#-contributing)
- [🎗 License](#-license)
- [🙌 Acknowledgments](#-acknowledgments)

---

## 📍 Overview

<code> FR ❯ VSS est un utilitaire coder un Rust. Il permetant de mettre en place un server de vidéo MediaMTX et l'exposer sur internet grace a une ouverture de port par upnp permetant la conversion d'un flux RTMP, provenant d'une source t'elle que OBS, vers un flux RTSP compatibles avec VRChat. VSS a pour objectif de fournir un flux vidéo stable a faible latence gratuitement, sans nécessiter de configuration complexe ou de matériel spécialisé, rendant la diffusion en direct accessible à tous les utilisateurs de VRChat.
</code>

<code> EN ❯ VSS is a utility coded in Rust. It allows you to set up a MediaMTX video server and expose it on the internet through port forwarding via UPnP, enabling the conversion of an RTMP stream, originating from a source such as OBS, to an RTSP stream compatible with VRChat. VSS aims to provide a stable, low-latency video stream for free, without requiring complex configuration or specialized hardware, making live streaming accessible to all VRChat users.
</code>

---

## 👾 Features

  * MediaMtx auto install
  * Upnp support
  * Host file support(to prevent the nat lookback)
  * Auto update

---

## 📁 Project Structure

```sh
└── VSS/
    ├── Cargo.lock
    ├── Cargo.toml
    ├── LICENSE
    ├── README.md
    ├── libs
    │   └── mediamtx
    └── src
        └── main.rs
```


---
## 🚀 Getting Started

### ☑️ Prerequisites

Before getting started with VSS, ensure your runtime environment meets the following requirements:

- **Broadcast software:** OBS or any software capable of streaming via RTMP.
- **Updated os:** Windows 10 or later, or a compatible Linux distribution.
- **Network configuration:** Ensure that your network allows for UPnP port forwarding, or be prepared to configure manual port forwarding on your router if necessary.


### ⚙️ Installation

Install VSS using one of the following methods:

**Using pre-built binaries:**
1. Download the latest release from the [GitHub Releases](https://gitjhub.com/Simaire/VSS/releases) page.
2. Extract the downloaded archive to your desired location.
3. Run the executable file as root/admnistrateur to start VSS.

**Build from source:**

1. Clone the VSS repository:
```sh
❯ git clone https://github.com/Simaire/VSS
```

2. Navigate to the project directory:
```sh
❯ cd VSS
```

3. Run the following command to build the project:
```sh
❯ cargo build --release
```

4. After the build process completes, you can find the executable in the `target/release` directory. Run it as root/administrateur to start VSS.


### 🤖 Usage

```sh
1❯ run the executable file as root/administrateur to start VSS.
```
```sh
2❯ If UPnP port forwarding fails, configure manual port forwarding on your router to forward the necessary ports to your local machine (port given by VSS).
```
```sh
3❯ Copie the RTMP URL provided by VSS and use it as the streaming URL in your broadcasting software (like OBS).
```
```sh
4❯ Start streaming from your broadcasting software, and VSS will convert the RTMP stream to RTSP, making it compatible with VRChat.
```
```sh
5❯ In VRChat, add the RTSP URL provided by VSS to your world to display the live stream.
```
```sh
Info❯ 
  - Some world may not support RTSP, make sure to use a world that does, or ask the world creator to add support for it.
  - Quest users may ave issues with RTSP streams  
```


### OBS Settings

- **Output Mode:** Advanced
- **Encoder:** x264
- **Bitrate:** 2500 Kbps (adjust based on your internet upload speed)
- **Keyframe Interval:** 2 seconds
- **CPU Usage Preset:** Very Fast (adjust based on your CPU capabilities)


---

## 🔰 Contributing

- **💬 [Join the Discussions](https://github.com/Simaire/VSS/discussions)**: Share your insights, provide feedback, or ask questions.
- **🐛 [Report Issues](https://github.com/Simaire/VSS/issues)**: Submit bugs found or log feature requests for the `VSS` project.
- **💡 [Submit Pull Requests](https://github.com/Simaire/VSS/blob/main/CONTRIBUTING.md)**: Review open PRs, and submit your own PRs.



<summary>Contributor Graph</summary>
<br>
<p align="left">
   <a href="https://github.com/Simaire/VSS/graphs/contributors">
      <img src="https://contrib.rocks/image?repo=Simaire/VSS">
   </a>
</p>


---

## 🎗 License

This project is protected under the [MIT License](https://choosealicense.com/licenses/mit/) License.

---

## 🙌 Acknowledgments

- [MediaMTX](https://github.com/bluenviron/mediamtx)
- [VRCVideoCacher](https://github.com/EllyVR/VRCVideoCacher)
- Neocraft1293
- Saokina
- IA tools used in this project:
  - [ChatGPT](https://chat.openai.com/)
  - [GitHub Copilot](https://copilot.github.com/)
  - [gemini](https://gemini.google.dev/)

---
