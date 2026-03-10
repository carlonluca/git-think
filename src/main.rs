/*
 * This file is part of git-think.
 *
 * Copyright (c) 2026 Luca Carlon
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, version 3.

 * This program is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
 * General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 */

pub mod gacli;
pub mod gagit;
pub mod gaollama;

use clap::Parser;
use log::warn;

#[tokio::main]
async fn main() {
    env_logger::init();

    let args = gacli::GACli::parse();
    let patch = gagit::GAGit::read_staged();
    if patch.is_none() {
        return;
    }
    
    let patch = patch.unwrap();
    let patch = patch.trim();
    if patch.is_empty() {
        warn!("No staged changes");
        return;
    }

    let ollama = gaollama::GAOllama {
        llm: args.model,
        patch: patch.to_string(),
        threads: args.threads
    };

    match args.cmd.as_str() {
        "gen-commit-msg" => {
            let res = ollama.query_gen_commit_msg().await;
            println!("{}", res.unwrap());
        }
        "gen-review" => {
            let res = ollama.query_gen_review().await;
            println!("{}", res.unwrap());
        }
        _ => {}
    }
}
