import json
from http.server import BaseHTTPRequestHandler, HTTPServer

class TraceHandler(BaseHTTPRequestHandler):
    def do_POST(self):
        content_length = int(self.headers['Content-Length'])
        json_data = self.rfile.read(content_length)
        
        try:
            trace = json.loads(json_data)
            print(json.dumps(trace, indent=4))  # Output the trace to standard output
        except json.JSONDecodeError as e:
            print(f"Failed to parse JSON: {e}")
        
        self.send_response(200)
        self.send_header('Content-type', 'text/html')
        self.end_headers()
        self.wfile.write(b'Trace received successfully')

def run_server(server_class=HTTPServer, handler_class=TraceHandler, port=8000):
    server_address = ('', port)
    httpd = server_class(server_address, handler_class)
    print(f'Starting server on port {port}...')
    httpd.serve_forever()

if __name__ == '__main__':
    run_server()