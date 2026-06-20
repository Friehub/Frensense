function redirect(next: string) {
    const allowed = ["/dashboard", "/home"];
    if (allowed.includes(next)) {
        res.redirect(next);
    } else {
        res.redirect("/dashboard");
    }
}
