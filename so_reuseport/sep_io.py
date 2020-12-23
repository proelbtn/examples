import socket

sock1 = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
sock1.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEPORT, 1)
sock1.bind(("localhost", 8080))

sock2 = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
sock2.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEPORT, 1)
sock2.bind(("localhost", 8080))

socks = [sock1, sock2]

i = 0
while True:
    data, addr = sock1.recvfrom(2048)
    print(data, addr)
    err = sock2.sendto(data, addr)

    i = (i + 1) % len(socks)

