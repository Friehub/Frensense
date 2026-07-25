// [frensense]
// observation: A WebRTC DataChannel is opened after an SDP exchange without any peer identity verification. The code does not check the remote peer's DTLS fingerprint or any authentication token before sending sensitive data.
// impact: An attacker who establishes a WebRTC connection (e.g., via STUN reflection or MITM) can receive sensitive data sent over the DataChannel. Without peer authentication, the channel is effectively anonymous.
// improvement: Verify the remote peer's identity via DTLS fingerprint verification (`RTCPeerConnection.getRemoteCertificates()`) or exchange an authentication token over a secure side-channel before sending data.
// cwe: CWE-287
// cvss: 9.8
// owasp: A07:2021
// severity: Critical

const pc = new RTCPeerConnection({
  iceServers: [{ urls: 'stun:stun.l.google.com:19302' }],
});

const dc = pc.createDataChannel('chat');

dc.onopen = () => {
  dc.send(JSON.stringify({ token: localStorage.getItem('authToken') }));
};

pc.createOffer().then((offer) => pc.setLocalDescription(offer));
