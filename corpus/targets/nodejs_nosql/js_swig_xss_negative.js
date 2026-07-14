function renderProfile(req, res) {
    const profileData = escapeHtml(req.body.profile);
    // Negative: Data is escaped before rendering
    res.render('profile', { userProfile: profileData });
}

function displayError(req, res) {
    const errorMsg = "A standard error occurred.";
    // Negative: Rendering a static string
    res.render('error', { message: errorMsg });
}
