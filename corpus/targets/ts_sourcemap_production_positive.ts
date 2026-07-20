// [frensense]
// observation: Express serves the entire dist directory including .map files, exposing original TypeScript source code inline with JS bundles
// impact: Attackers can request the .map files to recover original source code including comments, internal API endpoints, authentication logic, and hardcoded test credentials
// improvement: Disable sourceMap in production tsconfig or build config, or configure express.static to deny .map files

import express from 'express';
import { join } from 'path';

function serveFrontend(app: express.Application): void {
  app.use(express.static(join(__dirname, '../../dist')));
}
