const { auditContent } = require('./index');

const code = `
async fn leaky() {
    let m = std::sync::Mutex::new(0);
    let _g = m.lock().unwrap();
    tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
}
`;

console.log('--- Testing GenSense Node.js Bindings ---');
const results = auditContent('test.rs', code);

console.log(`Found ${results.length} advisories.`);

results.forEach(adv => {
  console.log(`[${adv.severity}] ${adv.ruleId}: ${adv.observation}`);
  console.log(`  Location: ${adv.filePath}:${adv.line}:${adv.column}`);
});

if (results.length > 0) {
  console.log('✅ Test Passed');
} else {
  console.log('❌ Test Failed: No advisories found for known-bad code.');
  process.exit(1);
}
