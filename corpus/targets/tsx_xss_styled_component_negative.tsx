// SAFE: CSS values are validated and sanitized; only known-safe color values are allowed
import React from "react";
import styled from "styled-components";

const ALLOWED_COLORS = new Set(["red", "blue", "green", "yellow", "white", "black", "transparent"]);

const UserBox = styled.div<{ bgColor: string }>`
    background-color: ${props => props.bgColor};
    padding: 20px;
`;

export function UserCard({ user }: { user: { bannerColor: string; name: string } }) {
    const color = ALLOWED_COLORS.has(user.bannerColor) ? user.bannerColor : "white";
    return (
        <UserBox bgColor={color}>
            <h2>{user.name}</h2>
        </UserBox>
    );
}

export function CustomWidget({ styleString }: { styleString: string }) {
    return <div style={{}}>Widget</div>;
}
