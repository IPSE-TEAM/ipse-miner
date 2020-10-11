pub(crate) mod ipfs;

use crate::error::IpseError;
use http::Uri;
use ipfs::IpfsStorage;
use ipfs_api::IpfsClient;
use ipfs_api::TryFromUri;

pub trait Storage {
    fn write(&self, path: &str) -> Result<String, IpseError>;
    fn read(&self, key: &str) -> Result<Vec<u8>, IpseError>;
    fn delete(&self, key: &str) -> Result<(), IpseError>;
}

pub fn new_ipfs_storage(ipfs_url: String) -> IpfsStorage {
    let uri = ipfs_url.parse::<Uri>().expect("url parse failed");
    let cli = IpfsClient::build_with_base_uri(uri);
    IpfsStorage { cli }
}
