const { GenSense } = require('../../gensense.js');


/**
 * GenSense Integration Test
 * Verifies the high-level professional JS API.
 */

async function runTest() {
  console.log('--- Testing GenSense Professional JS API ---');

  try {
    // 1. Initialize with tags
    const engine = new GenSense({
      // tags: ['security', 'governance'], 
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
    
    // 3. Verify at least one finding was produced (catches broken NAPI builds)
    if (advisories.length === 0) {
      console.error('❌ FAILURE: No advisories returned from auditContent. Engine may be broken.');
      process.exit(1);
    }

    // 4. Verify specific findings (RUST_UNWRAP_SAFETY)
    const safety = advisories.find(a => a.ruleId === 'RUST_UNWRAP_SAFETY');
    if (safety) {
      console.log(`SUCCESS: Found ${safety.ruleId}`);
      console.log(`   Observation: ${safety.observation}`);
    } else {
      console.error('❌ FAILURE: RUST_UNWRAP_SAFETY not found in advisories.');
      process.exit(1);
    }

    // 4. Verify tag-based findings
    advisories.forEach(a => {
        console.log(`- [${a.severity}] ${a.ruleId}: ${a.observation.substring(0, 50)}...`);
    });

    // --- NEW: Edge Cases ---
    
    // 5. Empty code string
    console.log('\n--- Testing Empty Content ---');
    const emptyAdvisories = engine.auditContent('empty.rs', '');
    console.log(`Audited empty string. Found ${emptyAdvisories.length} findings.`);
    
    // 6. Large code string
    console.log('\n--- Testing Large Content ---');
    const largeCode = 'fn main() { ' + 'let x = 1; '.repeat(1000) + ' }';
    const largeAdvisories = engine.auditContent('large.rs', largeCode);
    console.log(`Audited 1000+ line string. Found ${largeAdvisories.length} findings.`);

    // 7. Version check
    console.log('\n--- Testing Version API ---');
    console.log(`Engine Version: ${engine.version}`);
    // Support v0.3.x and ensure it follows semantic versioning format
    const semverRegex = /^\d+\.\d+\.\d+(-[a-zA-Z0-9.]+)?$/;
    if (!semverRegex.test(engine.version) || !engine.version.startsWith('0.3')) {
      throw new Error(`Invalid or unexpected version reported: ${engine.version}`);
    }

    // 8. Test auditProject (Cross-file rules)
    console.log('\n--- Testing auditProject ---');
    const path = require('path');
    const fixturePath = path.resolve(__dirname, '../fixtures/project_with_guard_rule');
    const projectAdvisories = engine.auditProject(fixturePath);
    console.log(`Audited project. Found ${projectAdvisories.length} findings.`);
    
    const ruleIds = projectAdvisories.map(a => a.ruleId);
    if (ruleIds.includes('GUARD_CHECK')) {
      console.log('SUCCESS: Project rules fired via auditProject');
    } else {
      console.error('❌ FAILURE: GUARD_CHECK not found in project advisories.');
      process.exit(1);
    }

    // 9. Invalid Project Path
    console.log('\n--- Testing Invalid Project Path ---');
    try {
      engine.auditProject('./non_existent_path_xyz_123');
      console.error('❌ FAILURE: auditProject did not throw on non-existent path');
      process.exit(1);
    } catch (err) {
      console.log('SUCCESS: Caught expected error for invalid path');
    }

    console.log('\n✅ All API checks passed.');
  } catch (err) {
    console.error('❌ Integration Test Failed:', err.stack || err.message);
    process.exit(1);
  }
}

runTest();
