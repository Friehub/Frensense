// SAFE: Uses inline style objects instead of template literals; CSS property value is restricted to a safe set
import React from "react";

const ALLOWED_BG = ["#fff", "#f0f0f0", "#e0e0e0", "#333", "#000"];

export function UserCard({ user }: { user: { bannerColor: string; name: string } }) {
    const bgColor = ALLOWED_BG.includes(user.bannerColor) ? user.bannerColor : "#fff";
    return (
        <div style={{ backgroundColor: bgColor, padding: "20px" }}>
            <h2>{user.name}</h2>
        </div>
    );
}

export function CustomWidget({ styleString }: { styleString: string }) {
    return <div style={{}}>Widget</div>;
}
