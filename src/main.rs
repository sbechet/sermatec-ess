use clap::{Parser, Subcommand};
use std::io::prelude::*;
use std::net::{Ipv4Addr, SocketAddr, TcpStream};

mod protocol;
use protocol::hardware::Hardware;
use protocol::{nom_helper::hexadecimal_u16_value, Protocol};

use nix::unistd::daemon;

mod daemon;
use daemon::Daemon;

#[derive(Parser)]
struct Cli {
    /// Sets Sermatec ESS Ipv4Addr
    #[arg(
        short = 'i',
        long,
        default_value = "10.10.100.254",
        value_name = "Inverter IPv4"
    )]
    inverter_host: Option<Ipv4Addr>,

    /// Sets Sermatec ESS Port number
    #[arg(short = 'p', long, default_value = "8899", value_name = "Port number")]
    inverter_port: Option<u16>,

    /// Turn debugging information on or off
    #[arg(short, long, default_value = "false")]
    debug: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Get a specific things
    Get {
        /// ask for a specific command
        #[arg(short, long)]
        el: String,
    },
    /// Get listing of all things
    List {},
    /// Daemon mode use sermatec-ess as a MQTT client
    Daemon {
        /// MQTT Server hostname
        #[arg(short = 'm', long)]
        mqtt_host: String,
        /// MQTT Server TCP port
        #[arg(short = 't', long, default_value = "1883")]
        mqtt_port: u16,
        /// waiting time between two updates (seconds)
        #[arg(short = 'w', long, default_value = "300")]
        wait: u16,
        /// Detaching from the controlling terminal
        #[arg(short = 'f', long)]
        fork: bool,
    },
}

fn main() -> std::io::Result<()> {
    let p = Protocol::new();
    let cli = Cli::parse();

    println!("--===~ Sermatec ESS CLI AND MQTT PROXY ~===--");
    println!(
        "Asking to Sermatec Inverter {:?}:{}",
        cli.inverter_host.unwrap(),
        cli.inverter_port.unwrap()
    );
    let inverter_socket: SocketAddr =
        SocketAddr::from((cli.inverter_host.unwrap(), cli.inverter_port.unwrap()));

    let mut inverter_stream = TcpStream::connect(inverter_socket)?;
    let hardware = Hardware::get_info(&p, &mut inverter_stream).unwrap();

    // Next step ask a real question...
    match &cli.command {
        Some(Commands::Get { el }) => {
            if *el == "BB" {
                println!("SECURITY ISSUE: Denial App Access. See README.md");
                // 0x4065 0x4119 0x409d 0x4080 0x4088 0x410d 0x4054 0x4053
            } else {
                let (_input, c) = hexadecimal_u16_value(&el).unwrap();
                let cmds = p["osim"].get_commands(hardware.pcu_version);
                let cmd = cmds[&c]; // TODO: check if c exist
                println!("Getting {:02X} ({})...", c, cmd.comment);
                let packet = cmd.build_packet().unwrap();
                inverter_stream.write(&packet)?;
                let elements = cmd.parse_answer(&mut inverter_stream);
                cmd.print_nice_answer(&elements);
            }
        }
        Some(Commands::List {}) => {
            println!("listing commands:\n");
            p["osim"].listing(hardware.pcu_version);
        }
        Some(Commands::Daemon {
            mqtt_host,
            mqtt_port,
            fork,
            wait,
        }) => {
            if *fork {
                println!("Detaching from terminal");
                daemon(true, false).unwrap();
            }
            println!("Sending data to MQTT Daemon {}:{}\n", mqtt_host, mqtt_port);
            let cmds = p["osim"].get_commands(hardware.pcu_version);
            let mut daemon = Daemon::new(
                hardware,
                inverter_socket,
                inverter_stream,
                mqtt_host,
                *mqtt_port,
                cmds,
                *wait,
                cli.debug,
            );
            daemon.run();
        }
        None => {}
    }
    Ok(())
}
