#!/usr/bin/ruby

require "socket"

udps = UDPSocket.open()

udps.bind("0.0.0.0", 9000)

loop do
  p udps.recv(65535)
end

udps.close

