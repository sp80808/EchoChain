from flask import Flask, jsonify, send_file, abort
import socket
import json
import os

app = Flask(__name__)
LOCAL_API_HOST = '127.0.0.1'
LOCAL_API_PORT = 9000  # Adjust if needed


def send_api_command(command):
    with socket.create_connection((LOCAL_API_HOST, LOCAL_API_PORT)) as sock:
        sock.sendall(json.dumps(command).encode())
        response = sock.recv(4096)
        return json.loads(response.decode())

@app.route('/files', methods=['GET'])
def list_files():
    cmd = {"type": "local_request_content_info", "payload": {"content_hash": "all_available_content"}}
    resp = send_api_command(cmd)
    if resp.get('status') == 'success':
        return jsonify(resp.get('available_content', []))
    return jsonify({'error': resp.get('message', 'Unknown error')}), 404

@app.route('/files/<file_hash>/info', methods=['GET'])
def file_info(file_hash):
    cmd = {"type": "local_request_content_info", "payload": {"content_hash": file_hash}}
    resp = send_api_command(cmd)
    if resp.get('status') == 'success':
        return jsonify(resp)
    return jsonify({'error': resp.get('message', 'Not found')}), 404

@app.route('/files/<file_hash>', methods=['GET'])
def download_file(file_hash):
    # First, request the file (may trigger download if not local)
    cmd = {"type": "local_request_file", "payload": {"content_hash": file_hash}}
    resp = send_api_command(cmd)
    if resp.get('status') == 'success':
        # Try to serve the file from the downloads directory
        filename = resp.get('filename')
        if filename:
            file_path = os.path.join('downloads', filename)
            if os.path.exists(file_path):
                return send_file(file_path, as_attachment=True)
    return abort(404)

if __name__ == '__main__':
    app.run(host='0.0.0.0', port=5001) 