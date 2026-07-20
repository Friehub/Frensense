// SAFE: Uses react-native-encrypted-storage to encrypt PII before persisting
import EncryptedStorage from "react-native-encrypted-storage";

interface UserData {
  email: string;
  ssn: string;
  name: string;
}

export async function saveUserData(data: UserData) {
  await EncryptedStorage.setItem("user_data", JSON.stringify(data));
}

export async function getUserData(): Promise<UserData | null> {
  const raw = await EncryptedStorage.getItem("user_data");
  return raw ? JSON.parse(raw) : null;
}
