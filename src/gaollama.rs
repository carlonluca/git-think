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

use log::debug;
use reqwest::Client;
use serde::Deserialize;
use serde_json::json;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio_util::io::StreamReader;
use futures::TryStreamExt;

#[derive(Deserialize)]
struct Chunk {
    response: String,
}

pub struct GAOllama {
    pub llm: String,
    pub patch: String,
    pub threads: u16,
    pub think: bool
}

impl GAOllama {
   pub async fn query_gen_commit_msg(&self) -> Option<String> {
      let prompt = format!(r#"
Given the patch below, create a proper git commit message.
Output ONLY the commit message text.
Do not include explanations, confirmations, or any additional text. This is the patch:
{0}"#, self.patch);

      self.query(&prompt).await
   }

   pub async fn query_gen_review(&self) -> Option<String> {
      let prompt = format!(r#"
Output only comments in a numbered list.
Cite the lines of code you comment when possible.
Give priority to bugs.
Review this diff:
{0}"#, self.patch);

      self.query(&prompt).await
   }

   async fn query(&self, prompt: &str) -> Option<String> {
      let client = Client::new();
      let payload = json!({
         "model": self.llm,
         "prompt": prompt,
         "think": self.think,
         "options": {
            "temperature": 0,
            "num_thread": self.threads
         }
      });

      let resp = client
         .post("http://localhost:11434/api/generate")
         .json(&payload)
         .send()
         .await
         .ok()?;

      let stream = resp.bytes_stream();
      let reader = StreamReader::new(
         stream.map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e)),
      );

      let mut lines = BufReader::new(reader).lines();
      let mut out = String::new();
      while let Ok(Some(line)) = lines.next_line().await {
         if let Ok(chunk) = serde_json::from_str::<Chunk>(&line) {
            out.push_str(&chunk.response);
         }
      }

      Some(out)
   }
}
