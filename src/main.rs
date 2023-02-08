use std::io::prelude::*;
use std::net::{ Ipv4Addr, SocketAddr, TcpStream };
use clap::{Parser, Subcommand};

mod protocol;
use protocol::{ Protocol, FieldType };


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
    println!("# Inverter\n\n{:?}:{}\n", sermatec_ip, sermatec_port);
    let sermatec_socket: SocketAddr = SocketAddr::from((sermatec_ip, sermatec_port));
    let mut stream = TcpStream::connect(sermatec_socket)?;

    // First step: Query 98@version0 to know real version
    let command = p["osim"].get_command(0, "98").unwrap();
    println!("# JSON Command\n\n{:#?}\n", command);
    let packet = command.build_packet().unwrap();
    println!("# Question\n\n{:x?}\n", packet);
    stream.write(&packet)?;

    let elements = command.parse_answer(98,&mut stream);
    // Print results for debug purpose
    println!("# {}\n\n{:#?}\n", command.comment, elements);
    let mut pcu_version = 9999;
    match elements {
        Ok(elts) => {
            for e in &elts {
                if e.0 == "pcuVersion" {
                    if let FieldType::Int(v) = e.2 {
                        pcu_version = v;
                        break;
                    }
                }
            }
        },
        Err(e) => {
            println!("Parsing Error: {:?}", e);
        }
    };

    // Next step ask a real question...
    match &cli.command {
        Some(Commands::Get { el }) => {
            if *el != "98" {
                println!("Getting {}...TODO", el);
                // TODO: howto check good commands compatible versions
            }
        },
        Some(Commands::List {}) => {
            println!("listing all...");
            // TODO: add minimal API version
            // TODO: for now implement op 1
            // TODO: one day later add op 2 and op 3 API
            p["osim"].listing(pcu_version, 1);
        },
        None => {}
    }
    Ok(())
}
