/*******************************************************************************
* Copyright 2023, IBM Corp.
*
* Licensed under the Apache License, Version 2.0 (the "License");
* you may not use this file except in compliance with the License.
* You may obtain a copy of the License at
*
* http://www.apache.org/licenses/LICENSE-2.0
*
* Unless required by applicable law or agreed to in writing, software
* distributed under the License is distributed on an "AS IS" BASIS,
* WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
* See the License for the specific language governing permissions and
* limitations under the License.
*******************************************************************************/

use reqwest::blocking::Client;

const NUM_ARGS: u32 = 2;
const DEFAULT_STATUS: u16 = 404;
const SLEEP_MILLIS: u64 = 1;

fn query_endpoint(endpoint: &String) -> i32 {

    /* Intialize new Client structure */
    let client = Client::new();

    /* Initialize a default status code for comparison below */
    let response = reqwest::StatusCode::from_u16(DEFAULT_STATUS);
    if response.is_err() {
        println!("Could not initialize the status code");
        return 1;
    }
    let mut status = response.unwrap();

    /* Query the endpoint until the status code it returns is 200 */
    while status != reqwest::StatusCode::OK {
        std::thread::sleep(std::time::Duration::from_millis(SLEEP_MILLIS));

        /* Ignore errors, since the endpoint may not be available yet */
        if let Ok(resp) = client.get(endpoint).send() {
            status = resp.status();
        }
    }

    0
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != (NUM_ARGS as usize) {
        println!("Invalid arguments\n");
        println!("Usage: {} <endpoint>", args[0]);
        std::process::exit(1);
    }

    let endpoint = &args[1];

    println!("Querying {} using GET until a HTTP response code of 200", endpoint);
    std::process::exit(query_endpoint(endpoint));
}
