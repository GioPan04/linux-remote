# linux-remote

A KDE Connect replacement written in Rust. This is not a drop-in replacement, you'll need another client (currently I'm creating one in Dart/Flutter, it will be available soon)

## The reason

Why build this from the ground up? Well, I didn't wanna to install all the kde dependencies only to control remotely my PC.
My version uses low level tools like uinput and mpris and uses little to no dependencies.

## Todos

- [x] Mouse/Keyboard remote control
- [ ] SSL encryption
- [ ] Trusted devices
- [ ] Multimedia integration
  - [x] Send events to clients
  - [ ] Control players from clients 