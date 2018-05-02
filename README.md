# File Futures

This crate provides the Futures abstraction around many of the poll methods provided in tokio-fs.

## Usage

Add the dependencies in Cargo.toml
```toml
# Cargo.toml

[dependencies]
tokio-file-futures = "0.1"
tokio-fs = "0.1"
```

```rust
// src/main.rs

extern crate file_futures;
extern crate futures;
extern crate tokio_fs;
```

### Example
```rust
use std::io::SeekFrom;

use file_futures::AsyncFile;
use futures::Future;
use tokio_fs::File;

fn main() {
    let future = File::create("/tmp/some-tmpfile")
        .map_err(|e| println!("Create Error {}", e))
        .and_then(|_| {
            let future1 = File::open("/tmp/some-tmpfile")
                .and_then(|file| file.metadata().and_then(|(_file, _metadata)| Ok(())))
                .map_err(|e| println!("Error1: {}", e));

            let future2 = File::open("/tmp/some-tmpfile")
                .and_then(|file| {
                    file.seek(SeekFrom::Start(30))
                        .and_then(|(_file, _from_start)| Ok(()))
                })
                .map_err(|e| println!("Error2: {}", e));

            future1.join(future2)
        });

    tokio::run(future.map(|_| ()));
}
```

### Contributing
Feel free to open issues for anything you find an issue with. Please note that any contributed code will be licensed under the GPLv3.

### License

Copyright © 2017 Riley Trautman

Tokio File Futures is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

Tokio File Futures is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details. This file is part of Tokio File Futures.

You should have received a copy of the GNU General Public License along with Tokio File Futures. If not, see [http://www.gnu.org/licenses/](http://www.gnu.org/licenses/).
