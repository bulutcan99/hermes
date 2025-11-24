# Voice Service

WebRTC-based voice communication service for Discord Clone.

## Responsibilities

- WebRTC signaling for voice channels
- Voice session management
- Audio routing coordination
- Mute/unmute state tracking
- Speaking indicators
- Voice channel participant management

## WebRTC Flow

```
Client A                Voice Service              Media Server
   |                          |                          |
   |--- Join Voice ---------> |                          |
   |<-- ICE Candidates ---    |                          |
   |--- SDP Offer ---------> |                          |
   |                          |--- Create Session -----> |
   |<-- SDP Answer -------    |<-- Session Info ------   |
   |                          |                          |
   |<=== RTP Audio Stream ===========================>   |
   |                          |                          |
```

## API Endpoints

### Join Voice Channel
```http
POST /voice/join
Authorization: Bearer <token>
Content-Type: application/json

{
  "channel_id": "uuid",
  "mute": false,
  "deaf": false
}
```

Response:
```json
{
  "session_id": "uuid",
  "server_endpoint": "media.example.com:8000",
  "ice_servers": [
    {
      "urls": ["stun:turn.example.com:3478"],
      "username": "discord",
      "credential": "password"
    }
  ]
}
```

### Update Voice State
```http
PATCH /voice/state
Authorization: Bearer <token>
Content-Type: application/json

{
  "session_id": "uuid",
  "mute": true,
  "deaf": false
}
```

### Leave Voice Channel
```http
POST /voice/leave
Authorization: Bearer <token>
Content-Type: application/json

{
  "session_id": "uuid"
}
```

### WebRTC Signaling
```http
POST /voice/signal
Authorization: Bearer <token>
Content-Type: application/json

{
  "session_id": "uuid",
  "type": "offer|answer|ice",
  "sdp": "v=0...",
  "candidate": { /* ICE candidate */ }
}
```

## NATS Events

### Published Events
- `voice.session.created` - User joined voice
- `voice.session.updated` - Voice state changed
- `voice.session.ended` - User left voice
- `voice.speaking.started` - User started speaking
- `voice.speaking.stopped` - User stopped speaking

### Subscribed Events
- `channel.deleted` - End all voice sessions
- `user.banned` - Disconnect user

## Environment Variables

```bash
DATABASE_URL=postgres://discord:password@localhost:5432/discord
REDIS_URL=redis://:password@localhost:6379
NATS_URL=nats://localhost:4222
MEDIA_SERVER_URL=http://localhost:8089
TURN_URL=turn:localhost:3478
TURN_USERNAME=discord
TURN_PASSWORD=discord_turn_password
PORT=8085
```

## Running

```bash
cargo run --bin voice-service
```

Server starts on: http://localhost:8085
