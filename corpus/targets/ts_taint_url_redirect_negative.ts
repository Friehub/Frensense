function redirect(req: Request) {
    const url = req.query.next;
    if (isAllowedRedirect(url)) {
        res.redirect(url);
    } else {
        res.redirect("/");
    }
}
