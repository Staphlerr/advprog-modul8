# Advance Programming Tutorial 8 - High Level Networking (GRPC)

Name: Belva Ghani Abhinaya
<br>Class: Advance Programming B
<br>Student Number: 2306203526

<details>
<summary><b>Reflection on Module 8</b></summary>
<br>

### Reflection for Tutorial 8

#### 1. Key differences between unary, server streaming, and bi-directional streaming RPC methods

We see unary RPC as a simple one-and-done call: we send one request, we get one response. Server streaming feels like “one request, many responses,” so it’s great for sending back a large list or real-time updates without reconnecting. Bi-directional streaming is like a live chat—both sides can keep talking whenever, which is perfect for interactive features. We’d pick unary for quick lookups (e.g., auth), server streaming for feeds (e.g., logs or history), and bi-directional for real-time interaction (e.g., chat or gaming).

#### 2. Potential security considerations in Rust gRPC

Rust gives us memory safety by default, but gRPC still needs TLS for encryption (so data isn’t sniffed). For authentication, we’d integrate something like JWT or mTLS to verify clients. Authorization means checking roles or scopes on each method—otherwise any client could call anything. Also we need to watch out for DoS: streaming can tie up threads, so we should rate-limit or use timeouts.

#### 3. Challenges when handling bi-directional streaming in Rust gRPC

In chat-style streams, we must manage backpressure carefully—if a client floods us, the server can run out of memory or hang. Handling broken connections gracefully is tricky: we need to detect and clean up both reading and writing tasks. Synchronizing shared state (e.g., chat rooms) safely across async tasks without deadlocks also adds complexity.

#### 4. Pros and cons of using `tokio_stream::wrappers::ReceiverStream`

Using `ReceiverStream` is super convenient: it wraps a Tokio channel into a gRPC stream, so we don’t write boilerplate. But the default channel size can be a bottleneck (fixed buffer), and if messages aren’t consumed fast enough we’ll hit backpressure or drop messages unless we handle it. It also hides some details of error handling, so we need extra logging if things go wrong.

#### 5. Structuring Rust gRPC code for reuse and modularity

We’d split each service into its own module (`payment.rs`, `transaction.rs`, `chat.rs`) and define shared types in a `proto_types` module. Common utility functions (like TLS setup and error mapping) would live in a `utils` crate. Using traits for service logic lets us swap implementations in tests or future versions. Finally, grouping build and client-side code under `build.rs` and a `client/` folder keeps everything tidy.

#### 6. MyPaymentService: extra steps for complex payment logic

Beyond the simple echo example, we’d add validations (e.g., check account balance), integrate with a database or external payment gateway, handle retries on failures, and log transactions securely. We’d implement idempotency keys to avoid double charges and wrap business errors in custom gRPC status codes (e.g., `InvalidArgument`, `Unavailable`).

#### 7. Impact of adopting gRPC on distributed system architecture

Adopting gRPC pushes us toward strongly typed, contract-first APIs—service boundaries become clear. It improves performance with HTTP/2 multiplexing but adds a dependency on codegen tools. It plays nicely with other languages (thanks to Protobuf) but might complicate integration with systems expecting plain JSON over HTTP.

#### 8. Advantages and disadvantages of HTTP/2 vs HTTP/1.1 (or WebSocket)

HTTP/2 gives us multiplexed streams over a single connection (faster startup, less head-of-line blocking) and built-in binary framing (ideal for Protobuf). In contrast, HTTP/1.1 needs separate connections or WebSocket hacks. But HTTP/2 can be harder to debug, some proxies don’t support it well, and it’s more complex to set up TLS everywhere.

#### 9. REST request-response vs gRPC bidirectional streaming

With REST we make a request, wait, get a response, then close the connection—fine for CRUD. But gRPC streaming keeps the channel open, so we get near-instant updates without reconnecting. That makes real-time features way more efficient and responsive, but also means we have to manage long-lived connections and more complex error handling.

#### 10. Schema-based Protobuf vs schema-less JSON in REST

Protobuf forces us to define a strict schema up front, which gives us fast binary encoding and compile-time checks—but we lose on-the-fly flexibility. JSON lets us send ad-hoc fields and debug easily in text, but it’s heavier on the wire and we only catch missing fields at runtime. We prefer Protobuf for performance-critical systems, while JSON still shines for simple public APIs.

</details>
