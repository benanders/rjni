
Peer to Peer in Rust
====================

p2prs is a library for creating a TCP connection between two peers that may be behind a Network Address Translation box (NAT). This type of peer to peer communication requires a more complex method of connection initiation, which this library wraps up into a simple API. The method this library uses is described in [this paper](http://ra.adm.cs.cmu.edu/anon/isri2005/CMU-ISRI-05-104.pdf).

The library is for Mozilla's awesome new language [Rust](http://www.rust-lang.org).

### Connection Method

Below is a description of the method used to create the peer to peer TCP socket. The method requires an intermediate server (called the Connection Broker). The sender is the program wishing to create a new TCP request, and the receiver is the program the sender wishes to connect to.

*Initiation*

1. TCP: Receiver connects to connection broker, sends across UID
2. TCP: Connection broker sends acknowledgment back
3. TCP: Receiver sends keep alive messages every 30 seconds
4. TCP: Receiver sends kill message on quit. Broker also kills receiver if no message has been received for 65 seconds, or connection is closed

*Connection formation*

1. TCP: Sender sends request for connection to connection broker (identifying the receiver by its UID). Broker records public IP (`IP_SENDER`) and port (`PORT_SENDER`). Keeps TCP connection open until step 11.
2. TCP: Connection broker sends a connection request to receiver using the TCP connection
3. TCP: Receiver forms TCP with broker on outgoing `PORT_RECEIVER`, sends acknowledgment packet Broker records port as `PORT_RECEIVER`. Broker records public IP as `IP_RECEIVER`
4. TCP: Broker sends connection details to receiver. Sends sender's public IP (`IP_SENDER`) and port (`PORT_SENDER`)
5. Receiver closes connection to broker
6. TCP: Receiver sends TCP connection request to sender on `PORT_RECEIVER` to sender's public IP (`IP_SENDER`) and port (`PORT_SENDER`)
7. TCP: Error returned to receiver as that port will already be in use due to communication with broker
8. TCP: Receiver begins listening on `PORT_RECEIVER` for connection from sender
9. TCP: Receiver connects to broker on new TCP port, sends finished hole punch message, closes connection
10. TCP: Broker tells sender about receiver's details on socket opened in step 1. Sends `IP_RECEIVER` and `PORT_RECEIVER`
11. TCP: Broker closes TCP connection to sender
12. TCP: Sender connects to receiver on `IP_RECEIVER` and `PORT_RECEIVER`

### License

p2prs is licensed under the MIT license. See the `LICENSE` file for more detail.
