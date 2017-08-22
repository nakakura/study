# _*_ coding: utf-8 _*_
require 'kconv'
require "socket"

udp = UDPSocket.open()
sockaddr = Socket.pack_sockaddr_in(3000, "127.0.0.1")

for num in 1..10 do
udp.send("HELLO_" + num.to_s , 0, sockaddr)
udp.send("あいうえお_" + num.to_s, 0, sockaddr)
udp.send("漢字_" + num.to_s, 0, sockaddr)
end

udp.close
