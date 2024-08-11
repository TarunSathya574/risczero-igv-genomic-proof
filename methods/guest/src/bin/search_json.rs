// Copyright 2023 RISC Zero, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![no_main]

use json::parse;
use json_core::Outputs;
use risc0_zkvm::guest::{env, sha};

risc0_zkvm::guest::entry!(main);

pub fn main() {
    let data: String = env::read();
    let sha = sha::digest(&data.as_bytes());
    let data = parse(&data).unwrap();
    // let proven_val = data["critical_data"].as_u32().unwrap();
    // let proven_val = data["breast_cancer"]["mutation_sequence"].as_str()
    // .expect("Missing field: breast_cancer.mutation_sequence")
    // .to_string();
    let chromosome_string = data["genome"]["chromosomes"]["7"].as_str()
    .expect("Missing field: genome.chromosomes.7")
    .to_string();
    
    //Prove that chromosome 7 has the mutation sequence for specific disease, in this case Cystic Fibrosis
    if !chromosome_string.contains("ATT") {
        panic!("Substring 'ATT' not found in chromosome 7");
    }

    // println!("Hello , {}", proven_val);
    let out = Outputs {
        data: "ATT".to_string(),
        hash: *sha,
    };
    env::commit(&out);
}
