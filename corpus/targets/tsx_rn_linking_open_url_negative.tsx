// SAFE: Validates URL against an allowlist of allowed schemes and hostnames before opening
import { Linking, Button, Alert } from "react-native";

const ALLOWED_SCHEMES = new Set(["https", "mailto"]);
const ALLOWED_HOSTS = new Set(["example.com", "api.example.com"]);

interface LinkProps {
  userUrl: string;
}

function isValidUrl(url: string): boolean {
  try {
    const parsed = new URL(url);
    return ALLOWED_SCHEMES.has(parsed.protocol.replace(":", "")) &&
      ALLOWED_HOSTS.has(parsed.hostname);
  } catch {
    return false;
  }
}

export function OpenUserLink({ userUrl }: LinkProps) {
  const handlePress = () => {
    if (isValidUrl(userUrl)) {
      Linking.openURL(userUrl);
    } else {
      Alert.alert("Invalid URL");
    }
  };

  return <Button title="Open Link" onPress={handlePress} />;
}
