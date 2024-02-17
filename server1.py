# server1.py
import socket

HOST = 'localhost'
PORT = 8001

def main():
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
        s.bind((HOST, PORT))
        s.listen()
        print(f"Server 1 listening on {HOST}:{PORT}")
        while True:
            conn, addr = s.accept()
            with conn:
                print('Connected by', addr)
                data = conn.recv(1024)
                print('Received', repr(data))
                conn.sendall(b'HTTP/1.1 200 OK\n\nHello, World! from Server 1')

if __name__ == "__main__":
    main()

