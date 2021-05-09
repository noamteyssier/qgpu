
use std::path::Path;
use shellexpand::tilde;
use clap::{
    Arg, App, SubCommand, AppSettings, ArgMatches
};


pub fn default_node_exists(matches: &ArgMatches) {

    let node_path = matches.value_of("nodes").unwrap();

    // check if default is given
    if node_path.eq("~/.qgpu_node_config.json") {

        // check if default exists in file system
        let node_path = tilde(node_path).into_owned();
        let path = Path::new(&node_path);

        // quits if missing
        if !path.exists() {
            println!("Error: Requires node config to be given if default ~/.qgpu_node_config.json is missing");
            std::process::exit(-1);
        }
    }

}

pub fn get_args() -> ArgMatches<'static> {

    let app = App::new("qgpu")
                    .version("0.1")
                    .author("Noam Teyssier <nb.teyssier@gmail.com>")
                    .about("A simple job scheduler with respect to gpu usage across multiple nodes")
                    .setting(AppSettings::SubcommandRequired)
                    .subcommand(
                        SubCommand::with_name("stat")
                            .about("retrieves usage statistics without submitting jobs")
                                .arg(Arg::with_name("nodes")
                                    .short("i")
                                    .long("node_config")
                                    .takes_value(true)
                                    .required(false)
                                    .default_value("~/.qgpu_node_config.json")
                                    .help("node config file (json formatted)"))
                            .arg(Arg::with_name("usage_free_threshold")
                                .short("f")
                                .long("free_usage")
                                .takes_value(true)
                                .required(false)
                                .default_value("95")
                                .help("free usage % required to consider a resource available (default = 95)"))
                            .arg(Arg::with_name("memory_free_threshold")
                                .short("F")
                                .long("free_memory")
                                .takes_value(true)
                                .required(false)
                                .default_value("95")
                                .help("free memory % required to consider a resource available (default = 95)"))
                         )
                     .subcommand(
                         SubCommand::with_name("sub")
                             .about("submits jobs on available resources")
                                 .arg(Arg::with_name("nodes")
                                     .short("i")
                                     .long("node_config")
                                     .takes_value(true)
                                     .required(false)
                                     .default_value("~/.qgpu_node_config.json")
                                     .help("node config file (json formatted)"))
                                 .arg(Arg::with_name("jobs")
                                     .short("j")
                                     .long("job_config")
                                     .takes_value(true)
                                     .required(true)
                                     .help("job config file (json formatted)"))
                                 .arg(Arg::with_name("dry")
                                     .short("d")
                                     .long("dry")
                                     .required(false)
                                     .takes_value(false)
                                     .help("dry run - will not submit jobs"))
                                 .arg(Arg::with_name("usage_free_threshold")
                                     .short("f")
                                     .long("free_usage")
                                     .takes_value(true)
                                     .required(false)
                                     .default_value("95")
                                     .help("free usage % required to consider a resource available (default = 95)"))
                                 .arg(Arg::with_name("memory_free_threshold")
                                     .short("F")
                                     .long("free_memory")
                                     .takes_value(true)
                                     .required(false)
                                     .default_value("95")
                                     .help("free memory % required to consider a resource available (default = 95)"))
                          );

    let matches = app.get_matches();

    // exits if default_node_config missing
    match matches.subcommand() {

        ("stat", Some(sub_m)) => default_node_exists(sub_m),
        ("sub", Some(sub_m)) => default_node_exists(sub_m),
        _ => unreachable!()

    };

    matches
}
