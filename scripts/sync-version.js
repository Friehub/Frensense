const fs = require('fs');
const path = require('path');

/**
 * Syncs the version from package.json to Cargo.toml
 */
function syncVersion() {
  const pkgPath = path.join(__dirname, '..', 'package.json');
  const cargoPath = path.join(__dirname, '..', 'Cargo.toml');

  if (!fs.existsSync(pkgPath) || !fs.existsSync(cargoPath)) {
    console.error('Error: package.json or Cargo.toml not found');
    process.exit(1);
  }

  const pkg = JSON.parse(fs.readFileSync(pkgPath, 'utf8'));
  const version = pkg.version;

  let cargo = fs.readFileSync(cargoPath, 'utf8');
  
  // Replace version = "x.y.z" in [package] section
  const newCargo = cargo.replace(
    /^version\s*=\s*".*"/m,
    `version = "${version}"`
  );

  if (cargo !== newCargo) {
    fs.writeFileSync(cargoPath, newCargo);
    console.log(`Successfully synced version ${version} to Cargo.toml`);
  } else {
    console.log(`Version ${version} is already in sync`);
  }
}

syncVersion();
