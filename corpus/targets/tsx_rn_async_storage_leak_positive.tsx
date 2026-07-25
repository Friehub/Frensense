// [frensense]
// observation: React Native AsyncStorage stores user-provided PII (email, SSN) in plaintext without encryption.
// impact: Plaintext PII in AsyncStorage is accessible to any JavaScript running in the app (WebView, third-party SDKs) or extracted from device backups.
// improvement: Encrypt sensitive data before storing in AsyncStorage using react-native-encrypted-storage or a similar encryption library.
// cwe: CWE-200
// cvss: 5.3
// owasp: 
// severity: Medium

import AsyncStorage from "@react-native-async-storage/async-storage";

interface UserData {
  email: string;
  ssn: string;
  name: string;
}

export async function saveUserData(data: UserData) {
  await AsyncStorage.setItem("user_data", JSON.stringify(data));
}

export async function getUserData(): Promise<UserData | null> {
  const raw = await AsyncStorage.getItem("user_data");
  return raw ? JSON.parse(raw) : null;
}
