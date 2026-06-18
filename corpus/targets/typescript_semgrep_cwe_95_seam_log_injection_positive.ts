// Vulnerable: Seam Logging API support an expression language to introduce bean property to log messages. The expression language can also be the source to unwanted code execution. In this context, an expression is built with a dynamic value. The source of the value(s) should be verified to avoid that unfiltered values fall into this risky code evaluation.
// Pattern: $LOG.$INFO($X + $Y,...)
function vulnerable() {
  // TODO: implement pattern match
}
