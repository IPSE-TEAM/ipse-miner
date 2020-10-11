#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
use crate::error::IpseError;
use crate::miner::Miner;
use clap::{App, Arg};
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::path::Path;
use std::fs;
// use clap::lazy_static::lazy::Lazy;

use rocket_upload::MultipartDatas;

mod calls;
mod config;
mod error;
mod miner;
mod storage;

static MINER: Lazy<Mutex<Miner>> = Lazy::new(|| {
    let matches = App::new("Ipse Miner")
        .version("0.1.0")
        .about("Mining for Ipse chain")
        .arg(
            Arg::with_name(CONF_PATH)
                .short('c')
                .long(CONF_PATH)
                .default_value("config.yaml"),
        )
        .get_matches();
    let conf_fpath = matches.value_of(CONF_PATH).unwrap();

    let cfg = config::load_conf(conf_fpath);
    Mutex::new(miner::Miner::new(cfg))
});

pub const CONF_PATH: &'static str = "conf_path";

fn main() {
    {
        MINER.lock().unwrap().register_miner();
    }
    rocket::ignite()
        .mount("/", routes![new_order, delete_order])
        .launch();

    //let mut rt = Runtime::new().unwrap();
    // rt.block_on( async move {
    //     HttpServer::new( || {
    //         WebApp::new().wrap(middleware::Logger::default()).service(
    //             web::resource("/")
    //                 .route(web::post().to(new_order))
    //                 .route(web::delete().to(delete_order)),
    //         )
    //     })
    //         .bind("localhost:8000").unwrap().run().await
    // })


}

// #[post("/order?<id>", data = "<file>")]
// pub fn new_order(id: usize, file: Data) -> Result<String, IpseError> {
//     println!("data from client is :{}", std::str::from_utf8(file.peek()).unwrap());
//     let fpath = format!("/tmp/{}", id);
//     file.stream_to_file(Path::new(&fpath))?;
//
//     let file_url = MINER.lock().unwrap().write_file(id as i64, &fpath)?;
//     fs::remove_file(&fpath)?;
//     Ok(file_url)
// }

#[post("/order?<id>", data = "<data>")]
pub fn new_order(id: usize, data: MultipartDatas) -> Result<String, IpseError> {
    let fp = data.files.get(0).ok_or(IpseError::FileNotFoundFromClient)?;

    println!("path: {}", &fp.path);
    let fpath = format!("/tmp/upload-{}", id);
    fs::create_dir_all(&fpath)?;
    println!("create dir: {}", &fpath);
    fp.persist(Path::new(&fpath));

    let fpath_to_ipfs = format!("{}/{}",&fpath,&fp.filename);
    let file_url = MINER.lock().unwrap().write_file(id as i64, &fpath_to_ipfs)?;
    println!("store into ipfs");

    fs::remove_dir(&fpath)?;
    println!("remove dir");

    Ok(file_url)
}

#[delete("/order?<id>")]
pub fn delete_order(id: usize) -> Result<(), IpseError> {
    MINER.lock().unwrap().delete_file(id as i64)
}

// async fn new_order(mut payload: Multipart) -> Result<HttpResponse, actix_web::Error> {
//
// }
//
// async fn delete_order(id: usize) -> Result<HttpResponse, actix_web::Error> {
//     MINER.lock().unwrap().delete_order(id as i64)
// }
