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

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct GACli {
   pub cmd: String,
   #[arg(short = 'm', long = "llm", env = "GT_LLM")]
   pub model: String,
   #[arg(short = 't', long = "threads", env = "GT_THREADS")]
   pub threads: u16,
   #[arg(short = 'T', long = "think", env = "GT_THINK")]
   pub think: bool
}
