use crate::storage::Storage;
//use futures::TryStreamExt;
use ipfs_api::IpfsClient;
//use tokio::runtime::Runtime;
use std::str;
use crate::error::IpseError;

pub struct IpfsStorage {
    pub cli: IpfsClient,
}

impl Storage for IpfsStorage {
    fn write(&self, path: &str) -> Result<String, IpseError> {
        // let mut rt = Runtime::new()?;
        // rt.block_on(async move {
        //     let file_data = File::open(path)?;
        //     let content = fs::read(path)?;
        //     let string = str::from_utf8(content.as_slice()).unwrap();
        //     println!("file content: {}", string);
        //     let res = self.cli.add(file_data).await.expect("store ipfs error");
        //     println!("add file finished!!");
        //     Ok(res.hash)
        // })
        let (_, stdout, stderr) = shells::sh!("ipfs add {}", path);
        if &stderr != ""{
            return Err(IpseError::IpfsResp(stderr))
        }
        println!("{}", &stdout);
        let mut iter = stdout.split_whitespace();
        iter.next();
        let file_url = iter.next().unwrap();
        Ok(file_url.parse().unwrap())
    }

    fn read(&self, key: &str) -> Result<Vec<u8>, IpseError> {
        // let mut rt = Runtime::new()?;
        // rt.block_on(async move {
        //     self.cli
        //         .cat(key)
        //         .map_ok(|chunk| chunk.to_vec())
        //         .try_concat()
        //         .await
        //         .map_err(|e| From::from(e))
        // })
        unimplemented!()
    }

    fn delete(&self, key: &str) -> Result<(), IpseError> {
        // let mut rt = Runtime::new()?;
        // rt.block_on(async move { self.cli.pin_rm(key, false).await })?;
        // Ok(())
        unimplemented!()
    }
}
