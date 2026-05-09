const path = require('path');

/**
 * Loading the native binding with a professional fall-back strategy.
 */
let native;
try {
  native = require('./gensense.node');
} catch (e) {
  // If local binary is missing, we try the built-in loader pattern (for multi-platform)
  try {
    native = require('./dist/gensense.node');
  } catch (err) {
    throw new Error(
      `[GenSense] Failed to load native binary. Please ensure the package was built correctly for your platform.\n` +
      `Error: ${err.message}`
    );
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
}

module.exports = {
  GenSense,
  // Expose the raw engine for advanced use-cases
  GenSenseEngine: native.GenSenseEngine
};
