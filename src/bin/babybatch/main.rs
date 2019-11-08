use std::fs::{self, OpenOptions};
use std::io::BufWriter;
use std::path::PathBuf;

use serde_json;

use babybatch::{BabyBatchResponse, Resource, Result, ThreadPool};

// Module Declarations.
mod cli;
mod commands;

fn main() -> Result<()> {
    let resource;
    // Read api key from environment.
    if let Ok(api_key) = fs::read_to_string(".env") {
        resource = Resource::new(api_key.trim_end())
            .country_code("US")
            .page_token("&");
    } else {
        // If there is no `.env` file to read from in the current working directory,
        // then alert the user and exit.
        eprintln!("baby-batch: failed to read api key from `.env`");
        std::process::exit(1);
    }

    let pool = ThreadPool::new(6);
    let mut rl = RequestLoop::from((pool, resource, vec![])).into_iter();

    loop {
        if rl.next().is_none() {
            break;
        }
    }
    Ok(())
}

struct RequestLoop {
    pool: ThreadPool,
    resource: Resource,
    info: Vec<Result<()>>,
    version: u64,
}

impl From<(ThreadPool, Resource, Vec<Result<()>>)> for RequestLoop {
    fn from((pool, resource, info): (ThreadPool, Resource, Vec<Result<()>>)) -> Self {
        RequestLoop {
            pool,
            resource,
            info,
            version: 0u64,
        }
    }
}

impl Iterator for RequestLoop {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        if let Ok(BabyBatchResponse { body, page_token }) = self.request() {
            self.resource.set_token(&page_token);
            let path = PathBuf::from(&format!("{}.json", self.version));
            if let Ok(handle) = OpenOptions::new().create(true).write(true).open(&path) {
                let w = BufWriter::new(handle);
                self.info.push(
                    self.pool
                        .execute(move || match serde_json::to_writer(w, &body) {
                            Ok(()) => (),
                            Err(e) => eprintln!("{}", e),
                        }),
                );
            }
            self.version += 1;
            return Some(self.version);
        }
        None
    }
}

impl RequestLoop {
    fn request(&mut self) -> Result<BabyBatchResponse> {
        self.resource.request()
    }
}
