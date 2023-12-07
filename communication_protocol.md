# Communication Protocol

## Message Structure
as usual, the message structure follows like this:
`<kind><data-length><data>`

- the `cmd` part is a single-byte which represents what kind of message is this.
- the `data-length` part is an unsigned 4-byte integer in big-endian which represents the length of the data part.
- the `data` part is an arbitary length data (length specified in `data-length`) this segmend contains data and/or attributes which are individual.

## Message Kinds

### Client -> Server
...

### Server -> Client
...
