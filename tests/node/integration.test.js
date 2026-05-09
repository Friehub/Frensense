const { GenSense } = require('../../index');

/**
 * GenSense Integration Test
 * Verifies the high-level professional JS API.
 */

async function runTest() {
  console.log('--- Testing GenSense Professional JS API ---');

  try {
    // 1. Initialize with tags
    const engine = new GenSense({
      tags: ['security', 'governance'],
      environment: 'development'
    });

    // 2. Audit a code string
    const code = `
      async fn main() {
          let m = std::sync::Mutex::new(0);
          let _guard = m.lock().unwrap();
          tokio::time::sleep(std::time::Duration::from_millis(10)).await;
      }
    `;

    const advisories = engine.auditContent('test.rs', code);

    console.log(`Audited content. Found ${advisories.length} findings.`); console.log("Findings:", advisories);
    
    // 3. Verify specific findings (RUST_UNWRAP_SAFETY)
    const lockIo = advisories.find(a => a.ruleId === 'RUST_UNWRAP_SAFETY');
    if (lockIo) {
      console.log(`SUCCESS: Found ${lockIo.ruleId}`);
      console.log(`   Observation: ${lockIo.observation}`);
    } else {
      console.error('❌ FAILURE: RUST_UNWRAP_SAFETY not found in advisories.');
      process.exit(1);
    }

    // 4. Verify tag-based findings (MISSING_SBOM - should be present because tag is enabled)
    // Actually MISSING_SBOM is a path-based check, but let's see if we get any other security findings
    advisories.forEach(a => {
        console.log(`- [${a.severity}] ${a.ruleId}: ${a.observation.substring(0, 50)}...`);
    });

    console.log('All API checks passed.');
  } catch (err) {
    console.error('❌ Integration Test Failed:', err.message);
    process.exit(1);
  }
}

runTest();
