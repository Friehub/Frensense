// Vulnerable: OpenAI response content accessed without checking for refusal. The model may refuse requests, and accessing .content without checking .refusal first may lead to unexpected behavior. Check response.choices[0].message.refusal before accessing content.
// Pattern: $RESP.choices[0].message.content
function vulnerable() {
  // TODO: implement pattern match
}
