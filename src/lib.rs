extern crate futures;
extern crate tokio_fs;

use std::{fs::{Metadata, Permissions}, io::{Error, SeekFrom}};
use futures::{Async, Future, Poll};

pub trait AsyncFile: Sized {
    fn poll_seek(&mut self, pos: SeekFrom) -> Poll<u64, Error>;
    fn poll_sync_all(&mut self) -> Poll<(), Error>;
    fn poll_sync_data(&mut self) -> Poll<(), Error>;
    fn poll_set_len(&mut self, size: u64) -> Poll<(), Error>;
    fn poll_metadata(&mut self) -> Poll<Metadata, Error>;
    fn poll_try_clone(&mut self) -> Poll<tokio_fs::file::File, Error>;
    fn poll_set_permissions(&mut self, perm: Permissions) -> Poll<(), Error>;

    fn seek(self, pos: SeekFrom) -> Seek<Self> {
        Seek {
            pos,
            inner: Some(self),
        }
    }

    fn sync_all(self) -> SyncAll<Self> {
        SyncAll { inner: Some(self) }
    }

    fn sync_data(self) -> SyncData<Self> {
        SyncData { inner: Some(self) }
    }

    fn set_len(self, size: u64) -> SetLen<Self> {
        SetLen {
            size,
            inner: Some(self),
        }
    }

    fn metadata(self) -> GetMetadata<Self> {
        GetMetadata { inner: Some(self) }
    }

    fn try_clone(self) -> TryClone<Self> {
        TryClone { inner: Some(self) }
    }

    fn set_permissions(self, perm: Permissions) -> SetPermissions<Self> {
        SetPermissions {
            perm,
            inner: Some(self),
        }
    }
}

impl AsyncFile for tokio_fs::file::File {
    fn poll_seek(&mut self, pos: SeekFrom) -> Poll<u64, Error> {
        tokio_fs::file::File::poll_seek(self, pos)
    }

    fn poll_sync_all(&mut self) -> Poll<(), Error> {
        tokio_fs::file::File::poll_sync_all(self)
    }

    fn poll_sync_data(&mut self) -> Poll<(), Error> {
        tokio_fs::file::File::poll_sync_data(self)
    }

    fn poll_set_len(&mut self, size: u64) -> Poll<(), Error> {
        tokio_fs::file::File::poll_set_len(self, size)
    }

    fn poll_metadata(&mut self) -> Poll<Metadata, Error> {
        tokio_fs::file::File::poll_metadata(self)
    }

    fn poll_try_clone(&mut self) -> Poll<tokio_fs::file::File, Error> {
        tokio_fs::file::File::poll_try_clone(self)
    }

    fn poll_set_permissions(&mut self, perm: Permissions) -> Poll<(), Error> {
        tokio_fs::file::File::poll_set_permissions(self, perm)
    }
}

pub struct Seek<T> {
    pos: SeekFrom,
    inner: Option<T>,
}

impl<T> Future for Seek<T>
where
    T: AsyncFile,
{
    type Item = (T, u64);
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        let mut inner = self.inner.take().unwrap();

        match inner.poll_seek(self.pos) {
            Ok(Async::Ready(seek)) => Ok(Async::Ready((inner, seek))),
            Ok(_) => Ok(Async::NotReady),
            Err(e) => Err(e),
        }
    }
}

impl<T> AsyncFile for Seek<T>
where
    T: AsyncFile,
{
    fn poll_seek(&mut self, pos: SeekFrom) -> Poll<u64, Error> {
        let mut inner = self.inner.take().unwrap();

        let res = inner.poll_seek(pos);
        self.inner = Some(inner);

        res
    }

    fn poll_sync_all(&mut self) -> Poll<(), Error> {
        let mut inner = self.inner.take().unwrap();

        let res = inner.poll_sync_all();
        self.inner = Some(inner);

        res
    }

    fn poll_sync_data(&mut self) -> Poll<(), Error> {
        let mut inner = self.inner.take().unwrap();

        let res = inner.poll_sync_data();
        self.inner = Some(inner);

        res
    }

    fn poll_set_len(&mut self, size: u64) -> Poll<(), Error> {
        let mut inner = self.inner.take().unwrap();

        let res = inner.poll_set_len(size);
        self.inner = Some(inner);

        res
    }

    fn poll_metadata(&mut self) -> Poll<Metadata, Error> {
        let mut inner = self.inner.take().unwrap();

        let res = inner.poll_metadata();
        self.inner = Some(inner);

        res
    }

    fn poll_try_clone(&mut self) -> Poll<tokio_fs::file::File, Error> {
        let mut inner = self.inner.take().unwrap();

        let res = inner.poll_try_clone();
        self.inner = Some(inner);

        res
    }

    fn poll_set_permissions(&mut self, perm: Permissions) -> Poll<(), Error> {
        let mut inner = self.inner.take().unwrap();

        let res = inner.poll_set_permissions(perm);
        self.inner = Some(inner);

        res
    }
}

pub struct SyncAll<T> {
    inner: Option<T>,
}

impl<T> Future for SyncAll<T>
where
    T: AsyncFile,
{
    type Item = T;
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        let mut inner = self.inner.take().unwrap();

        match inner.poll_sync_all() {
            Ok(Async::Ready(())) => Ok(Async::Ready(inner)),
            Ok(_) => Ok(Async::NotReady),
            Err(e) => Err(e),
        }
    }
}

impl<T> AsyncFile for SyncAll<T>
where
    T: AsyncFile,
{
    fn poll_seek(&mut self, pos: SeekFrom) -> Poll<u64, Error> {
        let mut inner = self.inner.take().unwrap();

        let res = inner.poll_seek(pos);
        self.inner = Some(inner);

        res
    }

    fn poll_sync_all(&mut self) -> Poll<(), Error> {
        let mut inner = self.inner.take().unwrap();

        let res = inner.poll_sync_all();
        self.inner = Some(inner);

        res
    }

    fn poll_sync_data(&mut self) -> Poll<(), Error> {
        let mut inner = self.inner.take().unwrap();

        let res = inner.poll_sync_data();
        self.inner = Some(inner);

        res
    }

    fn poll_set_len(&mut self, size: u64) -> Poll<(), Error> {
        let mut inner = self.inner.take().unwrap();

        let res = inner.poll_set_len(size);
        self.inner = Some(inner);

        res
    }

    fn poll_metadata(&mut self) -> Poll<Metadata, Error> {
        let mut inner = self.inner.take().unwrap();

        let res = inner.poll_metadata();
        self.inner = Some(inner);

        res
    }

    fn poll_try_clone(&mut self) -> Poll<tokio_fs::file::File, Error> {
        let mut inner = self.inner.take().unwrap();

        let res = inner.poll_try_clone();
        self.inner = Some(inner);

        res
    }

    fn poll_set_permissions(&mut self, perm: Permissions) -> Poll<(), Error> {
        let mut inner = self.inner.take().unwrap();

        let res = inner.poll_set_permissions(perm);
        self.inner = Some(inner);

        res
    }
}

pub struct SyncData<T> {
    inner: Option<T>,
}

impl<T> Future for SyncData<T>
where
    T: AsyncFile,
{
    type Item = T;
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        let mut inner = self.inner.take().unwrap();

        match inner.poll_sync_all() {
            Ok(Async::Ready(())) => Ok(Async::Ready(inner)),
            Ok(_) => Ok(Async::NotReady),
            Err(e) => Err(e),
        }
    }
}

impl<T> AsyncFile for SyncData<T>
where
    T: AsyncFile,
{
    fn poll_seek(&mut self, pos: SeekFrom) -> Poll<u64, Error> {
        let mut inner = self.inner.take().unwrap();

        let res = inner.poll_seek(pos);
        self.inner = Some(inner);

        res
    }

    fn poll_sync_all(&mut self) -> Poll<(), Error> {
        let mut inner = self.inner.take().unwrap();

        let res = inner.poll_sync_all();
        self.inner = Some(inner);

        res
    }

    fn poll_sync_data(&mut self) -> Poll<(), Error> {
        let mut inner = self.inner.take().unwrap();

        let res = inner.poll_sync_data();
        self.inner = Some(inner);

        res
    }

    fn poll_set_len(&mut self, size: u64) -> Poll<(), Error> {
        let mut inner = self.inner.take().unwrap();

        let res = inner.poll_set_len(size);
        self.inner = Some(inner);

        res
    }

    fn poll_metadata(&mut self) -> Poll<Metadata, Error> {
        let mut inner = self.inner.take().unwrap();

        let res = inner.poll_metadata();
        self.inner = Some(inner);

        res
    }

    fn poll_try_clone(&mut self) -> Poll<tokio_fs::file::File, Error> {
        let mut inner = self.inner.take().unwrap();

        let res = inner.poll_try_clone();
        self.inner = Some(inner);

        res
    }

    fn poll_set_permissions(&mut self, perm: Permissions) -> Poll<(), Error> {
        let mut inner = self.inner.take().unwrap();

        let res = inner.poll_set_permissions(perm);
        self.inner = Some(inner);

        res
    }
}

pub struct SetLen<T> {
    size: u64,
    inner: Option<T>,
}

impl<T> Future for SetLen<T>
where
    T: AsyncFile,
{
    type Item = T;
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        let mut inner = self.inner.take().unwrap();

        match inner.poll_set_len(self.size) {
            Ok(Async::Ready(())) => Ok(Async::Ready(inner)),
            Ok(_) => Ok(Async::NotReady),
            Err(e) => Err(e),
        }
    }
}

impl<T> AsyncFile for SetLen<T>
where
    T: AsyncFile,
{
    fn poll_seek(&mut self, pos: SeekFrom) -> Poll<u64, Error> {
        let mut inner = self.inner.take().unwrap();

        let res = inner.poll_seek(pos);
        self.inner = Some(inner);

        res
    }

    fn poll_sync_all(&mut self) -> Poll<(), Error> {
        let mut inner = self.inner.take().unwrap();

        let res = inner.poll_sync_all();
        self.inner = Some(inner);

        res
    }

    fn poll_sync_data(&mut self) -> Poll<(), Error> {
        let mut inner = self.inner.take().unwrap();

        let res = inner.poll_sync_data();
        self.inner = Some(inner);

        res
    }

    fn poll_set_len(&mut self, size: u64) -> Poll<(), Error> {
        let mut inner = self.inner.take().unwrap();

        let res = inner.poll_set_len(size);
        self.inner = Some(inner);

        res
    }

    fn poll_metadata(&mut self) -> Poll<Metadata, Error> {
        let mut inner = self.inner.take().unwrap();

        let res = inner.poll_metadata();
        self.inner = Some(inner);

        res
    }

    fn poll_try_clone(&mut self) -> Poll<tokio_fs::file::File, Error> {
        let mut inner = self.inner.take().unwrap();

        let res = inner.poll_try_clone();
        self.inner = Some(inner);

        res
    }

    fn poll_set_permissions(&mut self, perm: Permissions) -> Poll<(), Error> {
        let mut inner = self.inner.take().unwrap();

        let res = inner.poll_set_permissions(perm);
        self.inner = Some(inner);

        res
    }
}

pub struct GetMetadata<T> {
    inner: Option<T>,
}

impl<T> Future for GetMetadata<T>
where
    T: AsyncFile,
{
    type Item = (T, Metadata);
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        let mut inner = self.inner.take().unwrap();

        match inner.poll_metadata() {
            Ok(Async::Ready(metadata)) => Ok(Async::Ready((inner, metadata))),
            Ok(_) => Ok(Async::NotReady),
            Err(e) => Err(e),
        }
    }
}

impl<T> AsyncFile for GetMetadata<T>
where
    T: AsyncFile,
{
    fn poll_seek(&mut self, pos: SeekFrom) -> Poll<u64, Error> {
        let mut inner = self.inner.take().unwrap();

        let res = inner.poll_seek(pos);
        self.inner = Some(inner);

        res
    }

    fn poll_sync_all(&mut self) -> Poll<(), Error> {
        let mut inner = self.inner.take().unwrap();

        let res = inner.poll_sync_all();
        self.inner = Some(inner);

        res
    }

    fn poll_sync_data(&mut self) -> Poll<(), Error> {
        let mut inner = self.inner.take().unwrap();

        let res = inner.poll_sync_data();
        self.inner = Some(inner);

        res
    }

    fn poll_set_len(&mut self, size: u64) -> Poll<(), Error> {
        let mut inner = self.inner.take().unwrap();

        let res = inner.poll_set_len(size);
        self.inner = Some(inner);

        res
    }

    fn poll_metadata(&mut self) -> Poll<Metadata, Error> {
        let mut inner = self.inner.take().unwrap();

        let res = inner.poll_metadata();
        self.inner = Some(inner);

        res
    }

    fn poll_try_clone(&mut self) -> Poll<tokio_fs::file::File, Error> {
        let mut inner = self.inner.take().unwrap();

        let res = inner.poll_try_clone();
        self.inner = Some(inner);

        res
    }

    fn poll_set_permissions(&mut self, perm: Permissions) -> Poll<(), Error> {
        let mut inner = self.inner.take().unwrap();

        let res = inner.poll_set_permissions(perm);
        self.inner = Some(inner);

        res
    }
}

pub struct TryClone<T> {
    inner: Option<T>,
}

impl<T> Future for TryClone<T>
where
    T: AsyncFile,
{
    type Item = (T, tokio_fs::file::File);
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        let mut inner = self.inner.take().unwrap();

        match inner.poll_try_clone() {
            Ok(Async::Ready(file)) => Ok(Async::Ready((inner, file))),
            Ok(_) => Ok(Async::NotReady),
            Err(e) => Err(e),
        }
    }
}

impl<T> AsyncFile for TryClone<T>
where
    T: AsyncFile,
{
    fn poll_seek(&mut self, pos: SeekFrom) -> Poll<u64, Error> {
        let mut inner = self.inner.take().unwrap();

        let res = inner.poll_seek(pos);
        self.inner = Some(inner);

        res
    }

    fn poll_sync_all(&mut self) -> Poll<(), Error> {
        let mut inner = self.inner.take().unwrap();

        let res = inner.poll_sync_all();
        self.inner = Some(inner);

        res
    }

    fn poll_sync_data(&mut self) -> Poll<(), Error> {
        let mut inner = self.inner.take().unwrap();

        let res = inner.poll_sync_data();
        self.inner = Some(inner);

        res
    }

    fn poll_set_len(&mut self, size: u64) -> Poll<(), Error> {
        let mut inner = self.inner.take().unwrap();

        let res = inner.poll_set_len(size);
        self.inner = Some(inner);

        res
    }

    fn poll_metadata(&mut self) -> Poll<Metadata, Error> {
        let mut inner = self.inner.take().unwrap();

        let res = inner.poll_metadata();
        self.inner = Some(inner);

        res
    }

    fn poll_try_clone(&mut self) -> Poll<tokio_fs::file::File, Error> {
        let mut inner = self.inner.take().unwrap();

        let res = inner.poll_try_clone();
        self.inner = Some(inner);

        res
    }

    fn poll_set_permissions(&mut self, perm: Permissions) -> Poll<(), Error> {
        let mut inner = self.inner.take().unwrap();

        let res = inner.poll_set_permissions(perm);
        self.inner = Some(inner);

        res
    }
}

pub struct SetPermissions<T> {
    perm: Permissions,
    inner: Option<T>,
}

impl<T> Future for SetPermissions<T>
where
    T: AsyncFile,
{
    type Item = T;
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        let mut inner = self.inner.take().unwrap();

        match inner.poll_set_permissions(self.perm.clone()) {
            Ok(Async::Ready(())) => Ok(Async::Ready(inner)),
            Ok(_) => Ok(Async::NotReady),
            Err(e) => Err(e),
        }
    }
}

impl<T> AsyncFile for SetPermissions<T>
where
    T: AsyncFile,
{
    fn poll_seek(&mut self, pos: SeekFrom) -> Poll<u64, Error> {
        let mut inner = self.inner.take().unwrap();

        let res = inner.poll_seek(pos);
        self.inner = Some(inner);

        res
    }

    fn poll_sync_all(&mut self) -> Poll<(), Error> {
        let mut inner = self.inner.take().unwrap();

        let res = inner.poll_sync_all();
        self.inner = Some(inner);

        res
    }

    fn poll_sync_data(&mut self) -> Poll<(), Error> {
        let mut inner = self.inner.take().unwrap();

        let res = inner.poll_sync_data();
        self.inner = Some(inner);

        res
    }

    fn poll_set_len(&mut self, size: u64) -> Poll<(), Error> {
        let mut inner = self.inner.take().unwrap();

        let res = inner.poll_set_len(size);
        self.inner = Some(inner);

        res
    }

    fn poll_metadata(&mut self) -> Poll<Metadata, Error> {
        let mut inner = self.inner.take().unwrap();

        let res = inner.poll_metadata();
        self.inner = Some(inner);

        res
    }

    fn poll_try_clone(&mut self) -> Poll<tokio_fs::file::File, Error> {
        let mut inner = self.inner.take().unwrap();

        let res = inner.poll_try_clone();
        self.inner = Some(inner);

        res
    }

    fn poll_set_permissions(&mut self, perm: Permissions) -> Poll<(), Error> {
        let mut inner = self.inner.take().unwrap();

        let res = inner.poll_set_permissions(perm);
        self.inner = Some(inner);

        res
    }
}
