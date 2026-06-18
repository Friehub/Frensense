    one_or_more(|input| fws(input).or_else(|| comment(input)))(input)
fn comment(mut input: &[u8]) -> Option<ParsedItem<'_, ()>> {
    while let Some(rest) = ccontent(input) {