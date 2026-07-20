// [frensense]
// observation: React Native Linking.openURL called with user-controlled URL string without validation, enabling SSRF and app scheme abuse.
// impact: Attacker can open arbitrary URLs including internal network URLs (SSRF), custom app schemes (tele://, tel://), or phishing pages.
// improvement: Validate the URL against an allowlist of schemes and hostnames before calling openURL.

import { Linking, Button } from "react-native";

interface LinkProps {
  userUrl: string;
}

export function OpenUserLink({ userUrl }: LinkProps) {
  const handlePress = () => {
    Linking.openURL(userUrl);
  };

  return <Button title="Open Link" onPress={handlePress} />;
}
