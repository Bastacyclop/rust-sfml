/*
* Rust-SFML - Copyright (c) 2013 Letang Jeremy.
*
* The original software, SFML library, is provided by Laurent Gomila.
*
* This software is provided 'as-is', without any express or implied warranty.
* In no event will the authors be held liable for any damages arising from
* the use of this software.
*
* Permission is granted to anyone to use this software for any purpose,
* including commercial applications, and to alter it and redistribute it
* freely, subject to the following restrictions:
*
* 1. The origin of this software must not be misrepresented; you must not claim
*    that you wrote the original software. If you use this software in a product,
*    an acknowledgment in the product documentation would be appreciated but is
*    not required.
*
* 2. Altered source versions must be plainly marked as such, and must not be
*    misrepresented as being the original software.
*
* 3. This notice may not be removed or altered from any source distribution.
*/

//! Specialized socket using the TCP protocol

use libc::size_t;
use std::{slice, ptr, mem};
use std::vec::Vec;

use traits::Wrappable;
use network::{IpAddress, Packet, SocketStatus};
use system::Time;

use ffi::sfml_types::{SFTRUE, SFFALSE};
use ffi::network::tcp_socket as ffi;

/// Specialized socket using the TCP protocol
pub struct TcpSocket {
    #[doc(hidden)]
    socket: *mut ffi::sfTcpSocket
}

impl TcpSocket {
    /// Create a new TCP socket
    ///
    /// Return Some(TcpSocket) or None
    pub fn new() -> Option<TcpSocket> {
        let tcp = unsafe { ffi::sfTcpSocket_create() };
        if tcp.is_null() {
            None
        }
        else {
            Some(TcpSocket {
                socket: tcp
            })
        }
    }

    /// Set the blocking state of a TCP listener
    ///
    /// In blocking mode, calls will not return until they have
    /// completed their task. For example, a call to
    /// sfUDPSocket_receive in blocking mode won't return until
    /// new data was actually received.
    /// In non-blocking mode, calls will always return immediately,
    /// using the return code to signal whether there was data
    /// available or not.
    /// By default, all sockets are blocking.
    ///
    /// # Arguments
    /// * blocking - true to set the socket as blocking, false for non-blocking
    pub fn set_blocking(&mut self, blocking: bool) -> () {
        unsafe {
            match blocking  {
                true        => ffi::sfTcpSocket_setBlocking(self.socket, SFTRUE),
                false       => ffi::sfTcpSocket_setBlocking(self.socket, SFFALSE)
            }
        }
    }

    /// Tell whether a TCP socket is in blocking or non-blocking mode
    ///
    /// Return true if the socket is blocking, false otherwise
    pub fn is_blocking(&self) -> bool {
        match unsafe { ffi::sfTcpSocket_isBlocking(self.socket) } {
            SFFALSE => false,
            SFTRUE  => true
        }
    }

    /// Get the port to which a TCP socket is bound locally
    ///
    /// If the socket is not bound to a port, this function
    /// returns 0.
    ///
    /// Return the port to which the socket is bound
    pub fn get_local_port(&self) -> u16 {
        unsafe {
            ffi::sfTcpSocket_getLocalPort(self.socket)
        }
    }

    /// Get the address of the connected peer of a TCP socket
    ///
    /// It the socket is not connected, this function returns
    /// sfIpAddress_None.
    ///
    /// Return the address of the remote peer
    pub fn get_remote_address(&self) -> IpAddress {
        unsafe {
            Wrappable::wrap(ffi::sfTcpSocket_getRemoteAddress(self.socket))
        }
    }

    /// Get the port of the connected peer to which
    /// a TCP socket is connected
    ///
    /// If the socket is not connected, this function returns 0.
    ///
    /// Return the remote port to which the socket is connected
    pub fn get_remote_port(&self) -> u16 {
        unsafe {
            ffi::sfTcpSocket_getRemotePort(self.socket)
        }
    }

    /// Connect a TCP socket to a remote peer
    ///
    /// In blocking mode, this function may take a while, especially
    /// if the remote peer is not reachable. The last parameter allows
    /// you to stop trying to connect after a given timeout.
    /// If the socket was previously connected, it is first disconnected.
    ///
    /// # Arguments
    /// * remoteAddress - Address of the remote peer
    /// * remotePort - Port of the remote peer
    /// * timeout - Maximum time to wait
    pub fn connect(&self, host: &IpAddress, port: u16, timeout: Time) -> SocketStatus {
        unsafe {
            mem::transmute(ffi::sfTcpSocket_connect(self.socket, host.unwrap(), port, timeout.unwrap()) as i8)
        }
    }

    /// Disconnect a TCP socket from its remote peer
    ///
    /// This function gracefully closes the connection. If the
    /// socket is not connected, this function has no effect.
    pub fn disconnect(&mut self) -> () {
        unsafe {
            ffi::sfTcpSocket_disconnect(self.socket)
        }
    }

    /// Send raw data to the remote peer of a TCP socket
    ///
    /// # Arguments
    /// * data - Vector of the sequence of bytes to send
    ///
    /// Return the status code
    pub fn send(&self, data: &[i8]) -> SocketStatus {
        unsafe {
            mem::transmute(ffi::sfTcpSocket_send(self.socket, data.as_ptr(), data.len() as size_t) as i8)
        }
    }

    /// Receive raw data from the remote peer of a TCP socket
    ///
    /// In blocking mode, this function will wait until some
    /// bytes are actually received.
    /// This function will fail if the socket is not connected.
    ///
    /// # Arguments
    /// * size - Maximum number of bytes that can be received
    ///
    /// Return a tuple containing the size read, a vector width data and the socket status
    pub fn receive(&self, max_size: size_t) -> (Vec<i8>, SocketStatus, size_t) {
        unsafe {
            let mut s: size_t = 0;
            let datas: *mut i8 = ptr::null_mut();
            let stat: SocketStatus = mem::transmute(ffi::sfTcpSocket_receive(self.socket, datas, max_size, &mut s) as i8);
            (slice::from_raw_buf(mem::transmute(&datas), s as usize).to_vec(), stat, s)
        }
    }

    /// Send a formatted packet of data to the remote peer of a TCP socket
    ///
    /// # Arguments
    /// * packet - Packet to send
    ///
    /// Return the socket status
    pub fn send_packet(&self, packet: &Packet) -> SocketStatus {
        unsafe {
            mem::transmute(ffi::sfTcpSocket_sendPacket(self.socket, packet.unwrap()) as i8)
        }
    }

    /// Receive a formatted packet of data from the remote peer
    ///
    /// In blocking mode, this function will wait until the whole packet
    /// has been received.
    /// This function will fail if the socket is not connected.
    ///
    /// Return a packet and a socket status
    pub fn receive_packet(&self) -> (Packet, SocketStatus) {
        unsafe {
            let pack: *mut ::ffi::network::packet::sfPacket = ptr::null_mut();
            let stat: SocketStatus = mem::transmute(ffi::sfTcpSocket_receivePacket(self.socket, pack) as i8);
            (Wrappable::wrap(pack), stat)
        }
    }
}

impl Wrappable<*mut ffi::sfTcpSocket> for TcpSocket {
    fn wrap(socket: *mut ffi::sfTcpSocket) -> TcpSocket {
        TcpSocket {
            socket: socket
        }
    }

    fn unwrap(&self) -> *mut ffi::sfTcpSocket {
        self.socket
    }
}

impl Drop for TcpSocket {
    fn drop(&mut self) -> () {
        unsafe {
            ffi::sfTcpSocket_destroy(self.socket)
        }
    }
}
