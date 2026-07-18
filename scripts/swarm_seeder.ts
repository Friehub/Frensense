import * as fs from 'fs';
import * as path from 'path';
import { exec } from 'child_process';
import { promisify } from 'util';

// TODO: Run `npm install @google/genai` to use this
import { GoogleGenAI } from '@google/genai';

const execAsync = promisify(exec);

// Initialize Gemini client
// Make sure to export GEMINI_API_KEY="your-key" in your terminal before running
const gemini = new GoogleGenAI({
  apiKey: process.env.GEMINI_API_KEY,
});

// --- Configuration ---
const TAXONOMY_FILES = [
  '../frensense-4000-bug-taxonomy.md',
  '../frensense-10000-extended-taxonomy.md'
];
const CORPUS_DIR = '../corpus/targets';
const CONCURRENCY = 16;
const MAX_RETRIES = 3;

// --- Interfaces ---
interface Task {
  patternId: string;
  language: 'typescript' | 'rust';
  description: string;
  mutation: string;
  contextImport: string;
}

interface LLMResponse {
  positiveCode: string;
  negativeCode: string;
  negative2Code: string;
}

// --- Taxonomy Parsing ---
function parseTaxonomies(): Task[] {
  const tasks: Task[] = [];
  
  for (const file of TAXONOMY_FILES) {
    const content = fs.readFileSync(path.join(__dirname, file), 'utf-8');
    const lines = content.split('\n');
    
    let currentCategory = '';
    
    for (const line of lines) {
      if (line.startsWith('### Category')) {
        currentCategory = line.replace('###', '').trim();
      } else if (line.startsWith('| `') || line.startsWith('| **`')) {
        const parts = line.split('|').map(p => p.trim());
        if (parts.length >= 5) {
          const rawId = parts[1].replace(/[`*]/g, '');
          const desc = parts[2];
          const mutationsStr = parts[3];
          const langStr = parts[4];
          
          if (line.includes('✅')) continue;

          let language: 'typescript' | 'rust' = 'typescript';
          if (langStr.toLowerCase().includes('rust')) {
            language = 'rust';
          }
          
          let mutationCount = 1;
          const match = mutationsStr.match(/\[(\d+)\]/);
          if (match) {
            mutationCount = parseInt(match[1], 10);
          } else if (mutationsStr.includes('M1')) {
            mutationCount = 5;
          }

          for (let i = 1; i <= mutationCount; i++) {
            tasks.push({
              patternId: rawId,
              language,
              description: currentCategory + ' - ' + desc,
              mutation: 'M' + i,
              contextImport: getContextImport(currentCategory, language)
            });
          }
        }
      }
    }
  }
  return tasks;
}

function getContextImport(category: string, lang: string): string {
    if (lang === 'rust') {
        if (category.includes('Axum') || category.includes('Web')) return 'use axum::{extract::{State, Json}, routing::post, Router};';
        if (category.includes('DB') || category.includes('SQL')) return 'use sqlx::{Pool, Postgres};';
        return '';
    }
    
    if (category.includes('Cloudflare') || category.includes('Hono') || category.includes('KV')) {
        return "import { Hono } from 'hono';\nconst app = new Hono<{ Bindings: { DB: D1Database, KV: KVNamespace } }>();";
    }
    if (category.includes('Prisma')) {
        return "import { PrismaClient } from '@prisma/client';\nconst prisma = new PrismaClient();";
    }
    if (category.includes('Stripe') || category.includes('Payment')) {
        return "import Stripe from 'stripe';\nconst stripe = new Stripe(process.env.STRIPE_SECRET_KEY!);";
    }
    if (category.includes('AI') || category.includes('LLM')) {
        return "import OpenAI from 'openai';\nconst ai = new OpenAI({ apiKey: process.env.OPENAI_API_KEY });";
    }
    return '';
}

// --- LLM Interaction ---
async function callLLM(task: Task, previousError?: string): Promise<LLMResponse> {
  const fileExt = task.language === 'rust' ? 'rs' : 'ts';
  const prompt = `
## ROLE
You are a security engineer generating corpus pairs for a static analysis engine.

## TASK
Pattern ID: ${task.patternId}
Language: ${task.language}
Bug Category: ${task.description}
Mutation class: ${task.mutation}

Context Header to include:
${task.contextImport}

## OUTPUT FORMAT
Return EXACTLY three code blocks with these filenames as headers:

### corpus/targets/${task.patternId}_positive.${fileExt}
\`\`\`${task.language}
// [frensense]
// observation: <one sentence>
// impact: <one sentence>
// improvement: <one sentence>

<buggy code>
\`\`\`

### corpus/targets/${task.patternId}_negative.${fileExt}
\`\`\`${task.language}
// SAFE: <one sentence explaining what was fixed>

<safe code>
\`\`\`

### corpus/targets/${task.patternId}_negative2.${fileExt}
\`\`\`${task.language}
// SAFE: <alternate fix or different framework variant>

<alternate safe code>
\`\`\`

${previousError ? '\n## PREVIOUS ATTEMPT FAILED\nError: ' + previousError + '\nThe previous output had a syntax error. Fix it and regenerate.' : ''}
`;

  console.log('[Swarm] Dispatching task for ' + task.patternId + ' (' + task.mutation + ')');
  
  // Call Gemini API
  const response = await gemini.models.generateContent({
    model: 'gemini-2.5-pro',
    contents: prompt,
    config: {
        systemInstruction: 'You strictly return the requested code blocks and absolutely nothing else. No intro, no outro.',
        temperature: 0.2,
    }
  });

  const content = response.text || '';

  // Very naive markdown parsing (you might need a stronger regex depending on LLM output)
  const codeBlocks = [...content.matchAll(/```[\w]*\n([\s\S]*?)```/g)];
  
  if (codeBlocks.length < 3) {
      throw new Error("LLM did not return 3 code blocks as requested.");
  }

  return {
      positiveCode: codeBlocks[0][1],
      negativeCode: codeBlocks[1][1],
      negative2Code: codeBlocks[2][1]
  };
}

// --- Validation ---
async function validateFile(filePath: string, language: string): Promise<string | null> {
    try {
        if (language === 'typescript') {
            await execAsync('npx tsc --noEmit --allowJs --target es2022 ' + filePath);
        } else {
            await execAsync('rustfmt --check ' + filePath);
        }
        return null; // Success
    } catch (e: any) {
        return e.stdout || e.stderr || e.message;
    }
}

// --- Worker Loop ---
async function processTask(task: Task): Promise<boolean> {
    let attempts = 0;
    let lastError: string | undefined;

    const ext = task.language === 'rust' ? 'rs' : 'ts';
    const posPath = path.join(__dirname, CORPUS_DIR, task.patternId + '_positive.' + ext);
    const negPath = path.join(__dirname, CORPUS_DIR, task.patternId + '_negative.' + ext);
    const neg2Path = path.join(__dirname, CORPUS_DIR, task.patternId + '_negative2.' + ext);

    while (attempts < MAX_RETRIES) {
        attempts++;
        try {
            const response = await callLLM(task, lastError);
            
            fs.writeFileSync(posPath, response.positiveCode);
            fs.writeFileSync(negPath, response.negativeCode);
            fs.writeFileSync(neg2Path, response.negative2Code);

            // Validate Positive
            const posError = await validateFile(posPath, task.language);
            if (posError) {
                lastError = 'Syntax error in positive file:\n' + posError;
                continue;
            }

            // Validate Negative
            const negError = await validateFile(negPath, task.language);
            if (negError) {
                 lastError = 'Syntax error in negative file:\n' + negError;
                 continue;
            }

            console.log('✅ Successfully generated ' + task.patternId + ' (' + task.mutation + ')');
            return true;

        } catch (e: any) {
            lastError = e.message;
        }
    }

    console.error('❌ Failed to generate ' + task.patternId + ' (' + task.mutation + ') after ' + MAX_RETRIES + ' attempts.');
    return false;
}

// --- Orchestrator ---
async function main() {
    console.log("🚀 Starting Frensense Swarm Seeder...");
    const tasks = parseTaxonomies();
    console.log("📋 Found " + tasks.length + " tasks across both taxonomies.");

    let i = 0;
    const workers = Array(CONCURRENCY).fill(0).map(async () => {
        while (i < tasks.length) {
            const task = tasks[i++];
            await processTask(task);
        }
    });

    await Promise.all(workers);
    console.log("🎉 Seeding complete!");
}

if (require.main === module) {
    main().catch(console.error);
}
