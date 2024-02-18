use serde::{Serialize, Deserialize};

extern crate winresource;
extern crate serde_json;
extern crate serde;
//extern crate zip;


#[derive(Serialize, Deserialize, Debug)]
struct LastBuild {
    version: String,
    executable: String,
    executable_with_ext: String,
    release_path: String,
    included_files: Vec<String>,
}


fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        let mut res = winresource::WindowsResource::new();
        res.set_icon("images/app.ico");
        res.compile().unwrap();

        // I wish there is a post build script in cargo :(
        /*
        if std::env::var("PROFILE").unwrap() == "release" {
            let executable_name = std::env::var("CARGO_PKG_NAME").unwrap();
            let version = std::env::var("CARGO_PKG_VERSION").unwrap();
            let release_folder = "target/release/";
            let zip_name = format!("{}-{}.zip", executable_name, version);

            let mut zip = zip::ZipWriter::new(std::fs::File::create(&zip_name).unwrap());
            let options = zip::write::FileOptions::default()
                .compression_method(zip::CompressionMethod::Stored)
                .unix_permissions(0o755);
            let folder_name = format!("{}-{}", executable_name, version);
            zip.add_directory(format!("{}", folder_name), options).unwrap();

            zip.start_file(format!("{}/{}-{}", folder_name, &executable_name, version + ".exe"), options).unwrap();
            std::io::Write::write_all(&mut zip, std::fs::read(release_folder.to_owned() + &executable_name + ".exe").unwrap().as_slice()).unwrap();
            zip.start_file(format!("{}/config.json", folder_name), options).unwrap();
            std::io::Write::write_all(&mut zip, std::fs::read("config.json").unwrap().as_slice()).unwrap();
            zip.finish().unwrap();

            std::fs::copy(&zip_name, release_folder.to_owned() + &zip_name).unwrap();

            std::fs::remove_file(&zip_name).unwrap();
        }
        */

        // Create last_build.json
        let version = std::env::var("CARGO_PKG_VERSION").unwrap();
        let release_folder = "target/release/";
        let last_build = LastBuild {
            version,
            executable: std::env::var("CARGO_PKG_NAME").unwrap(),
            executable_with_ext: concat!(env!("CARGO_PKG_NAME"), ".exe").to_owned(),
            release_path: release_folder.to_owned(),
            included_files: vec!["config.json".to_owned()],
        };
        let last_build_json = serde_json::to_string_pretty(&last_build).unwrap();
        std::fs::write("last_build.json", last_build_json).unwrap();
    }
}
