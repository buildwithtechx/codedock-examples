import os
from flask import Flask, jsonify

app = Flask(__name__)

@app.get("/")
def read_root():
    return jsonify({"message": "Hello from Codedock Python Flask Example!"})

if __name__ == "__main__":
    port = int(os.environ.get("PORT", 3000))
    app.run(host="0.0.0.0", port=port)