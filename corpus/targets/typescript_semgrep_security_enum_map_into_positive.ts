// Vulnerable: Using `Enum.into/3` is more efficient than using `Enum.map/2 |> Enum.into/2`.
// Pattern: {'pattern': 'Enum.into(Enum.map($E, $FUN), $INTO)\n'} | {'pattern': 'Enum.map($E, $FUN)\n|> Enum.into($INTO)\n'} | {'pattern': '$E\n|> Enum.map($FUN)\n|> Enum.into($INTO)\n'}
function vulnerable() {
  // TODO: implement pattern match
}
