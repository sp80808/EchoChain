# EchoChain P2P Node: Multi-Language Client Usage Examples

This document provides usage examples for interacting with the EchoChain Python P2P node's local API (JSON-over-TCP) from various languages and environments.

---

## 1. Python Example
```python
import socket
import json

def send_command(command_type, payload, host='127.0.0.1', port=8002):
    with socket.create_connection((host, port)) as sock:
        message = json.dumps({'type': command_type, 'payload': payload}).encode()
        sock.sendall(message)
        response = sock.recv(4096)
        return json.loads(response.decode())

# Add file and announce
resp = send_command('local_add_file', {'filepath': '/path/to/file.wav'})
if resp['status'] == 'success':
    file_hash = resp['file_hash']
    send_command('local_announce_content', {'content_hash': file_hash})
    print('File added and announced:', file_hash)
else:
    print('Error:', resp['message'])

# Discover peers
peers_resp = send_command('local_request_content_info', {'content_hash': file_hash})
print('Peers:', peers_resp.get('peers'))

# Request file download
download_resp = send_command('local_request_file', {'content_hash': file_hash})
print('Download initiated:', download_resp)
```

---

## 2. Go Example
```go
package main

import (
    "encoding/json"
    "fmt"
    "net"
    "os"
)

func sendCommand(commandType string, payload map[string]interface{}) (map[string]interface{}, error) {
    conn, err := net.Dial("tcp", "127.0.0.1:8002")
    if err != nil {
        return nil, err
    }
    defer conn.Close()
    message := map[string]interface{}{ "type": commandType, "payload": payload }
    data, _ := json.Marshal(message)
    conn.Write(data)
    buf := make([]byte, 4096)
    n, _ := conn.Read(buf)
    var resp map[string]interface{}
    json.Unmarshal(buf[:n], &resp)
    return resp, nil
}

func main() {
    resp, err := sendCommand("local_add_file", map[string]interface{}{ "filepath": "/path/to/file.wav" })
    if err != nil {
        fmt.Println("Error:", err)
        os.Exit(1)
    }
    if resp["status"] == "success" {
        fileHash := resp["file_hash"].(string)
        sendCommand("local_announce_content", map[string]interface{}{ "content_hash": fileHash })
        fmt.Println("File added and announced:", fileHash)
        peersResp, _ := sendCommand("local_request_content_info", map[string]interface{}{ "content_hash": fileHash })
        fmt.Println("Peers:", peersResp["peers"])
        downloadResp, _ := sendCommand("local_request_file", map[string]interface{}{ "content_hash": fileHash })
        fmt.Println("Download initiated:", downloadResp)
    } else {
        fmt.Println("Error:", resp["message"])
    }
}
```

---

## 3. Bash Example (with netcat and jq)
```bash
# Add file and announce
echo '{"type":"local_add_file","payload":{"filepath":"/path/to/file.wav"}}' | nc 127.0.0.1 8002 | tee response.json
file_hash=$(jq -r '.file_hash' response.json)
echo "{\"type\":\"local_announce_content\",\"payload\":{\"content_hash\":\"$file_hash\"}}" | nc 127.0.0.1 8002

# Discover peers
echo "{\"type\":\"local_request_content_info\",\"payload\":{\"content_hash\":\"$file_hash\"}}" | nc 127.0.0.1 8002

# Request file download
echo "{\"type\":\"local_request_file\",\"payload\":{\"content_hash\":\"$file_hash\"}}" | nc 127.0.0.1 8002
```

---

## 4. C# (.NET Core Example)
```csharp
using System;
using System.Net.Sockets;
using System.Text;
using Newtonsoft.Json;
using Newtonsoft.Json.Linq;

class Program
{
    static JObject SendCommand(string commandType, object payload)
    {
        using (var client = new TcpClient("127.0.0.1", 8002))
        using (var stream = client.GetStream())
        {
            var message = JsonConvert.SerializeObject(new { type = commandType, payload });
            var data = Encoding.UTF8.GetBytes(message);
            stream.Write(data, 0, data.Length);
            var buffer = new byte[4096];
            int bytesRead = stream.Read(buffer, 0, buffer.Length);
            var response = Encoding.UTF8.GetString(buffer, 0, bytesRead);
            return JObject.Parse(response);
        }
    }

    static void Main()
    {
        var resp = SendCommand("local_add_file", new { filepath = "/path/to/file.wav" });
        if ((string)resp["status"] == "success")
        {
            var fileHash = (string)resp["file_hash"];
            SendCommand("local_announce_content", new { content_hash = fileHash });
            Console.WriteLine("File added and announced: " + fileHash);
            var peersResp = SendCommand("local_request_content_info", new { content_hash = fileHash });
            Console.WriteLine("Peers: " + peersResp["peers"]);
            var downloadResp = SendCommand("local_request_file", new { content_hash = fileHash });
            Console.WriteLine("Download initiated: " + downloadResp);
        }
        else
        {
            Console.WriteLine("Error: " + resp["message"]);
        }
    }
}
```

---

### How to Adapt
- Change the `filepath` and `content_hash` as needed.
- Ensure the Python P2P node is running and listening on the correct port.
- The protocol is simple JSON-over-TCP, so any language with TCP and JSON support can integrate. 