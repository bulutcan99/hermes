# Discord Clone - Real-time Communication Platform

A production-ready, Discord-like real-time communication platform built with Rust. Features text chat, voice communication, screen sharing, and video streaming using a scalable microservice architecture.

## 🎯 Features

- **Text Messaging**: Real-time text chat in channels and direct messages
- **Voice Communication**: WebRTC-based voice channels with high-quality audio
- **Screen Sharing**: Share your screen with other users in real-time
- **Video Streaming**: Live video streaming capabilities
- **Server/Channel Management**: Create servers, channels, and manage permissions
- **User Presence**: Real-time online/offline/away status tracking
- **Friend System**: Add friends, accept requests, and manage relationships
- **Rich Media**: Share files, images, and attachments
- **Webhooks**: Integration support for external services

## 🏗️ Architecture

### Microservices

```
┌─────────────────┐     ┌──────────────────┐
│  Gateway Service │────▶│  Auth Service    │
│  (WebSocket/HTTP)│     │  (JWT Auth)      │
└─────────────────┘     └──────────────────┘
         │
         ├──────────────┐
         ▼              ▼
┌─────────────────┐  ┌──────────────────┐
│  Chat Service   │  │  User Service    │
│  (Messaging)    │  │  (Profiles)      │
└─────────────────┘  └──────────────────┘
         │              │
         ▼              ▼
┌─────────────────┐  ┌──────────────────┐
│ Channel Service │  │ Presence Service │
│ (Rooms/Servers) │  │ (Online Status)  │
└─────────────────┘  └──────────────────┘
         │              │
         ▼              ▼
┌─────────────────┐  ┌──────────────────┐
│ Voice Service   │  │ Stream Service   │
│ (WebRTC Voice)  │  │ (Screen Share)   │
└─────────────────┘  └──────────────────┘
         │              │
         └──────┬───────┘
                ▼
        ┌──────────────────┐
        │  Media Server    │
        │  (Media Routing) │
        └──────────────────┘
```

### Infrastructure

- **PostgreSQL 16**: Primary database for users, messages, channels
- **Redis**: Caching, pub/sub, presence tracking, rate limiting
- **NATS**: Event streaming and inter-service communication
- **MinIO**: Object storage for file uploads and media
- **Coturn**: TURN/STUN server for WebRTC NAT traversal
- **Prometheus + Grafana**: Metrics and monitoring

## 📁 Project Structure

```
discord-clone/
├── Cargo.toml                 # Workspace configuration
├── docker-compose.yml         # Infrastructure services
├── crates/
│   ├── common/               # Shared utilities, models, events
│   ├── auth-service/         # Authentication & JWT
│   ├── user-service/         # User profiles & friends
│   ├── channel-service/      # Servers, channels, permissions
│   ├── chat-service/         # Text messaging
│   ├── voice-service/        # Voice communication (WebRTC signaling)
│   ├── stream-service/       # Screen sharing & video streaming
│   ├── presence-service/     # Online/offline status
│   ├── gateway-service/      # WebSocket gateway & REST API
│   └── media-server/         # Media routing & processing
├── infra/
│   ├── postgres/            # Database schemas & migrations
│   ├── coturn/              # TURN server configuration
│   ├── prometheus/          # Metrics configuration
│   └── grafana/             # Dashboard provisioning
└── docs/                    # Additional documentation
```

## 🚀 Getting Started

### Prerequisites

- Rust 1.75+ (stable)
- Docker & Docker Compose
- PostgreSQL 16
- Redis 7
- NATS 2.10

### Infrastructure Setup

```bash
# Start all infrastructure services
docker-compose up -d

# Wait for services to be healthy
docker-compose ps

# Check service logs
docker-compose logs -f postgres redis nats
```

### Building the Project

```bash
# Build all services
cargo build --workspace --release

# Run tests
cargo test --workspace

# Check formatting
cargo fmt --all --check

# Run lints
cargo clippy --workspace --all-targets
```

### Running Services

Each service can be run independently:

```bash
# Terminal 1: Gateway Service (API & WebSocket)
cd crates/gateway-service
RUST_LOG=info cargo run

# Terminal 2: Auth Service
cd crates/auth-service
cargo run

# Terminal 3: Chat Service
cd crates/chat-service
cargo run

# Terminal 4: Voice Service
cd crates/voice-service
cargo run

# Terminal 5: Stream Service
cd crates/stream-service
cargo run

# Terminal 6: User Service
cd crates/user-service
cargo run

# Terminal 7: Channel Service
cd crates/channel-service
cargo run

# Terminal 8: Presence Service
cd crates/presence-service
cargo run

# Terminal 9: Media Server
cd crates/media-server
cargo run
```

## 🔧 Configuration

All services use environment variables for configuration:

```bash
# Database
DATABASE_URL=postgres://discord:discord_dev_password@localhost:5432/discord

# Redis
REDIS_URL=redis://:redis_dev_password@localhost:6379

# NATS
NATS_URL=nats://localhost:4222

# MinIO (Object Storage)
MINIO_ENDPOINT=localhost:9000
MINIO_ACCESS_KEY=discord_minio
MINIO_SECRET_KEY=discord_minio_password

# JWT Secret
JWT_SECRET=your-secret-key-change-in-production

# TURN Server
TURN_URL=turn:localhost:3478
TURN_USERNAME=discord
TURN_PASSWORD=discord_turn_password

# Logging
RUST_LOG=info
```

Create a `.env` file in each service directory or export these variables.

## 🗄️ Database Schema

### Core Tables

- **users**: User accounts, profiles, authentication
- **servers**: Discord-like servers (guilds)
- **channels**: Text, voice, and category channels
- **messages**: Chat messages and attachments
- **voice_sessions**: Active voice channel participants
- **friendships**: Friend relationships and requests
- **server_members**: Server membership and roles
- **direct_messages**: Private message channels

See `infra/postgres/init.sql` for complete schema.

## 🔌 API Endpoints

### Authentication
- `POST /api/auth/register` - Register new user
- `POST /api/auth/login` - Login and get JWT token
- `POST /api/auth/refresh` - Refresh access token

### Users
- `GET /api/users/@me` - Get current user
- `PATCH /api/users/@me` - Update current user
- `GET /api/users/{id}` - Get user by ID

### Servers
- `GET /api/servers` - List user's servers
- `POST /api/servers` - Create new server
- `GET /api/servers/{id}` - Get server details
- `PATCH /api/servers/{id}` - Update server
- `DELETE /api/servers/{id}` - Delete server

### Channels
- `GET /api/channels/{id}` - Get channel details
- `POST /api/servers/{id}/channels` - Create channel
- `PATCH /api/channels/{id}` - Update channel
- `DELETE /api/channels/{id}` - Delete channel

### Messages
- `GET /api/channels/{id}/messages` - Get channel messages
- `POST /api/channels/{id}/messages` - Send message
- `PATCH /api/messages/{id}` - Edit message
- `DELETE /api/messages/{id}` - Delete message

### WebSocket Gateway
- `wss://gateway/` - WebSocket connection for real-time events

## 📊 Monitoring

### Prometheus Metrics
Access at: http://localhost:9090

Example queries:
- `up{job="gateway-service"}` - Service availability
- `http_requests_total` - Total HTTP requests
- `websocket_connections` - Active WebSocket connections

### Grafana Dashboards
Access at: http://localhost:3000 (admin/admin)

Pre-configured dashboards for:
- Service health and uptime
- Message throughput
- Active users and connections
- Database performance

### NATS Monitoring
Access at: http://localhost:8222

## 🧪 Testing

```bash
# Run all tests
cargo test --workspace

# Run specific service tests
cargo test -p chat-service

# Run with output
cargo test --workspace -- --nocapture

# Run integration tests
cargo test --workspace --test '*'
```

## 📈 Performance Considerations

- **WebSocket Scaling**: Use multiple gateway instances behind load balancer
- **Database Sharding**: Shard by server ID for large deployments
- **Redis Clustering**: Enable Redis cluster for high availability
- **Media Server**: Deploy regional media servers for low latency
- **CDN**: Use CDN for static assets and uploaded files

## 🔐 Security

- JWT-based authentication with refresh tokens
- Argon2 password hashing
- Rate limiting on all endpoints
- CORS configuration for web clients
- SQL injection prevention via SQLx
- Input validation with validator crate

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📝 License

This project is licensed under the MIT License - see the LICENSE file for details.

## 🙏 Acknowledgments

- Inspired by Discord's architecture
- Built with the amazing Rust ecosystem
- WebRTC for real-time communication
- PostgreSQL for reliable data storage
