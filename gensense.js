const path = require('path');

/**
 * Loading the native binding with a professional fall-back strategy.
 */
let native;
try {
  // 1. Try primary location (local or pre-built)
  native = require('./gensense.node');
} catch (e1) {
  try {
    // 2. Try distribution location
    native = require('./dist/gensense.node');
  } catch (e2) {
    try {
      // 3. Try NAPI-RS naming convention (platform-prefixed)
      const platform = process.platform;
      const arch = process.arch;
      const napiName = `./gensense.${platform}-${arch}.node`;
      native = require(napiName);
    } catch (e3) {
      // 4. Fallback: Search the directory for ANY .node file
      const fs = require('fs');
      const searchDirs = ['.', './dist', './binaries'];
      let found = false;
      
      for (const dir of searchDirs) {
        if (fs.existsSync(dir)) {
          const files = fs.readdirSync(dir).filter(f => f.endsWith('.node'));
          if (files.length > 0) {
            try {
              native = require(path.resolve(dir, files[0]));
              found = true;
              break;
            } catch (e4) {
              // Continue searching
            }
          }
        }
      }

      if (!found) {
        throw new Error(
          `[GenSense] Critical Failure: Could not load native semantic engine.\n` +
          `Tried: ./gensense.node, ./dist/gensense.node, and platform-specific binaries.\n` +
          `Error Details:\n` +
          `- Local: ${e1.message}\n` +
          `- Dist: ${e2.message}\n` +
          `- Platform: ${e3.message}\n` +
          `Please ensure '@friehub/gensense' was installed correctly for your platform (${process.platform}-${process.arch}).`
        );
      }
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
   * The current version of the GenSense engine.
   * @type {string}
   */
  get version() {
    return this.engine.version;
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
