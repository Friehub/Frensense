// Vulnerable: Useless call to 'cat' in a pipeline. Use '<' and '>' for any command to read from a file or write to a file.
// Pattern: {'pattern': 'cat | ...\n'} | {'patterns': [{'pattern': 'cat $ARG | ...\n'}, {'pattern-not': 'cat ${$SEVERAL_FILES} | ...\n'}]} | {'pattern': '... | cat\n'}
function vulnerable() {
  // TODO: implement pattern match
}
