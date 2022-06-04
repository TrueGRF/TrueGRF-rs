use clap::{Arg, Command};
use std::env;
use std::io::{Read, Write};

mod grf;


fn load_sprite_bytes(filename: &str) -> Vec<u8> {
    let mut bytes = Vec::new();
    std::fs::File::open(&filename).unwrap().read_to_end(&mut bytes).unwrap();

    bytes
}

fn main() {
    let matches = Command::new("TrueGRF Compiler")
        .arg(Arg::new("project")
                 .short('p')
                 .long("project")
                 .takes_value(true)
                 .help("Project folder"))
        .arg(Arg::new("output")
                 .short('o')
                 .long("output")
                 .takes_value(true)
                 .help("Output file"))
        .get_matches();

    let project = match matches.value_of("project") {
        None => {
            println!("No project folder specified");
            return;
        },
        Some(project) => project,
    };
    let output = match matches.value_of("output") {
        None => {
            println!("No output file specified");
            return;
        },
        Some(output) => output,
    };

    /* Change cwd, to make relative paths easier inside YAML files. */
    let cwd = env::current_dir().unwrap();
    env::set_current_dir(project).unwrap();

    /* Read the general part. */
    let fp = std::fs::File::open("truegrf.yaml").unwrap();

    let general: grf::NewGRFGeneral = serde_yaml::from_reader(fp).unwrap();

    let options = match general.r#type.as_str() {
        "industry" => {
            let mut options = grf::NewGRFConfigIndustry {
                general: general,
                cargoes: Vec::new(),
                industries: Vec::new(),
            };

            /* Read the cargoes. */
            std::fs::read_dir("cargoes").unwrap().for_each(|entry| {
                let entry = entry.unwrap();
                let path = entry.path();

                if path.is_file() && path.extension().unwrap() == "yaml" {
                    let fp = std::fs::File::open(path).unwrap();
                    options.cargoes.push(serde_yaml::from_reader(fp).unwrap());
                }
            });

            /* Read the industries. */
            std::fs::read_dir("industries").unwrap().for_each(|entry| {
                let entry = entry.unwrap();
                let path = entry.path();

                if path.is_file() && path.extension().unwrap() == "yaml" {
                    let fp = std::fs::File::open(path).unwrap();
                    options.industries.push(serde_yaml::from_reader(fp).unwrap());
                }
            });

            grf::NewGRFConfig::industry(options)
        }
        "townname" => {
            let mut options = grf::NewGRFConfigTownname {
                general: general,
                townnames: Vec::new(),
            };

            /* Read the townnames. */
            std::fs::read_dir("townnames").unwrap().for_each(|entry| {
                let entry = entry.unwrap();
                let path = entry.path();

                if path.is_file() && path.extension().unwrap() == "yaml" {
                    let fp = std::fs::File::open(path).unwrap();
                    options.townnames.push(serde_yaml::from_reader(fp).unwrap());
                }
            });

            grf::NewGRFConfig::townname(options)
        }
        _ => {
            println!("Unsupported TrueGRF type '{}'", general.r#type);
            return;
        }
    };

    /* Create the GRF. */
    match grf::write_grf(options, &load_sprite_bytes) {
        Ok(data) => {
            /* Restore original cwd, to ensure "output" ends up in the right place. */
            env::set_current_dir(cwd).unwrap();

            let mut fp = std::fs::File::create(output).unwrap();
            fp.write_all(&data).unwrap();

            println!("GRF written to {}", output);
        }
        Err(error) => {
            println!("Error when creating GRF: {}", error);
        },
    }
}
