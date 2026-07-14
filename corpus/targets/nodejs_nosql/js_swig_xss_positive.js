function renderProfile(req, res) {
    const profileData = req.body.profile;
    // Positive: Unsafe data passed to template rendering
    res.render('profile', { userProfile: profileData });
}

function displayError(req, res) {
    const errorMsg = req.query.msg;
    // Positive: Reflected XSS through template engine
    res.render('error', { message: errorMsg });
}
