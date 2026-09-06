const fs = require('fs');
const path = require('path');

// Mock function for LLM pattern generation
// In production, this would call an LLM API like OpenAI or Anthropic
// with a prompt asking for structural vulnerability patterns.
async function generateVulnerabilityPattern(cveDescription) {
    console.log(`[AGENT] Analyzing CVE: ${cveDescription}`);
    console.log(`[AGENT] Generating structural positive and negative patterns...`);
    
    // Simulate API delay
    await new Promise(resolve => setTimeout(resolve, 1500));
    
    return {
        positive: `function vulnerable(req, res) {\n    // Positive: Auto-generated pattern\n    const data = req.body.data;\n    db.collection('test').find({ payload: data }).toArray(cb);\n}`,
        negative: `function secure(req, res) {\n    // Negative: Auto-generated pattern\n    const data = String(req.body.data);\n    db.collection('test').find({ payload: data }).toArray(cb);\n}`
    };
}

async function main() {
    const args = process.argv.slice(2);
    if (args.length < 2) {
        console.error("Usage: node generate_corpus.js <vuln_name> <cve_description>");
        process.exit(1);
    }
    
    const vulnName = args[0];
    const cveDesc = args[1];
    
    const patterns = await generateVulnerabilityPattern(cveDesc);
    
    const targetDir = path.join(__dirname, '..', 'corpus', 'targets', 'nodejs_nosql');
    if (!fs.existsSync(targetDir)) {
        fs.mkdirSync(targetDir, { recursive: true });
    }
    
    const posPath = path.join(targetDir, `js_${vulnName}_positive.js`);
    const negPath = path.join(targetDir, `js_${vulnName}_negative.js`);
    
    fs.writeFileSync(posPath, patterns.positive);
    fs.writeFileSync(negPath, patterns.negative);
    
    console.log(`Successfully generated patterns:`);
    console.log(`  + ${posPath}`);
    console.log(`  - ${negPath}`);
}

main().catch(console.error);
