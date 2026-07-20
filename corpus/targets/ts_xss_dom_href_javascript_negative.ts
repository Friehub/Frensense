// SAFE: Validates the URL protocol before assigning to href or src
export function createUserLink(username: string) {
    const url = new URLSearchParams(location.search).get("url")!;
    const a = document.createElement("a");
    try {
        const parsed = new URL(url);
        if (parsed.protocol === "http:" || parsed.protocol === "https:") {
            a.href = parsed.href;
        } else {
            a.href = "/default";
        }
    } catch {
        a.href = "/default";
    }
    a.textContent = `Visit ${username}`;
    document.body.appendChild(a);
}

export function setAvatar() {
    const avatarUrl = new URLSearchParams(location.search).get("avatar")!;
    try {
        const parsed = new URL(avatarUrl);
        if (parsed.protocol === "http:" || parsed.protocol === "https:") {
            document.getElementById("avatar")!.setAttribute("src", parsed.href);
        }
    } catch {
        document.getElementById("avatar")!.setAttribute("src", "/default-avatar.png");
    }
}
