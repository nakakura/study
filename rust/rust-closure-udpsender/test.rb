require 'socket'

udps = UDPSocket.open()

port = 10000

udps.bind('', port)

p "start loop"
loop do
  p "in loop"
    data = udps.recv(65535).chomp
      if data == 'exit'
            break
              else
                    p data
                      end
end

udps.close
