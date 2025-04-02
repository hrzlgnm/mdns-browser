![GitHub Downloads (all assets, all releases)](https://img.shields.io/github/downloads/hrzlgnm/mdns-browser/total)
![GitHub Downloads (all assets, latest release)](https://img.shields.io/github/downloads/hrzlgnm/mdns-browser/latest/total)
![GitHub Release](https://img.shields.io/github/v/release/hrzlgnm/mdns-browser)
![GitHub Release Date](https://img.shields.io/github/release-date/hrzlgnm/mdns-browser)

# mDNS-Browser

This application allows you to browse services using mDNS.

Screenshots from [v0.9.5](https://github.com/hrzlgnm/mdns-browser/releases/tag/mdns-browser-v0.9.5)

### Startup

![intro](./screenshots/intro.png)

### Browsing for \_ssh.\_tcp

![browsing-ssh](./screenshots/browsing-_ssh._tcp.png)

### Details of a resolved service having many IPs

![ssh-details](./screenshots/_ssh._tcp.details.png)

### Details of a resolved service having a subtype and TXT records

![amzn-details](./screenshots/_amzn-wplay._tcp-details.png)

### Checking for updates on Windows

![check-update](./screenshots/check-update.png)

### Response if no update is available

![no-update](./screenshots/no-update.png)

<!--toc:start-->

- [mDNS-Browser Overview](#mdns-browser)
  - [How to Build](#building)
  - [Where to find the executables?](#where-to-find-the-executables)
    - [GitHub Release](#github-releases)
    - [Winget Installation](#winget-installation)
    - [Arch Linux (AUR)](#arch-linux-aur)
    - [Void Linux](#void-linux)
  - [Privacy](#privacy)
  - [Acknowledgments](#acknowledgments)
  <!--toc:end-->

## Building

For instructions on building the application, checkout the document [BUILDING](BUILDING.md).

## Where to find the executables?

### GitHub Releases

You can download the latest version of the application from the [GitHub Release page](https://github.com/hrzlgnm/mdns-browser/releases/latest)

### Winget installation

To install via Winget, run the following command:

```console
winget install mdns-browser
```

### Arch Linux (AUR)

To install on Arch Linux using the AUR, you can use an AUR helper like yay or paru:

With `yay`:

```console
yay -S mdns-browser
```

Alternatively using the -bin package:

```console
yay -S mdns-browser-bin
```

With `paru`:

```console
paru -S mdns-browser
```

Alternatively using the -bin package:

```console
paru -S mdns-browser-bin
```

### Void Linux

To install on Void Linux with arch `x86_64`, you can add the GitHub release as a repository and install the package using `xbps-install`:

```console
# Add the repository
echo `repository=https://github.com/hrzlgnm/mdns-browser/releases/latest/download` | sudo tee /etc/xbps.d/mdns-browser-repo.conf

# Install the package
sudo xbps-install -S mdns-browser
```

There will be a prompt for accepting a public key signed by `hrzlgnm@users.noreply.github.com`. Which you need to accept to install the package. The repository and package are signed by with a key having the following fingerprint: `64:6d:b9:23:3d:ad:9d:f1:b0:fe:64:8e:da:46:57:d3`.

## Privacy

For a privacy statement checkout the document [PRIVACY](PRIVACY.md).

## Acknowledgments

This app uses the fantastic [mdns-sd library](https://github.com/keepsimple1/mdns-sd)to handle all mDNS functionality. If you find this app helpful, consider giving the library a star on GitHub!

```

```
