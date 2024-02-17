# main.py
import socket
import random

# List of server addresses
SERVERS = [('localhost', 8001), ('localhost', 8002)]

def main():
    # Setup main socket
    HOST = 'localhost'
    PORT = 9000

    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
        s.bind((HOST, PORT))
        s.listen()
        print(f"Load balancer listening on {HOST}:{PORT}")
        while True:
            conn, addr = s.accept()
            with conn:
                print('Connected by', addr)
                # Randomly choose a server to forward the request to
                server_address = random.choice(SERVERS)
                print('Forwarding request to', server_address)
                server_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
                server_socket.connect(server_address)
                while True:
                    data = conn.recv(1024)
                    if not data:
                        break
                    server_socket.sendall(data)
                    response = server_socket.recv(1024)
                    if not response:
                        break
                    conn.sendall(response)
                server_socket.close()

if __name__ == "__main__":
    main()

