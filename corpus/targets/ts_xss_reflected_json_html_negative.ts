// SAFE: Client-side rendering uses textContent for text and setAttribute for URLs, avoiding HTML injection
export function renderProfile() {
    fetch("/api/user/1")
        .then(r => r.json())
        .then(user => {
            const el = document.getElementById("profile")!;
            const h2 = document.createElement("h2");
            h2.textContent = user.name;
            const p = document.createElement("p");
            p.textContent = user.bio;
            const a = document.createElement("a");
            a.href = user.website;
            a.textContent = "Website";
            el.append(h2, p, a);
        });
}
