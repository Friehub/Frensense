// SAFE: Check for user activation before calling getDisplayMedia and fall back gracefully.

function isUserActivated(): boolean {
  return navigator.userActivation?.hasBeenActive === true;
}

async function startScreenShareSafe(): Promise<MediaStream | null> {
  if (!isUserActivated()) {
    console.warn('Screen share requires a user gesture');
    return null;
  }
  try {
    return navigator.mediaDevices.getDisplayMedia({
      video: { displaySurface: 'monitor' },
      audio: false,
    });
  } catch (err) {
    console.error('Screen share denied:', err);
    return null;
  }
}
