// SPDX-License-Identifier: MIT
//! `GenSense` MCP Server — stdin/stdout JSON-RPC bridge.
//!
//! Thin entry point that delegates to the `gensense::mcp` module.

use gensense::GENSENSE_VERSION;
use gensense::mcp::handler::handle_request;
use gensense::mcp::protocol::{JsonRpcRequest, RequestId, rpc_error, write_response};
use std::io::{self, BufRead};

fn main() {
    eprintln!("gensense-mcp v{GENSENSE_VERSION} starting");
    eprintln!(
        "gensense-mcp: cwd={:?}, has_rust={}",
        std::env::current_dir().ok(),
        cfg!(feature = "rust")
    );

    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = match line {
            Ok(l) => l,
            Err(e) => {
                eprintln!("gensense-mcp: stdin read error: {e}");
                break;
            }
        };

        if line.trim().is_empty() {
            continue;
        }

        let req: JsonRpcRequest = match serde_json::from_str(&line) {
            Ok(r) => r,
            Err(e) => {
                let err_resp = rpc_error(RequestId::Absent, -32700, format!("parse error: {e}"));
                write_response(&err_resp);
                continue;
            }
        };

        if req.method == "exit" {
            break;
        }

        let resp = handle_request(req);
        if resp.id.is_some() {
            write_response(&resp);
        }
    }

    eprintln!("gensense-mcp: exiting");
}
