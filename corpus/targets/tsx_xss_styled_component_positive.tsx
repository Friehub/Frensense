// [frensense]
// observation: A styled-components or CSS-in-JS library receives user input that is used in CSS prop values or template literals without sanitization, allowing CSS injection or script execution via CSS expressions.
// impact: An attacker can inject CSS that exfiltrates data via CSS selectors or triggers JavaScript in older browsers via expression().
// improvement: Sanitize user input used in CSS prop values, or avoid using dynamic user data in CSS-in-JS.
// cwe: CWE-79
// cvss: 6.1
// owasp: A03:2021
// severity: Medium
// runtime_probe: xss

import React from "react";
import styled from "styled-components";

const UserBox = styled.div<{ bgColor: string }>`
    background-color: ${props => props.bgColor};
    padding: 20px;
`;

export function UserCard({ user }: { user: { bannerColor: string; name: string } }) {
    return (
        <UserBox bgColor={user.bannerColor}>
            <h2>{user.name}</h2>
        </UserBox>
    );
}

const DynamicStyle = styled.div<{ userStyle: string }>`
    ${props => props.userStyle}
`;

export function CustomWidget({ styleString }: { styleString: string }) {
    return <DynamicStyle userStyle={styleString}>Widget</DynamicStyle>;
}
