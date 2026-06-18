// Vulnerable: COPY instructions cannot copy from its own alias. The '$REF' alias is used before switching to a new image. If you meant to switch to a new image, include a new 'FROM' statement. Otherwise, remove the '--from=$REF' from the COPY statement.
// Pattern: {'pattern': 'FROM $IMAGE:$TAG as $REF\n...\nCOPY --from=$REF\n...\nFROM\n'} | {'pattern': 'FROM $IMAGE:$TAG AS $REF\n...\nCOPY --from=$REF\n...\nFROM\n'}
function vulnerable() {
  // TODO: implement pattern match
}
