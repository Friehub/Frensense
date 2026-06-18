// Vulnerable: 'wget' and 'curl' are similar tools. Choose one and do not install the other to decrease image size.
// Pattern: {'pattern': 'RUN wget ...\n...\nRUN curl ...\n'} | {'pattern': 'RUN curl ...\n...\nRUN wget ...\n'}
function vulnerable() {
  // TODO: implement pattern match
}
