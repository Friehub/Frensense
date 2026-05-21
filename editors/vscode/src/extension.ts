import * as vscode from 'vscode';
import * as cp from 'child_process';
import * as path from 'path';
import * as fs from 'fs';
import * as os from 'os';

// ─── Types ────────────────────────────────────────────────────────────────────

interface GenSenseAdvisory {
    rule_id: string;
    severity: 'Critical' | 'Warning' | 'Info';
    observation: string;
    file_path: string;
    line: number;
    column: number;
    original_content: string;
    proposed_replacement?: string;
}

// ─── Constants ────────────────────────────────────────────────────────────────

const SEVERITY_ORDER: Record<string, number> = {
    Critical: 0,
    Warning: 1,
    Info: 2,
};

const DIAGNOSTICS_SOURCE = 'GenSense';

// ─── Activation ───────────────────────────────────────────────────────────────

let logger: vscode.OutputChannel;

// ─── Activation ───────────────────────────────────────────────────────────────

export function activate(context: vscode.ExtensionContext): void {
    logger = vscode.window.createOutputChannel('GenSense Engine');
    context.subscriptions.push(logger);

    logger.appendLine('[GenSense] Extension activated.');

    const collection = vscode.languages.createDiagnosticCollection('gensense');
    context.subscriptions.push(collection);

    // Run on save
    context.subscriptions.push(
        vscode.workspace.onDidSaveTextDocument((doc) => {
            const cfg = vscode.workspace.getConfiguration('gensense');
            if (cfg.get<boolean>('runOnSave', true)) {
                logger.appendLine(`[GenSense] Document saved: ${doc.uri.fsPath}`);
                scanDocument(doc, collection);
            }
        })
    );

    // Run on active editor change
    context.subscriptions.push(
        vscode.window.onDidChangeActiveTextEditor((editor) => {
            if (editor) {
                logger.appendLine(`[GenSense] Active editor changed: ${editor.document.uri.fsPath}`);
                scanDocument(editor.document, collection);
            }
        })
    );

    // Run scan command
    context.subscriptions.push(
        vscode.commands.registerCommand('gensense.runScan', async () => {
            const workspaceFolders = vscode.workspace.workspaceFolders;
            if (!workspaceFolders || workspaceFolders.length === 0) {
                vscode.window.showErrorMessage('GenSense: No workspace folder open.');
                return;
            }
            collection.clear();
            await vscode.window.withProgress(
                {
                    location: vscode.ProgressLocation.Notification,
                    title: 'GenSense: Scanning workspace...',
                    cancellable: false,
                },
                async () => {
                    for (const folder of workspaceFolders) {
                        logger.appendLine(`[GenSense] Running full scan on workspace: ${folder.uri.fsPath}`);
                        await scanPath(folder.uri.fsPath, collection);
                    }
                }
            );
        })
    );

    // Clear command
    context.subscriptions.push(
        vscode.commands.registerCommand('gensense.clearDiagnostics', () => {
            logger.appendLine('[GenSense] Clearing diagnostics.');
            collection.clear();
        })
    );

    // Scan the active document on startup
    const activeDoc = vscode.window.activeTextEditor?.document;
    if (activeDoc) {
        logger.appendLine(`[GenSense] Scanning active document on startup: ${activeDoc.uri.fsPath}`);
        scanDocument(activeDoc, collection);
    }
}

export function deactivate(): void {
    if (logger) {
        logger.dispose();
    }
}

// ─── Binary Resolution ────────────────────────────────────────────────────────

function resolveBinary(targetPath?: string): string | null {
    const cfg = vscode.workspace.getConfiguration('gensense');
    const configured = cfg.get<string>('binaryPath', '').trim();
    if (configured) {
        if (fs.existsSync(configured)) {
            logger.appendLine(`[GenSense] Using configured binary: ${configured}`);
            return configured;
        } else {
            logger.appendLine(`[GenSense] Configured binary path does not exist: ${configured}`);
        }
    }

    const isWin = os.platform() === 'win32';
    const binName = isWin ? 'gensense.exe' : 'gensense';

    // 1. Gather potential roots
    const roots = new Set<string>();
    const wsFolders = vscode.workspace.workspaceFolders;
    if (wsFolders) {
        for (const folder of wsFolders) {
            roots.add(folder.uri.fsPath);
            roots.add(path.join(folder.uri.fsPath, 'gensense'));
        }
    }

    // 2. Traversal upwards from active file
    if (targetPath) {
        let dir = path.dirname(targetPath);
        while (dir && dir !== path.parse(dir).root) {
            roots.add(dir);
            roots.add(path.join(dir, 'gensense'));
            dir = path.dirname(dir);
        }
    }

    // Log the roots searched
    logger.appendLine(`[GenSense] Searching for binary in roots: ${Array.from(roots).join(', ')}`);

    // 3. Check candidates in priority order
    for (const root of roots) {
        const candidates = [
            path.join(root, 'dist', 'binaries', platformBinName()),
            path.join(root, 'dist', binName),
            path.join(root, 'target', 'release', binName),
        ];
        for (const cand of candidates) {
            if (fs.existsSync(cand) && fs.statSync(cand).isFile()) {
                logger.appendLine(`[GenSense] Found binary: ${cand}`);
                return cand;
            }
        }
    }

    // 4. Fallback to global path lookup
    const globalBin = which('gensense');
    if (globalBin && fs.existsSync(globalBin)) {
        logger.appendLine(`[GenSense] Found global binary in PATH: ${globalBin}`);
        return globalBin;
    }

    logger.appendLine('[GenSense] Could not locate binary in workspace or PATH.');
    return null;
}

function platformBinName(): string {
    const p = os.platform();
    const a = os.arch();
    const isWin = p === 'win32';
    if (p === 'linux' && a === 'x64') return 'gensense-linux-x64';
    if (p === 'darwin' && a === 'x64') return 'gensense-macos-x64';
    if (p === 'darwin' && a === 'arm64') return 'gensense-macos-arm64';
    if (isWin && a === 'x64') return 'gensense-windows-x64.exe';
    return 'gensense';
}

function which(name: string): string | null {
    try {
        const result = cp.execSync(
            os.platform() === 'win32' ? `where ${name}` : `which ${name}`,
            { encoding: 'utf8', stdio: ['pipe', 'pipe', 'ignore'] }
        ).trim().split('\n')[0];
        return result || null;
    } catch {
        return null;
    }
}

// ─── Scanning ─────────────────────────────────────────────────────────────────

function scanDocument(
    doc: vscode.TextDocument,
    collection: vscode.DiagnosticCollection
): void {
    const supported = ['rust', 'typescript', 'javascript', 'solidity'];
    if (!supported.includes(doc.languageId)) {
        logger.appendLine(`[GenSense] Language not supported: ${doc.languageId}`);
        return;
    }
    scanPath(doc.uri.fsPath, collection);
}

async function scanPath(
    targetPath: string,
    collection: vscode.DiagnosticCollection
): Promise<void> {
    const binary = resolveBinary(targetPath);
    if (!binary) {
        vscode.window.showWarningMessage(
            'GenSense: Binary not found. Set `gensense.binaryPath` in settings or build the project first.'
        );
        return;
    }

    const cfg = vscode.workspace.getConfiguration('gensense');
    const minSeverity = cfg.get<string>('minSeverity', 'Warning');
    const minLevel = SEVERITY_ORDER[minSeverity] ?? 1;

    logger.appendLine(`[GenSense] Executing command: "${binary}" "${targetPath}" --json`);

    return new Promise((resolve) => {
        let stdout = '';
        let stderr = '';

        const child = cp.spawn(binary, [targetPath, '--json'], { stdio: 'pipe' });

        child.stdout.on('data', (d: Buffer) => { stdout += d.toString(); });
        child.stderr.on('data', (d: Buffer) => { stderr += d.toString(); });

        child.on('close', (code) => {
            logger.appendLine(`[GenSense] Process exited with code ${code}`);
            try {
                const advisories: GenSenseAdvisory[] = JSON.parse(stdout);
                logger.appendLine(`[GenSense] Parsed ${advisories.length} advisories.`);
                applyDiagnostics(advisories, minLevel, collection);
            } catch (err) {
                logger.appendLine(`[GenSense] Error parsing JSON output: ${(err as Error).message}`);
                logger.appendLine(`[GenSense] Raw stdout: ${stdout}`);
                if (stderr.trim()) {
                    logger.appendLine(`[GenSense] Raw stderr: ${stderr}`);
                }
            }
            resolve();
        });

        child.on('error', (err) => {
            logger.appendLine(`[GenSense] Child process error: ${err.message}`);
            vscode.window.showErrorMessage(`GenSense: Failed to launch binary — ${err.message}`);
            resolve();
        });
    });
}

// ─── Diagnostics ──────────────────────────────────────────────────────────────

function applyDiagnostics(
    advisories: GenSenseAdvisory[],
    minLevel: number,
    collection: vscode.DiagnosticCollection
): void {
    const byFile = new Map<string, vscode.Diagnostic[]>();

    for (const advisory of advisories) {
        if ((SEVERITY_ORDER[advisory.severity] ?? 99) > minLevel) {
            continue;
        }

        let filePath = advisory.file_path;
        if (!filePath) {
            // Project-level or global advisory; attach to the workspace root's Cargo.toml or package.json
            const ws = vscode.workspace.workspaceFolders?.[0]?.uri.fsPath;
            if (ws) {
                const cargoPath = path.join(ws, 'Cargo.toml');
                const pkgPath = path.join(ws, 'package.json');
                if (fs.existsSync(cargoPath)) filePath = cargoPath;
                else if (fs.existsSync(pkgPath)) filePath = pkgPath;
                else continue;
            } else {
                continue;
            }
        }

        let uri: vscode.Uri;
        try {
            uri = vscode.Uri.file(filePath);
        } catch {
            continue;
        }

        const line = Math.max(0, advisory.line - 1);
        const col = Math.max(0, advisory.column - 1);
        const range = new vscode.Range(line, col, line, col + Math.max(1, advisory.original_content.length));

        const diag = new vscode.Diagnostic(
            range,
            `[${advisory.rule_id}] ${advisory.observation}`,
            severityToVscode(advisory.severity)
        );
        diag.source = DIAGNOSTICS_SOURCE;
        if (advisory.proposed_replacement) {
            diag.code = { value: 'fix-available', target: uri };
        }

        const key = uri.fsPath;
        if (!byFile.has(key)) byFile.set(key, []);
        byFile.get(key)!.push(diag);
    }

    // Clear existing for files that were checked, then apply new ones
    for (const [filePath, diags] of byFile) {
        logger.appendLine(`[GenSense] Applying ${diags.length} diagnostics to ${filePath}`);
        collection.set(vscode.Uri.file(filePath), diags);
    }
}

function severityToVscode(severity: string): vscode.DiagnosticSeverity {
    switch (severity) {
        case 'Critical':
            return vscode.DiagnosticSeverity.Error;
        case 'Warning':
            return vscode.DiagnosticSeverity.Warning;
        case 'Info':
        default:
            return vscode.DiagnosticSeverity.Information;
    }
}
