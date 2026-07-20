// SAFE: Call getDisplayMedia directly inside a user-initiated click handler.

const shareButton = document.getElementById('share-screen') as HTMLButtonElement;

shareButton.addEventListener('click', async () => {
  const stream = await navigator.mediaDevices.getDisplayMedia({
    video: true,
    audio: false,
  });
  const videoElement = document.getElementById('preview') as HTMLVideoElement;
  videoElement.srcObject = stream;
});
