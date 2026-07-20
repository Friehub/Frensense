// SAFE: Stores only non-sensitive data in AsyncStorage and encrypts PII fields separately
import AsyncStorage from "@react-native-async-storage/async-storage";
import EncryptedStorage from "react-native-encrypted-storage";

interface UserData {
  email: string;
  ssn: string;
  name: string;
}

export async function saveUserData(data: UserData) {
  const { ssn, email, ...safeData } = data;
  await AsyncStorage.setItem("user_profile", JSON.stringify(safeData));
  await EncryptedStorage.setItem("user_sensitive", JSON.stringify({ email, ssn }));
}

export async function getUserData(): Promise<UserData | null> {
  const safeRaw = await AsyncStorage.getItem("user_profile");
  const sensitiveRaw = await EncryptedStorage.getItem("user_sensitive");
  if (!safeRaw || !sensitiveRaw) return null;
  return { ...JSON.parse(safeRaw), ...JSON.parse(sensitiveRaw) };
}
