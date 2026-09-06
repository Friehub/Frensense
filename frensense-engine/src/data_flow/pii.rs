// SPDX-License-Identifier: MIT

pub fn is_pii_field(name: &str) -> bool {
    let lower = name.to_lowercase();
    matches!(
        lower.as_str(),
        // ContactInfo
        "email" | "emailaddress" | "email_address" | "useremail" |
        "phone" | "phonenumber" | "phone_number" | "mobile" | "mobilenumber" | "cell" |
        // Credentials
        "password" | "passwordhash" | "password_hash" | "hashedpassword" | "hashed_password" | "passworddigest" |
        "apikey" | "api_key" | "secretkey" | "secret_key" | "accesstoken" | "access_token" | "refreshtoken" | "refresh_token" | "privatekey" | "private_key" |
        // GovernmentId
        "ssn" | "socialsecuritynumber" | "social_security_number" | "taxid" | "tax_id" | "nin" | "tfn" |
        "passportnumber" | "passport_number" | "driverlicense" | "driver_license" | "licensenumber" |
        // PaymentInfo
        "creditcard" | "credit_card" | "cardnumber" | "card_number" | "pan" | "cvv" | "cvc" | "expirydate" |
        // Demographics
        "dateofbirth" | "date_of_birth" | "dob" | "birthdate" | "birth_date" |
        // Location
        "address" | "streetaddress" | "street_address" | "homeaddress" | "billingaddress" | "shippingaddress" |
        // Network
        "ipaddress" | "ip_address" | "userip" | "user_ip" | "clientip" | "client_ip" |
        // Medical
        "medicalrecord" | "medical_record" | "diagnosis" | "healthcondition" | "health_condition" | "prescription" |
        // Financial
        "bankaccount" | "bank_account" | "accountnumber" | "account_number" | "routingnumber" | "routing_number" | "iban" | "swift"
    )
}
