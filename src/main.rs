use std::io::prelude::*;
use std::net::{ Ipv4Addr, SocketAddr, TcpStream };
use clap::{Parser, Subcommand};

mod protocol;
use protocol::{ Protocol, nom_helper::hexadecimal_u16_value };
use protocol::hardware::Hardware;

use nix::unistd::daemon;

mod daemon;
use daemon::Daemon;


#[derive(Parser)]
struct Cli {
    /// Sets Sermatec ESS Ipv4Addr
    #[arg(short ='i', long, default_value="10.10.100.254", value_name = "Inverter IPv4")]
    inverter: Option<Ipv4Addr>,

    /// Sets Sermatec ESS Port number
    #[arg(short ='p', long, default_value="8899", value_name = "Port number")]
    port: Option<u16>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

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
        #[arg(short ='m', long)]
        host: String,
        /// MQTT Server TCP port
        #[arg(short ='t', long, default_value="1883")]
        port: u16,
        /// waiting time between two updates (seconds)
        #[arg(short ='w', long, default_value="300")]
        wait: u16,
        /// Detaching from the controlling terminal
        #[arg(short ='f', long)]
        fork: bool,
    },
}


fn main() -> std::io::Result<()> {
    let p = Protocol::new();
    let cli = Cli::parse();

    let sermatec_ip: Ipv4Addr = if let Some(inverter) = cli.inverter {
        inverter
    } else {
        Ipv4Addr::new(10, 10, 100, 254)
    };
    let sermatec_port: u16 = if let Some(port) = cli.port {
        port
    } else {
        8899
    };
    println!("--===~ Sermatec ESS CLI AND MQTT PROXY ~===--");
    println!("Asking to Sermatec Inverter {:?}:{}", sermatec_ip, sermatec_port);
    let sermatec_socket: SocketAddr = SocketAddr::from((sermatec_ip, sermatec_port));
    let mut stream = TcpStream::connect(sermatec_socket)?;


    let hardware = Hardware::get_info(&p, &mut stream).unwrap();

    // Next step ask a real question...
    match &cli.command {
        Some(Commands::Get { el }) => {
            if *el == "BB" {
                println!("SECURITY ISSUE: Denial App Access. See README.md");
                // 0x4065 0x4119 0x409d 0x4080 0x4088 0x410d 0x4054 0x4053
            } else if *el == "98" {
                // special print
                println!("{:?}", &hardware);
            }
            else {
                let (_input, c) = hexadecimal_u16_value(&el).unwrap();
                let cmds = p["osim"].get_commands(hardware.pcu_version);
                let cmd = cmds[&c]; // TODO: check if c exist
                println!("Getting {:02X} ({})...", c, cmd.comment);
                let packet = cmd.build_packet().unwrap();
                stream.write(&packet)?;
                let elements = cmd.parse_answer(&mut stream);
                cmd.print_nice_answer(&elements);
            }
        },
        Some(Commands::List {}) => {
            println!("listing commands:\n");
            p["osim"].listing(hardware.pcu_version);
        },
        Some(Commands::Daemon { host, port , fork, wait}) => {
            if *fork {
                println!("Detaching from terminal");
                daemon(true, false).unwrap();
            }
            println!("Sending data to MQTT Daemon {}:{}\n", host, port);
            let cmds = p["osim"].get_commands(hardware.pcu_version);
            let mut daemon = Daemon::new(hardware, sermatec_socket, stream, host, *port, cmds, *wait);
            daemon.run();
        },
        None => {}
    }
    Ok(())
}
