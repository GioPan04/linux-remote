# linux-remote

A KDE Connect replacement written in Rust. This is not a drop-in replacement, you'll need another client (currently I'm creating one in Dart/Flutter, it will be available soon)

## The reason

Why build this from the ground up? Well, I didn't wanna to install all the kde dependencies only to control remotely my PC.
My version uses low level tools like uinput and mpris and uses little to no dependencies.

## Todos

- [x] Mouse/Keyboard remote control
- [x] Receive and show notifications
- [ ] SSL encryption
- [ ] Trusted devices
- [ ] Multimedia integration
  - [x] Send events to clients
  - [ ] Control players from clients 

## Docs

### Protocol

This project has been developed with the possibility to be expanded in the future. Each message sent in the socket it's a JSON object contains a `target` and a `payload`.

The target it's a string composed by the "plugin" name and the action name separated by a colon. The payload contents can be an object or a simple value.

Here it's an example used to move the cursor 3px left and 5px up:
```json
{
  "target": "uinput:cursor",
  "payload": {
    "x": 3,
    "y": 10
  }
}
```