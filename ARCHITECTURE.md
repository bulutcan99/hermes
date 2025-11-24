# Architecture Documentation

## Overview

Discord Clone is built using a microservice architecture with Rust, designed for scalability, real-time communication, and high performance. The system uses event-driven communication between services and WebRTC for media streaming.

## System Architecture

```
                                    Internet
                                       |
                                  Load Balancer
                                       |
                        ┌──────────────┴──────────────┐
                        |                             |
                   Gateway Service              Gateway Service
                  (WebSocket + HTTP)           (WebSocket + HTTP)
                        |                             |
                        └──────────────┬──────────────┘
                                       |
                        ┌──────────────┴──────────────┐
                        |                             |
                    NATS Event Bus              Redis Cache/PubSub
                        |                             |
          ┌─────────────┼─────────────┐              |
          |             |             |              |
    ┌─────▼─────┐ ┌────▼────┐  ┌────▼────┐    ┌────▼─────┐
    │   Auth    │ │  User   │  │ Channel │    │ Presence │
    │  Service  │ │ Service │  │ Service │    │ Service  │
    └─────┬─────┘ └────┬────┘  └────┬────┘    └────┬─────┘
          |            |            |              |
          └────────────┴────────────┴──────────────┘
                        |
                  PostgreSQL
                        
    ┌─────────────┬─────────────┬─────────────┐
    │    Chat     │   Voice     │   Stream    │
    │   Service   │   Service   │   Service   │
    └──────┬──────┴──────┬──────┴──────┬──────┘
           |             |             |
           └─────────────┴─────────────┘
                        |
                   Media Server
                   (WebRTC SFU)
```

## Service Responsibilities

### 1. Gateway Service
**Port:** 8080  
**Tech:** Axum, WebSocket, HTTP

The Gateway is the main entry point for all client connections.

**Responsibilities:**
- WebSocket connection management
- Client authentication
- Real-time event distribution
- REST API routing to backend services
- Rate limiting
- Connection health monitoring

**Data Flow:**
```
Client → Gateway → Auth Service (verify JWT)
Client ← Gateway ← Event Bus (broadcast events)
```

### 2. Auth Service
**Port:** 8081  
**Tech:** Axum, JWT, Argon2

Handles all authentication and authorization.

**Responsibilities:**
- User registration
- User login/logout
- JWT token generation
- Token validation
- Password hashing (Argon2)
- Refresh token management

**Security Features:**
- Argon2id password hashing
- JWT with RS256 signing
- Refresh token rotation
- Rate limiting on auth endpoints

### 3. User Service
**Port:** 8082  
**Tech:** Axum, PostgreSQL, Redis

Manages user profiles and relationships.

**Responsibilities:**
- User profile CRUD
- Friend system
- User search
- Avatar/banner management
- User settings
- Block list management

**Caching Strategy:**
- User profiles cached in Redis (TTL: 1 hour)
- Friend lists cached (TTL: 5 minutes)
- Cache invalidation on updates

### 4. Channel Service
**Port:** 8083  
**Tech:** Axum, PostgreSQL, Redis

Manages servers, channels, and permissions.

**Responsibilities:**
- Server (Guild) CRUD
- Channel CRUD
- Role management
- Permission calculations
- Member management
- Invite system
- Bans and moderation

**Permission Model:**
```rust
// Bitflag-based permissions
ADMINISTRATOR    = 1 << 3   // 8
MANAGE_CHANNELS  = 1 << 4   // 16
MANAGE_SERVER    = 1 << 5   // 32
SEND_MESSAGES    = 1 << 11  // 2048
```

### 5. Chat Service
**Port:** 8084  
**Tech:** Axum, PostgreSQL, Redis, NATS

Handles all text messaging.

**Responsibilities:**
- Send/receive messages
- Message history
- Message editing/deletion
- Reactions
- File attachments
- Mentions
- Message search

**Message Flow:**
```
Client → Gateway → Chat Service → Database
                               → NATS (broadcast)
                               → Other Gateways → Clients
```

**Storage:**
- Recent messages cached in Redis (last 50 per channel)
- Full history in PostgreSQL
- Attachments in MinIO

### 6. Voice Service
**Port:** 8085  
**Tech:** Axum, WebRTC, Redis, NATS

WebRTC signaling for voice communication.

**Responsibilities:**
- WebRTC signaling (SDP exchange)
- ICE candidate exchange
- Voice session management
- Mute/deaf state tracking
- Speaking indicators

**Voice Flow:**
```
Client A → Voice Service → Media Server ← Voice Service ← Client B
           (signaling)     (RTP media)     (signaling)
```

### 7. Stream Service
**Port:** 8086  
**Tech:** Axum, WebRTC, Redis, NATS

Screen sharing and video streaming.

**Responsibilities:**
- Screen share signaling
- Video stream management
- Quality adaptation
- Viewer management
- Stream recording

**Stream Quality Tiers:**
- High: 1080p @ 30fps, 2.5 Mbps
- Medium: 720p @ 30fps, 1.5 Mbps
- Low: 480p @ 24fps, 800 Kbps

### 8. Presence Service
**Port:** 8087  
**Tech:** Axum, Redis, NATS

Real-time presence tracking.

**Responsibilities:**
- Online/offline status
- Custom status messages
- Activity tracking
- Typing indicators
- Heartbeat management

**Presence Data:**
```redis
Key: presence:user:{user_id}
Type: Hash
TTL: 5 minutes (refreshed by heartbeat)
Fields:
  - status: online|idle|dnd|offline
  - custom_status: string
  - last_seen: timestamp
```

### 9. Media Server
**Port:** 8089  
**Tech:** WebRTC, RTP/RTCP

SFU (Selective Forwarding Unit) for media routing.

**Responsibilities:**
- RTP packet routing
- Audio mixing (optional)
- Bandwidth adaptation
- Simulcast support
- Media recording

**Media Processing:**
- Codec: Opus (audio), VP8/VP9 (video)
- Jitter buffer management
- Packet loss concealment
- Bandwidth estimation

## Data Storage

### PostgreSQL Schema

**Core Tables:**
- `users` - User accounts and authentication
- `servers` - Discord guilds
- `channels` - Text, voice, category channels
- `messages` - Chat messages
- `server_members` - Server membership
- `roles` - Permission roles
- `friendships` - Friend relationships
- `voice_sessions` - Voice channel participants
- `bans` - Banned users
- `audit_logs` - Moderation actions

**Indexes:**
- `idx_messages_channel` - Fast message queries
- `idx_users_username` - User search
- `idx_server_members` - Membership lookups
- Composite indexes for complex queries

### Redis Usage

**Cache:**
- User profiles
- Channel metadata
- Permission calculations
- Message history (recent)

**Pub/Sub:**
- Real-time events
- Typing indicators
- Presence updates

**Data Structures:**
- Sorted Sets: Typing indicators, leaderboards
- Hashes: User presence, cached objects
- Sets: Online users per server

### MinIO (Object Storage)

**Buckets:**
- `avatars` - User profile pictures
- `attachments` - Message file uploads
- `recordings` - Voice/stream recordings
- `emojis` - Custom emoji uploads

## Event-Driven Communication

### NATS Event Topics

```
auth.user.created
auth.user.deleted

user.profile.updated
user.friend.added

server.created
server.deleted
channel.created
channel.deleted

message.created
message.updated
message.deleted

voice.session.created
voice.session.ended
voice.speaking.started

stream.started
stream.stopped

presence.status.changed
presence.typing.started
```

### Event Flow Example: Send Message

```
1. Client sends message via WebSocket
   Client → Gateway

2. Gateway publishes to Chat Service
   Gateway → NATS → Chat Service

3. Chat Service saves to database
   Chat Service → PostgreSQL

4. Chat Service broadcasts event
   Chat Service → NATS → message.created

5. Gateway receives event and broadcasts
   NATS → Gateway → WebSocket → All connected clients
```

## WebRTC Architecture

### Voice Communication

```
Client A                                      Client B
   |                                             |
   |-- SDP Offer -------> Voice Service -------> |
   |<------- SDP Answer ------ Voice Service <---|
   |                                             |
   |<========== RTP Audio ==========> Media Server <=====> |
```

**Components:**
- **Voice Service**: Handles WebRTC signaling
- **Media Server**: Routes audio packets
- **TURN Server**: NAT traversal

**Codecs:**
- Audio: Opus @ 32-64 kbps
- Video: VP8/VP9 @ 500-2500 kbps

## Scaling Strategy

### Horizontal Scaling

**Stateless Services (scale freely):**
- Gateway Service (behind load balancer)
- Auth Service
- User Service
- Channel Service
- Chat Service

**Stateful Services (coordination required):**
- Presence Service (use Redis for shared state)
- Media Server (regional deployment)

### Database Scaling

**Read Replicas:**
```
PostgreSQL Primary (writes)
    ├── Replica 1 (reads)
    ├── Replica 2 (reads)
    └── Replica 3 (reads)
```

**Sharding Strategy:**
- Shard by `server_id` for server-specific data
- Keep user data on primary

### Redis Scaling

**Cluster Mode:**
```
Redis Cluster
    ├── Master 1 (hash slots 0-5461)
    ├── Master 2 (hash slots 5462-10922)
    └── Master 3 (hash slots 10923-16383)
Each with replicas
```

## Security

### Authentication Flow

```
1. User login with credentials
2. Auth Service validates
3. Generate JWT access token (1 hour)
4. Generate refresh token (7 days)
5. Client stores tokens
6. Access token in Authorization header
7. Refresh when expired
```

### Rate Limiting

Implemented at Gateway level:
- **Messages**: 10 per 10 seconds per user
- **API calls**: 60 per minute per user
- **Auth attempts**: 5 per minute per IP

### Data Encryption

- **In Transit**: TLS 1.3
- **At Rest**: PostgreSQL encryption
- **Passwords**: Argon2id
- **Tokens**: RS256 JWT

## Monitoring & Observability

### Metrics (Prometheus)

**Service Health:**
- `up{job="service_name"}` - Service availability
- `http_requests_total` - Request count
- `http_request_duration_seconds` - Latency

**Business Metrics:**
- `messages_sent_total` - Message throughput
- `active_websocket_connections` - Connected users
- `voice_sessions_active` - Voice channel usage

### Tracing

Using OpenTelemetry with distributed tracing:
```
Client Request → Gateway → Chat Service → Database
     [trace_id: abc123 across all services]
```

### Logging

Structured JSON logging:
```json
{
  "timestamp": "2024-01-01T12:00:00Z",
  "level": "info",
  "service": "chat-service",
  "trace_id": "abc123",
  "user_id": "uuid",
  "message": "Message sent"
}
```

## Deployment

### Docker Compose (Development)

```bash
docker-compose up -d
```

### Kubernetes (Production)

```yaml
# Example deployment
apiVersion: apps/v1
kind: Deployment
metadata:
  name: gateway-service
spec:
  replicas: 3
  selector:
    matchLabels:
      app: gateway
  template:
    spec:
      containers:
      - name: gateway
        image: discord-clone/gateway:latest
        ports:
        - containerPort: 8080
```

### Load Balancing

**Gateway:**
- Layer 4 load balancer (TCP)
- Sticky sessions for WebSocket
- Health check: `/health`

**Services:**
- Round-robin
- Service discovery via Kubernetes DNS

## Performance Considerations

### Optimization Techniques

1. **Database Connection Pooling**
   - SQLx connection pool (min: 5, max: 20)

2. **Caching Strategy**
   - Write-through cache for reads
   - Cache invalidation on updates

3. **Message Batching**
   - Batch insert messages (100ms window)

4. **WebSocket Compression**
   - Per-message deflate

5. **Media Optimization**
   - Simulcast for adaptive quality
   - Bandwidth estimation

### Capacity Planning

**Expected Load per Server:**
- Gateway: 10,000 concurrent WebSocket connections
- Chat Service: 1,000 messages/second
- Voice: 100 concurrent voice sessions
- Database: 10,000 queries/second

## Future Enhancements

- [ ] End-to-end encryption for DMs
- [ ] Message threads and forums
- [ ] Voice transcription
- [ ] AI-powered moderation
- [ ] Mobile push notifications
- [ ] Video calls (1-on-1 and group)
- [ ] Rich presence (games, Spotify)
- [ ] Bot API and webhooks
- [ ] Server discovery
- [ ] Nitro-like subscription features
