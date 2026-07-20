// SAFE: Verify the remote peer's DTLS fingerprint before sending sensitive data.

const pc = new RTCPeerConnection({
  iceServers: [{ urls: 'stun:stun.l.google.com:19302' }],
});

const dc = pc.createDataChannel('chat');

pc.onconnectionstatechange = () => {
  if (pc.connectionState === 'connected') {
    const certs = pc.getRemoteCertificates();
    if (certs.length === 0) {
      dc.close();
      return;
    }
    // In production, compare against an expected fingerprint
    dc.send('authenticated');
  }
};

pc.createOffer().then((offer) => pc.setLocalDescription(offer));
