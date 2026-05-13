const path = require('path');

/**
 * Loading the native binding with a professional fall-back strategy.
 */
let native;
try {
  native = require('./gensense.node');
} catch (e1) {
  try {
    native = require('./dist/gensense.node');
  } catch (e2) {
    // Platform-specific fallback (common in CI/CD environments)
    const fs = require('fs');
    const path = require('path');
    const rootFiles = fs.existsSync('.') ? fs.readdirSync('.').filter(f => f.endsWith('.node')) : [];
    const distFiles = fs.existsSync('./dist') ? fs.readdirSync('./dist').filter(f => f.endsWith('.node')) : [];
    
    const binary = rootFiles[0] || distFiles[0];
    if (binary) {
      try {
        native = require(path.join(rootFiles[0] ? '.' : './dist', binary));
      } catch (e3) {
        throw new Error(
          `[GenSense] Failed to load native binary '${binary}'.\n` +
          `Error: ${e3.message}`
        );
      }
    } else {
      throw new Error(
        `[GenSense] Failed to load native binary. Please ensure the package was built correctly for your platform.\n` +
        `Primary error: ${e1.message}\n` +
        `Secondary error: ${e2.message}\n` +
        `Available root files: ${rootFiles.join(', ') || 'none'}\n` +
        `Available dist files: ${distFiles.join(', ') || 'none'}`
      );
    }
  }
}

/**
 * GenSense is a high-precision semantic engine for auditing code.
 * This class provides a developer-centric wrapper around the native Rust core.
 */
class GenSense {
  /**
   * Initialize a new GenSense engine instance.
   * @param {Object} options Configuration options
   * @param {string[]} options.tags Optional tags to enable (e.g. ['sbom', 'security'])
   * @param {'production'|'staging'|'development'} options.environment The environment context
   */
  constructor(options = {}) {
    this.engine = new native.GenSenseEngine();
    
    if (options.environment) {
      this.engine.setEnvironment(options.environment);
    }
    
    if (options.tags && Array.isArray(options.tags)) {
      options.tags.forEach(tag => this.engine.enableTag(tag));
    }
  }

  /**
   * Audit a single code string.
   * @param {string} filePath Virtual file path (used for rule matching)
   * @param {string} content The source code content to audit
   * @returns {import('./index').JsAdvisory[]} Array of semantic findings
   */
  auditContent(filePath, content) {
    return this.engine.auditContent(filePath, content);
  }

  /**
   * Audit a project directory or a single file on disk.
   * @param {string} targetPath Absolute path to the file or directory
   * @returns {import('./index').JsAdvisory[]} Array of semantic findings
   */
  auditPath(targetPath) {
    const absolutePath = path.isAbsolute(targetPath) 
      ? targetPath 
      : path.resolve(process.cwd(), targetPath);
    return this.engine.auditPath(absolutePath);
  }

  /**
   * Audit an entire project directory, including cross-file project rules.
   * @param {string} rootDir Absolute path to the project root
   * @returns {import('./index').JsAdvisory[]} Array of semantic findings
   */
  auditProject(rootDir) {
    const absolutePath = path.isAbsolute(rootDir)
      ? rootDir
      : path.resolve(process.cwd(), rootDir);
    return this.engine.auditProject(absolutePath);
  }
}

module.exports = {
  GenSense,
  // Expose the raw engine for advanced use-cases
  GenSenseEngine: native.GenSenseEngine
};
