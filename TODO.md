# TODO/VISION/PLANS:

## To Implement initialy
### 1. **Latency Metrics**

- **DNS Lookup Time**: Time taken to resolve the domain name to an IP address.
- **TCP Connection Time**: Time spent establishing the TCP connection.
- **TLS Handshake Time**: Time to complete the SSL/TLS handshake (if HTTPS is used).
- **Time to First Byte (TTFB)**: Time taken to receive the first byte of the response after the request is sent.
- **Total Round Trip Time (RTT)**: Time from sending a request to receiving the complete response.

### 2. **Throughput Metrics**

- **Request/Response Rate**: Number of requests/responses processed per second.
- **Data Transfer Rate**: Rate at which data is transferred, usually measured in bytes per second (Bps) or bits per second (bps).
- **Bandwidth Utilization**: Measures how much of the available bandwidth is being used.

### 3. **Response Time Metrics**

- **Response Time Distribution**: Percentile-based analysis (e.g., 50th, 95th, 99th percentile) to see the distribution of response times.
- **Average Response Time**: Mean time to receive a response from the server.
- **Maximum/Minimum Response Time**: The longest and shortest times taken to get responses during the benchmarking.

### 4. **Error Metrics**

- **Error Rate**: Percentage of failed requests (e.g., timeouts, 4xx or 5xx HTTP status codes).
- **Response Codes Distribution**: Breakdown of response codes (e.g., 200, 404, 500) to understand the types of responses returned.
- **Retries or Timeouts**: Count of requests that were retried or timed out.

### 5. **Concurrency and Load Metrics**

- **Concurrent Connections**: Number of simultaneous connections being handled.
- **Requests Per Second (RPS)**: Number of HTTP requests completed per second.
- **Load per User**: Data sent/received per user session.

### 6. **Payload Metrics**

- **Request/Response Size**: Size of the HTTP requests and responses, including headers and body.
- **Compression Ratio**: Data compression ratio if content is sent with compression (e.g., Gzip or Brotli).
- **Content Length**: Length of the response body in bytes.

### 7. **Protocol-Level Metrics**

- **HTTP Version**: Whether HTTP/1.1, HTTP/2, or HTTP/3 is used, and how it affects performance.
- **Connection Reuse (Keep-Alive)**: How efficiently the tool is reusing existing TCP connections.
- **Header Size**: Size of HTTP headers in both request and response.
- **Pipeline/Multiplexing Efficiency**: Especially with HTTP/2, measuring how well multiple requests are handled over a single connection.

### 8. **Network Metrics**

- **Packet Loss**: Percentage of lost data packets during the transmission.
- **Jitter**: Variation in packet arrival times, important for real-time applications.
- **Network RTT**: The network-specific round-trip time excluding processing delays on the server.
- **Re-transmission Rate**: Number of packets that had to be re-sent due to network issues.

### 9. **CPU/Memory Utilization (Optional)**

- **Client-Side CPU Usage**: CPU resources used by the client during benchmarking.
- **Client-Side Memory Usage**: Memory consumed by the benchmarking process.
- **Server-Side CPU/Memory Usage (if accessible)**: Measuring the load on the server under test, such as CPU utilization, to see how it handles the requests.

### 10. **Geolocation and Network Path Information**

- **Geographical Latency**: Delays caused by the physical distance between the client and the server.
- **Hops and Network Path**: The number of hops or intermediate routers between the client and server (via traceroute or similar).

### 11. **Connection Stability**

- **Connection Timeouts**: How often connections are dropped or timed out.
- **Session Duration**: Length of time a session can be maintained before it is closed.

### 12. **SSL/TLS Information (for HTTPS)**

- **SSL/TLS Protocol Version**: The version of SSL/TLS used (e.g., TLS 1.2, 1.3).
- **Certificate Details**: Information about SSL certificates, including expiration.
- **Handshake Performance**: Time spent during SSL/TLS handshakes, especially for HTTPS performance.

## So topics up for debate going one by one, feel free to collab