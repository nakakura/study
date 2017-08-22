# _*_ coding: utf-8 _*_
require 'kconv'

#!/usr/bin/ruby

require "socket"

udps = UDPSocket.open()

udps.bind("0.0.0.0", 2000)
for num in 0..100 do
p udps.recv(65535).toutf8
end
udps.close


