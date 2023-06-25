use clap::{Parser, ValueEnum};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
/// Templater is a tool to help you get started quickly configuring cloud apps, libraries,
/// and frameworks.
struct Args {
    /// run `templater list` to see all available options.
    #[arg(value_enum)]
    config: Config,

    #[clap(short, long)]
    /// whether or not to output in the current directory
    output_file: bool,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Config {
    DigitalOceanApp,
    Kubernetes,
}

fn main() {
    let args = Args::parse();

    let output = args.output_file;

    match args.config {
        Config::DigitalOceanApp => {
            let config = std::fs::read_to_string("./src/configs/digitalocean_app_spec.yaml").unwrap();

            println!("{}", &config);

            if output {
                std::fs::write("./digitalocean_app_spec.yaml", &config).unwrap();
                println!("outputted config to current directory");
            }
        },
        Config::Kubernetes => println!("lib2"),
        _ => println!("please choose from the following"),
    }
}
