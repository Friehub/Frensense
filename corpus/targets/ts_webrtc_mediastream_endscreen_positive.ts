// [frensense]
// observation: `getDisplayMedia()` is called without requiring a preceding user gesture. While browsers require a user gesture for this API, the code can be invoked from an event handler that is not user-initiated (e.g., a timer or WebSocket message), which silently fails or leaks screen content depending on browser behavior.
// impact: An attacker can trick the user or silently screen-share when the call succeeds (e.g., via an auto-playing video or social engineering). The user's screen content — including open tabs, credentials, and private data — is exposed to the attacker.
// improvement: Always call `getDisplayMedia()` directly inside a user-initiated event handler (click, keypress). Never wrap it in async delays or invoke it from non-user events.

async function startScreenShare(): Promise<void> {
  const stream = await navigator.mediaDevices.getDisplayMedia({
    video: true,
    audio: false,
  });
  const videoElement = document.getElementById('preview') as HTMLVideoElement;
  videoElement.srcObject = stream;
}

// Called from setTimeout, not from user gesture
setTimeout(() => {
  startScreenShare();
}, 100);
