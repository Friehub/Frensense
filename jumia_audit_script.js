const { GenSense } = require('./gensense.js');
const fs = require('fs');
const path = require('path');

async function runAudit() {
  console.log('--- Jumia-Clone Refactoring Audit (GenSense) ---');

  // Initialize engine
  const engine = new GenSense({
    environment: 'development'
  });

  // Note: The native engine loads default rules from the 'rules' directory relative to the binary or cwd.
  // We'll test if we can trigger the God Function rule in quality.yml first.
  
  const targetFiles = [
    '/home/oxisrael/Friehub/Taas/jumia-clone/packages/api/modules/catalog/services/catalog-service.ts',
    '/home/oxisrael/Friehub/Taas/jumia-clone/packages/api/modules/payment/services/payment-service.ts',
    '/home/oxisrael/Friehub/Taas/jumia-clone/packages/api/modules/revenue/services/ledger-service.ts'
  ];

  for (const file of targetFiles) {
    if (!fs.existsSync(file)) continue;
    
    console.log(`\nScanning ${path.basename(file)}...`);
    const findings = engine.auditPath(file);
    
    const qualityFindings = findings.filter(f => f.category === 'Quality' || f.category === 'Architecture');
    
    if (qualityFindings.length === 0) {
      console.log('✅ No quality issues found.');
    } else {
      qualityFindings.forEach(f => {
        console.log(`- [${f.severity}] ${f.ruleId}: ${f.observation}`);
        console.log(`  Impact: ${f.impact}`);
        console.log(`  Improvement: ${f.improvement}`);
      });
    }
  }
}

runAudit().catch(err => {
  console.error('Audit failed:', err);
  process.exit(1);
});
