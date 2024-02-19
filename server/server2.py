# server2.py
from flask import Flask

app = Flask(__name__)

@app.route('/')
def index():
    return 'Hello world, Welcome to Server 2'

if __name__ == '__main__':
    app.run(host='172.20.10.3', port=8001)

