// Vulnerable: Using '--platform' with FROM restricts the image to build on a single platform. Further, this must be the same as the build platform. If you intended to specify the target platform, use the utility 'docker buildx --platform=' instead.
// Pattern: FROM --platform=$PLATFORM $IMAGE
function vulnerable() {
  // TODO: implement pattern match
}
