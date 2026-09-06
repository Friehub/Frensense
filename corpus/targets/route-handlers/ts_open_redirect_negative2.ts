// SAFE: Validates redirect URL is relative and within the same origin
function redirect(next: string) {
  if (next.startsWith("/") && !next.startsWith("//")) {
    res.redirect(next);
  } else {
    res.redirect("/dashboard");
  }
}
