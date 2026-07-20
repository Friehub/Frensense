// SAFE: Checks if the URL can be opened via canOpenURL and restricts to https only
import { Linking, Button, Alert } from "react-native";

interface LinkProps {
  userUrl: string;
}

export function OpenUserLink({ userUrl }: LinkProps) {
  const handlePress = async () => {
    try {
      const parsed = new URL(userUrl);
      if (parsed.protocol !== "https:") {
        Alert.alert("Only HTTPS URLs are allowed");
        return;
      }
      const canOpen = await Linking.canOpenURL(userUrl);
      if (canOpen) {
        await Linking.openURL(userUrl);
      } else {
        Alert.alert("Cannot open URL");
      }
    } catch {
      Alert.alert("Invalid URL");
    }
  };

  return <Button title="Open Link" onPress={handlePress} />;
}
