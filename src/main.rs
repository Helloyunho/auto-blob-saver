extern crate clap;
extern crate dirs;
extern crate job_scheduler;
extern crate regex;
extern crate reqwest;
extern crate tokio;
use clap::{value_t, App, Arg};
use rand::{thread_rng, Rng};
use regex::Regex;
use serde::Deserialize;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::{thread, time};

const APNONCE: &[u8] = b"1234567890abcdef";

fn check_and_make_dir(path: String) {
    let path_to_path = Path::new(&path);
    if !path_to_path.is_dir() {
        std::fs::create_dir(&path).expect("An error has occured while creating dir.");
    }
}

fn check_and_make_file(path: String, content: Option<&str>, exit: bool) {
    let path_to_path = Path::new(&path);
    if !path_to_path.exists() {
        if !content.is_none() {
            std::fs::write(&path, content.unwrap())
                .expect("An error has occured while creating file.");
        } else {
            std::fs::write(&path, "").expect("An error has occured while creating file.");
        }
        if exit {
            std::process::exit(0);
        }
    }
}

fn args_builder(
    ecid: &String,
    identifier: &String,
    boardconfig: Option<&String>,
    buildid: &String,
    generator: Option<&String>,
    apnonce: Option<&String>,
    ota: bool,
    shsh_path: &str,
) -> Vec<String> {
    let mut args = vec!["-d".to_string()];
    args.push(identifier.to_string());
    args.push("-e".to_string());
    args.push(ecid.to_string());
    args.push("--buildid".to_string());
    args.push(buildid.to_string());
    if !boardconfig.is_none() {
        args.push("-B".to_string());
        args.push(boardconfig.unwrap().to_string());
    }
    if ota {
        args.push("-o".to_string());
    }
    if !generator.is_none() {
        args.push("-g".to_string());
        args.push(generator.unwrap().to_string());
        args.push("--save-path".to_string());
        args.push(format!(
            "{}/{}/{}/{}",
            shsh_path,
            ecid,
            buildid,
            generator.unwrap().to_string()
        ));
    } else if !apnonce.is_none() {
        args.push("--apnonce".to_string());
        args.push(apnonce.unwrap().to_string());
        args.push("--save-path".to_string());
        args.push(format!(
            "{}/{}/{}/{}",
            shsh_path,
            ecid,
            buildid,
            apnonce.unwrap().to_string()
        ));
    }
    args.push("-s".to_string());
    println!("{:?}", args.join(" "));

    args
}

#[derive(Deserialize)]
struct Response {
    firmwares: Vec<Firmware>,
}

#[derive(Deserialize)]
struct Firmware {
    buildid: String,
    releasetype: String,
    signed: bool,
}

#[derive(Deserialize, Clone)]
struct Device {
    ecid: String,
    identifier: String,
    boardconfig: Option<String>,
}

impl Device {
    async fn get_blobs(&self, shsh_path: &str) {
        check_and_make_dir(format!("{}/{}", shsh_path, self.ecid));
        let json_text = reqwest::get(
            format!("https://api.ipsw.me/v4/device/{}?type=ota", self.identifier).as_str(),
        )
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
        let api_responsed: Response = serde_json::from_str(json_text.as_str()).unwrap();
        let firmwares = api_responsed.firmwares;
        for firmware in firmwares {
            if !firmware.signed {
                continue;
            }
            check_and_make_dir(format!("{}/{}/{}", shsh_path, self.ecid, firmware.buildid));
            check_and_make_dir(format!(
                "{}/{}/{}/0x1111111111111111",
                shsh_path, self.ecid, firmware.buildid
            ));
            check_and_make_dir(format!(
                "{}/{}/{}/0xbd34a880be0b53f3",
                shsh_path, self.ecid, firmware.buildid
            ));

            for generator in vec!["0x1111111111111111", "0xbd34a880be0b53f3"] {
                Command::new("tsschecker")
                    .args(&args_builder(
                        &self.ecid,
                        &self.identifier,
                        self.boardconfig.as_ref(),
                        &firmware.buildid,
                        Some(&generator.to_string()),
                        None,
                        firmware.releasetype == "Beta",
                        shsh_path,
                    ))
                    .spawn()
                    .expect("An error has occured while getting the blob.");
            }

            for _i in 1..10 {
                let mut rng = thread_rng();
                let re = Regex::new(r"(?P<model>iPhone|iPad|iPod)(?P<gen>\d{1,2}),\d{1}").unwrap();
                let parsed = re.captures(&self.identifier).unwrap();
                let model = &parsed["model"];
                let gen = &parsed["gen"];
                let gen_num: u32 = gen.parse().unwrap();
                let length = if ((model == "iPhone" || model == "iPod") && gen_num < 9)
                    || (model == "iPad" && gen_num < 7)
                {
                    40
                } else {
                    64
                };
                let apnonce: String = (0..length)
                    .map(|_| {
                        let idx = rng.gen_range(0, APNONCE.len());
                        APNONCE[idx] as char
                    })
                    .collect();

                check_and_make_dir(format!(
                    "{}/{}/{}/{}",
                    shsh_path, self.ecid, firmware.buildid, apnonce
                ));
                Command::new("tsschecker")
                    .args(&args_builder(
                        &self.ecid,
                        &self.identifier,
                        self.boardconfig.as_ref(),
                        &firmware.buildid,
                        None,
                        Some(&apnonce),
                        firmware.releasetype == "Beta",
                        shsh_path,
                    ))
                    .spawn()
                    .expect("An error has occured while getting the blob.");
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let mut default_devices_path = dirs::home_dir().unwrap();
    default_devices_path.push(".auto_blob_saver");
    default_devices_path.push("devices.json");
    let mut default_shsh_path = dirs::home_dir().unwrap();
    default_shsh_path.push(".shsh");

    let matches = App::new("Auto Blob Saver")
        .version("0.1.0")
        .author("Helloyunho (@yunho098765)")
        .about("Save your blobs automatically")
        .arg(
            Arg::with_name("devices")
                .long("devices")
                .value_name("DEVICES FILE")
                .help("Sets the devices.json path")
                .takes_value(true)
                .default_value(default_devices_path.to_str().unwrap()),
        )
        .arg(
            Arg::with_name("shsh")
                .long("shsh")
                .value_name("SHSH FOLDER")
                .help("Sets shsh folder path")
                .takes_value(true)
                .default_value(default_shsh_path.to_str().unwrap()),
        )
        .arg(
            Arg::with_name("time")
                .short("t")
                .long("time")
                .value_name("MILLISECOND")
                .help("Sets shsh download time interval")
                .takes_value(true)
                .default_value("600000"),
        )
        .get_matches();

    let devices_path = matches.value_of("devices").unwrap();
    let shsh_path = matches.value_of("shsh").unwrap();
    let interval = value_t!(matches, "time", u64).unwrap();

    check_and_make_dir(
        default_devices_path
            .parent()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string(),
    );
    check_and_make_file(
        devices_path.to_string(),
        Some(
            "[
    {
        \"ecid\": \"ECID\",
        \"identifier\": \"Phone Identifier(Ex. iPhone12,1)\",
        \"boardconfig\": \"Phone Boardconfig(It's optional)(Ex. n71ap)\"
    }
]",
        ),
        true,
    );
    check_and_make_dir(shsh_path.to_string());

    let devices_string =
        fs::read_to_string(devices_path).expect("An error has occured while reading devices.json.");
    let devices: Vec<Device> = serde_json::from_str(&devices_string).unwrap();
    loop {
        for device in &devices {
            device.get_blobs(shsh_path).await;
            thread::sleep(time::Duration::from_millis(1000));
        }
        thread::sleep(time::Duration::from_millis(interval));
    }
}
