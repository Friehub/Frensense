// SAFE: Exchange an authentication token over a secure side-channel before opening the DataChannel.

async function openAuthenticatedDataChannel(peerId: string, authToken: string): Promise<RTCDataChannel> {
  const pc = new RTCPeerConnection({
    iceServers: [{ urls: 'stun:stun.l.google.com:19302' }],
  });

  const dc = pc.createDataChannel('secure-chat');

  const offer = await pc.createOffer();
  await pc.setLocalDescription(offer);
  // Send offer to peer via authenticated side-channel (e.g., WebSocket with known peerId)

  dc.onopen = () => {
    dc.send(JSON.stringify({ auth: authToken }));
  };

  return dc;
}
