# server1.py
from flask import Flask

app = Flask(__name__)

@app.route('/')
def index():
    return 'Hello World, Welcome to Server 1'

if __name__ == '__main__':
    app.run(host='192.168.1.137', port=8000)

