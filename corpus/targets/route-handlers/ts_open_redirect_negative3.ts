// SAFE: Redirects only to hardcoded safe paths
function redirect(next: string) {
    var safePaths = ['/dashboard', '/home', '/profile', '/settings'];
    if (safePaths.indexOf(next) !== -1) {
        res.redirect(next);
    } else {
        res.redirect('/dashboard');
    }
}
