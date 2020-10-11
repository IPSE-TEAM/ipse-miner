#[derive(Debug)]
pub enum IpseError {
    NoneOrder,
    DataInvalid,
    FileNotFoundFromClient,
    IO(std::io::Error),
    Sqlite(rusqlite::Error),
    IpfsResp(String),
    Substrate(substrate_subxt::Error),
}

impl From<std::io::Error> for IpseError {
    fn from(err: std::io::Error) -> Self {
        IpseError::IO(err)
    }
}

impl From<rusqlite::Error> for IpseError {
    fn from(err: rusqlite::Error) -> Self {
        IpseError::Sqlite(err)
    }
}

impl From<substrate_subxt::Error> for IpseError {
    fn from(err: substrate_subxt::Error) -> Self {
        IpseError::Substrate(err)
    }
}
