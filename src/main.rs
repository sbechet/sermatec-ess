use std::io::prelude::*;
use std::net::{ Ipv4Addr, SocketAddr, TcpStream };
use clap::{Parser, Subcommand};

mod protocol;
use protocol::{ Protocol, nom_helper::hexadecimal_u16_value };
use protocol::fieldtype::FieldType;

mod daemon;
use daemon::Daemon;


#[derive(Parser)]
struct Cli {
    /// Sets Sermatec ESS Ipv4Addr
    #[arg(short ='i', long, value_name = "IPv4 (default 10.10.100.254)")]
    inverter: Option<Ipv4Addr>,

    /// Sets Sermatec ESS Port number
    #[arg(short ='p', long, value_name = "Port number (default 8899)")]
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
    /// Daemon mode: sermatec-ess as a MQTT client
    Daemon {
        /// MQTT Server hostname
        #[arg(short ='m', long)]
        host: String,
        /// MQTT Server TCP port
        #[arg(short ='t', long)]
        port: u16,
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
    println!("--===~ Sermatec ESS CLI ~===--");
    println!("Asking to {:?}:{}\n", sermatec_ip, sermatec_port);
    let sermatec_socket: SocketAddr = SocketAddr::from((sermatec_ip, sermatec_port));
    let mut stream = TcpStream::connect(sermatec_socket)?;

    let command = p["osim"].get_command(0, "98").unwrap();
    let packet = command.build_packet().unwrap();
    stream.write(&packet)?;

    let elements = command.parse_answer(&mut stream);
    let mut pcu_version: i16 = 0;
    match &elements {
        Ok(elts) => {
            for fa in elts {
                if fa.f.tag == "pcuVersion" {
                    if let FieldType::Int(v) = fa.v {
                        pcu_version = v as i16;
                        break;
                    }
                }
            }
        },
        Err(e) => {
            println!("Parsing Error: {:?}", e);
            return Ok(());
        }
    };
    let cmds = p["osim"].get_commands(pcu_version);

    // Next step ask a real question...
    match &cli.command {
        Some(Commands::Get { el }) => {
            if *el == "BB" {
                println!("SECURITY ISSUE: Denial App Access. See README.md");
            } else if *el == "98" {
                command.print_nice_answer(&elements);
            }
            else {
                let (_input, c) = hexadecimal_u16_value(&el).unwrap();
                let cmd = cmds[&c]; // TODO: check if c exist
                println!("Getting {} ({})...", c, cmd.comment);
                let packet = cmd.build_packet().unwrap();
                stream.write(&packet)?;
                let elements = cmd.parse_answer(&mut stream);
                cmd.print_nice_answer(&elements);
            }
        },
        Some(Commands::List {}) => {
            println!("listing commands:\n");
            p["osim"].listing(pcu_version);
        },
        Some(Commands::Daemon { host, port }) => {
            let daemon = Daemon::new(host, *port);
            daemon.run(&mut stream, &cmds).unwrap();
        },
        None => {}
    }
    Ok(())
}
