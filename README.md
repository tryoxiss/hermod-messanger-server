# hermod-server

> **Important**
> We have migrated this repository to [codeberg](https://codeberg.org/tryoxiss/hermod-messanger-server) due to github using public repos as AI training data. This repository will be completely purged at some point.

An implementation of our down Distributed Instant Messaging (DIM) protocol in Rust. It is built in full complaince with the spec and is intended to be as easy to install and configure as possible and be able to be run on anything from a Rasberry Pi to a indistry grade server. 

This implementation has a few goals, most importantly *100% spec compliance*, being *simple and easy* to set up and manage, being *highly configurable*, and being *fast yet still memory efficent*. While hermod-server can run almost anywhere, we only officially support important enviornments such as Linux Sever distros, and Rasberry Pi OS.

> **Warning**
> This software is not currently suitable for any productin builds and is not spec-compliant as it is in very early developement.

## Progress

- [x] IPv6
- [x] Constructing Packets
- [x] Encryption
- [ ] Storing Data
- [ ] Responding properly to requests

## FAQ

**What operating systems can it run on?**
We currently only support Linux kernel operating systems. See the chart below for what we have tested. 

Namely we test it on CentOS, Unubtu Server, Debian, Fedora Server, and Rasberry Pi OS. 

| OS       | Version | Support   |
| -------- | ------- | --------- |
| Debian   | 12      | Very Good |
| Rasberry Pi OS | ??? | Very Good |
| Fedora Server | 38 | Good |
| Ubuntu Server | 22.04.2 LTS | Good |
| Windows Server | All | None |
| Mac Server | All | None |
| Other Distributions | N/A | If it runs |

**Why the name?**
*Hermod* is the name of the norse messanger god, which works for hopefully obvious reasons. The name could also be "war god".

## Developement

### What this is

- A server-side implementation of DIM protocol
- A way to fetch and relay said DIM data

### What this is not

- A client. Don't put crap in here, that goes to the users client.
