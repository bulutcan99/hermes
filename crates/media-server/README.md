# Media Server

WebRTC media routing and processing server for Discord Clone.

## Responsibilities

- RTP/RTCP packet routing
- Audio mixing for voice channels
- Video forwarding for streams
- Bandwidth adaptation
- Simulcast/SVC support
- Recording capabilities
- Media quality monitoring

## Architecture

The Media Server acts as an SFU (Selective Forwarding Unit) for both audio and video:

```
User A ─────RTP────▶ Media Server ─────RTP────▶ User B
                          │
                          └─────RTP────▶ User C
```

## Features

### Audio Processing
- Opus codec support
- Audio mixing for voice channels
- Voice activity detection (VAD)
- Automatic gain control (AGC)
- Noise suppression
- Echo cancellation

### Video Processing
- VP8/VP9/H.264 codec support
- Simulcast (multiple quality layers)
- SVC (Scalable Video Coding)
- Adaptive bitrate
- Bandwidth estimation

## API Endpoints

### Create Session
```http
POST /sessions
Content-Type: application/json

{
  "type": "audio" | "video",
  "channel_id": "uuid",
  "user_id": "uuid"
}
```

Response:
```json
{
  "session_id": "uuid",
  "ice_servers": [
    {
      "urls": ["turn:turn.example.com:3478"],
      "username": "discord",
      "credential": "password"
    }
  ],
  "dtls_parameters": { /* DTLS params */ }
}
```

### Publish Track
```http
POST /sessions/{session_id}/publish
Content-Type: application/json

{
  "kind": "audio" | "video",
  "rtp_parameters": { /* RTP params */ }
}
```

### Subscribe to Track
```http
POST /sessions/{session_id}/subscribe
Content-Type: application/json

{
  "track_id": "uuid",
  "rtp_capabilities": { /* RTP capabilities */ }
}
```

### Close Session
```http
DELETE /sessions/{session_id}
```

### Session Stats
```http
GET /sessions/{session_id}/stats
```

Response:
```json
{
  "session_id": "uuid",
  "duration": 120,
  "tracks": [
    {
      "track_id": "uuid",
      "kind": "audio",
      "bitrate": 64000,
      "packets_sent": 12000,
      "packets_lost": 5,
      "jitter": 0.02
    }
  ]
}
```

## Media Quality Presets

### Audio
- **Low**: 16 kbps mono (voice only)
- **Medium**: 32 kbps mono (default)
- **High**: 64 kbps stereo (music mode)

### Video
- **Low**: 320x180 @ 15fps, 200 kbps
- **Medium**: 640x360 @ 24fps, 500 kbps
- **High**: 1280x720 @ 30fps, 1500 kbps
- **Ultra**: 1920x1080 @ 30fps, 2500 kbps

## NATS Events

### Published Events
- `media.session.created` - New media session
- `media.session.closed` - Session ended
- `media.track.added` - New track published
- `media.track.removed` - Track unpublished
- `media.quality.degraded` - Network issues detected

### Subscribed Events
- `voice.session.ended` - Clean up audio session
- `stream.stopped` - Clean up video session

## Bandwidth Management

The media server automatically adapts quality based on:
- Available bandwidth
- Packet loss rate
- Round-trip time (RTT)
- CPU usage

## Recording

Enable recording for channels:

```http
POST /channels/{channel_id}/recording/start
Content-Type: application/json

{
  "format": "webm",
  "audio_bitrate": 128000,
  "video_bitrate": 2500000
}
```

Recordings are stored in MinIO and can be accessed later.

## Environment Variables

```bash
REDIS_URL=redis://:password@localhost:6379
NATS_URL=nats://localhost:4222
TURN_URL=turn:localhost:3478
TURN_USERNAME=discord
TURN_PASSWORD=discord_turn_password
PORT=8089
RTC_MIN_PORT=50000
RTC_MAX_PORT=60000
MAX_SESSIONS=1000
ENABLE_RECORDING=false
RECORDING_STORAGE=minio://localhost:9000
```

## Performance Tuning

### Recommended Server Specs
- CPU: 8+ cores (for mixing/transcoding)
- RAM: 16GB minimum
- Network: 1 Gbps
- Concurrent sessions: ~100 per core

### Scaling
Deploy multiple media servers and use DNS/load balancer for distribution:

```
Voice Service ──▶ media-1.example.com (US East)
             └──▶ media-2.example.com (EU West)
                 └──▶ media-3.example.com (Asia)
```

## Running

```bash
cargo run --bin media-server --release
```

Server starts on: http://localhost:8089

## Monitoring

Key metrics to track:
- Active sessions
- Total bandwidth usage
- Packet loss rate
- Average latency
- CPU/Memory usage
