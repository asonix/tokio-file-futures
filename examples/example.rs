extern crate file_futures;
extern crate futures;
extern crate tokio;
extern crate tokio_fs;

use std::io::SeekFrom;

use file_futures::AsyncFile;
use futures::Future;
use tokio_fs::File;

fn main() {
    let future = File::create("/tmp/some-tmpfile")
        .map_err(|e| println!("Create Error {}", e))
        .and_then(|file| {
            let future1 = File::open("/tmp/some-tmpfile")
                .and_then(|file| file.metadata().and_then(|(_file, _metadata)| Ok(())))
                .map_err(|e| println!("Error1: {}", e));

            let future2 = File::open("/tmp/some-tmpfile")
                .and_then(|file| {
                    file.seek(SeekFrom::Start(30))
                        .and_then(|(_file, _from_start)| Ok(()))
                })
                .map_err(|e| println!("Error2: {}", e));

            let future3 = file.set_len(30)
                .and_then(|file| {
                    file.sync_all().and_then(|file| {
                        file.sync_data().and_then(|file| {
                            file.try_clone().and_then(|(file, _file2)| {
                                file.metadata().and_then(|(file, metadata)| {
                                    let mut permissions = metadata.permissions();
                                    permissions.set_readonly(true);

                                    file.set_permissions(permissions)
                                })
                            })
                        })
                    })
                })
                .map_err(|e| println!("Error3: {}", e));

            future1.and_then(|_| future2).and_then(|_| future3)
        })
        .map(|_| ());

    tokio::run(future);
}
