// Vulnerable: Using `Enum.map_join/3` is more efficient than using `Enum.map/2 |> Enum.join/2`.
// Pattern: {'pattern': 'Enum.join(Enum.map($E, $FUN), $JOINER)\n'} | {'pattern': 'Enum.map($E, $FUN)\n|> Enum.join($JOINER)\n'} | {'pattern': '$E\n|> Enum.map($FUN)\n|> Enum.join($JOINER)\n'}
function vulnerable() {
  // TODO: implement pattern match
}
