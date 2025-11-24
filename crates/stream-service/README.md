# Stream Service

Screen sharing and video streaming service for Discord Clone.

## Responsibilities

- WebRTC signaling for screen sharing
- Video stream session management
- Screen capture coordination
- Video quality adaptation
- Stream viewer management
- Recording capabilities

## API Endpoints

### Start Screen Share
```http
POST /stream/start
Authorization: Bearer <token>
Content-Type: application/json

{
  "channel_id": "uuid",
  "resolution": "1920x1080",
  "framerate": 30,
  "bitrate": 2500
}
```

Response:
```json
{
  "stream_id": "uuid",
  "session_id": "uuid",
  "server_endpoint": "stream.example.com:8001",
  "ice_servers": [ /* ICE servers */ ]
}
```

### Stop Screen Share
```http
POST /stream/stop
Authorization: Bearer <token>
Content-Type: application/json

{
  "stream_id": "uuid"
}
```

### Watch Stream
```http
POST /stream/watch
Authorization: Bearer <token>
Content-Type: application/json

{
  "stream_id": "uuid"
}
```

Response:
```json
{
  "session_id": "uuid",
  "sdp_offer": "v=0...",
  "ice_servers": [ /* ICE servers */ ]
}
```

### Update Stream Quality
```http
PATCH /stream/{stream_id}/quality
Authorization: Bearer <token>
Content-Type: application/json

{
  "resolution": "1280x720",
  "framerate": 30,
  "bitrate": 1500
}
```

## NATS Events

### Published Events
- `stream.started` - Screen share started
- `stream.stopped` - Screen share stopped
- `stream.viewer.joined` - New viewer joined
- `stream.viewer.left` - Viewer left
- `stream.quality.changed` - Quality settings updated

### Subscribed Events
- `voice.session.ended` - Stop associated stream
- `channel.deleted` - Stop all channel streams

## Stream Quality Presets

### High Quality (1080p)
- Resolution: 1920x1080
- Framerate: 30 FPS
- Bitrate: 2500 kbps

### Medium Quality (720p)
- Resolution: 1280x720
- Framerate: 30 FPS
- Bitrate: 1500 kbps

### Low Quality (480p)
- Resolution: 854x480
- Framerate: 24 FPS
- Bitrate: 800 kbps

## Environment Variables

```bash
REDIS_URL=redis://:password@localhost:6379
NATS_URL=nats://localhost:4222
MEDIA_SERVER_URL=http://localhost:8089
TURN_URL=turn:localhost:3478
PORT=8086
MAX_STREAMS_PER_CHANNEL=5
MAX_VIEWERS_PER_STREAM=50
```

## Running

```bash
cargo run --bin stream-service
```

Server starts on: http://localhost:8086
