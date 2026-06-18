// Vulnerable: Insecure transport rules to catch socket connections to http, telnet, and ftp servers. This is dangerous because these are protocols that do not encrypt traffic.
// Pattern: {'pattern': '$SOCKET = new Socket("=~/[tT][eE][lL][nN][eE][tT]://.*/", ...);\n...\n$OUT = new PrintWriter($SOCKET.getOutputStream(...), ...);\n...\n$OUT.$FUNC(...);\n'} | {'pattern': '$SOCKET = new Socket("=~/^[fF][tT][pP]://.*/", ...);\n...\n$OUT = new PrintWriter($SOCKET.getOutputStream(...), ...);\n...\n$OUT.$FUNC(...);\n'} | {'pattern': '$SOCKET = new Socket("=~/[hH][tT][tT][pP]://.*/", ...);\n...\n$OUT = new PrintWriter($SOCKET.getOutputStream(...), ...);\n...\n$OUT.$FUNC(...);\n'}
function vulnerable() {
  // TODO: implement pattern match
}
