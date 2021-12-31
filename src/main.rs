use anchor_lang::{prelude::Pubkey, AccountDeserialize, AnchorDeserialize, AnchorSerialize};
use gumdrop::Options;
use nft_candy_machine_v2::{
    CandyMachine, Creator, EndSettings, GatekeeperConfig, HiddenSettings, WhitelistMintSettings,
};
use rusqlite::Connection;
use solana_client::rpc_client::RpcClient;
use solana_sdk::account::ReadableAccount;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse_args_default_or_exit();
    match args.clone().command {
        None => todo!(),
        Some(command) => match command {
            Command::MineTransactions(opts) => mine_transactions(args, opts).await,
            Command::ShowCandyMachine(opts) => show_candy_machine(args, opts).await,
        },
    }
}

async fn mine_transactions(args: Args, _opts: CandyMachineOptions) -> Result<(), Box<dyn Error>> {
    let _db = Connection::open(args.db)?;
    Ok(())
}

async fn show_candy_machine(args: Args, opts: CandyMachineOptions) -> Result<(), Box<dyn Error>> {
    // let _db = Connection::open(args.db)?;
    let rpc = RpcClient::new(args.rpc);

    let candy_machine = opts.id.parse()?;
    let candy_machine = rpc.get_account(&candy_machine)?;
    let candy_machine = &mut candy_machine.data();
    let candy_machine = CandyMachine::try_deserialize(candy_machine)?;

    eprintln!("authority: {}", candy_machine.authority.to_string());
    eprintln!("wallet: {}", candy_machine.wallet.to_string());
    eprintln!("token_mint: {:?}", candy_machine.token_mint);
    eprintln!("items_redeemed: {}", candy_machine.items_redeemed);
    eprintln!("{}", "data");
    eprintln!(" uuid: {}", candy_machine.data.uuid);
    eprintln!(" price: {}", candy_machine.data.price);
    eprintln!(" symbol: {}", candy_machine.data.symbol);
    eprintln!(
        " seller_fee_basis_points: {}",
        candy_machine.data.seller_fee_basis_points
    );
    eprintln!(" max_supply: {}", candy_machine.data.max_supply);
    eprintln!(" is_mutable: {}", candy_machine.data.is_mutable);
    eprintln!(" retain_authority: {}", candy_machine.data.retain_authority);
    eprintln!(
        " go_live_date: {}",
        format_go_live_date(candy_machine.data.go_live_date)
    );
    eprintln!(
        " end_settings: {}",
        format_end_settings(candy_machine.data.end_settings)
    );
    eprintln!(
        " creators: {}",
        format_creators(candy_machine.data.creators)
    );
    eprintln!(
        " hidden_settings: {}",
        format_hidden_settings(candy_machine.data.hidden_settings)
    );
    eprintln!(
        " whitelist_mint_settings: {}",
        format_whitelist_mint_settings(candy_machine.data.whitelist_mint_settings)
    );
    eprintln!(" items_available: {}", candy_machine.data.items_available);
    eprintln!(
        " gatekeeper: {}",
        format_gatekeeper(candy_machine.data.gatekeeper)
    );

    Ok(())
}

fn format_creators(creators: Vec<Creator>) -> String {
    let mut result: Vec<String> = vec![];
    for creator in creators {
        let x = format!(
            "\n  address: {}; verified: {}; share: {}",
            creator.address.to_string(),
            creator.verified,
            creator.share
        );
        result.push(x);
    }
    result.join("\n  ")
}

fn format_end_settings(endsettings: Option<EndSettings>) -> String {
    match endsettings {
        None => "None".to_string(),
        Some(_) => todo!(),
    }
}

fn format_gatekeeper(gatekeeper: Option<GatekeeperConfig>) -> String {
    match gatekeeper {
        None => "None".to_string(),
        Some(gatekeeper_config) => {
            let mut raw: &[u8] = &gatekeeper_config.try_to_vec().unwrap();
            #[derive(AnchorDeserialize)]
            struct DebugGatekeeperConfig {
                gatekeeper_network: Pubkey,
                expire_on_use: bool,
            }
            let dbg = DebugGatekeeperConfig::deserialize(&mut raw).unwrap();
            format!(
                "\n  gatekeeper_network: {}\n  expire_on_use: {}",
                dbg.gatekeeper_network, dbg.expire_on_use
            )
        }
    }
}

fn format_go_live_date(go_live_date: Option<i64>) -> String {
    match go_live_date {
        None => "None".to_string(),
        Some(go_live_date) => format!("{}", go_live_date),
    }
}

fn format_hidden_settings(hiddensettings: Option<HiddenSettings>) -> String {
    match hiddensettings {
        None => "None".to_string(),
        Some(_) => todo!(),
    }
}

fn format_whitelist_mint_settings(_: Option<WhitelistMintSettings>) -> String {
    "TODO".to_string()
}

#[derive(Clone, Debug, Options)]
struct Args {
    #[options(help = "slite db path")]
    db: String,
    #[options(help = "rpc server", default_expr = "default_rpc_url()", meta = "r")]
    rpc: String,
    #[options(command)]
    command: Option<Command>,
}

fn default_rpc_url() -> String {
    "https://api.mainnet-beta.solana.com".to_owned()
}

#[derive(Clone, Debug, Options)]
enum Command {
    MineTransactions(CandyMachineOptions),
    ShowCandyMachine(CandyMachineOptions),
}

#[derive(Clone, Debug, Options)]
struct CandyMachineOptions {
    #[options(default_expr = "default_public_candy_machine_address()")]
    id: String,
    #[options(default_expr = "default_public_candy_machine_name()")]
    name: String,
}

fn default_public_candy_machine_address() -> String {
    "5GcgyzujqfF6Rh896tKa5EBAXvM9hyXGUtYaUyBLiXco".to_owned()
}

fn default_public_candy_machine_name() -> String {
    "public".to_owned()
}
