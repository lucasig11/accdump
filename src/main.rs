use anyhow::Result;
use clap::arg_enum;
use clap::Parser;
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use std::fmt::Display;
use std::io::Write;

#[derive(clap::Parser)]
struct Args {
    account: Pubkey,
    #[clap(short, long = "mode", default_value = "hex")]
    mode: DisplayMode,
    #[clap(short, long, default_value = "http://localhost:8899")]
    url: String,
}

arg_enum! {
    enum DisplayMode {
        Hex,
        Dec,
    }
}

fn main() -> Result<()> {
    let args = Args::parse();
    let rpc = RpcClient::new(args.url);
    let data = rpc.get_account_data(&args.account)?;
    let bytes = AccountDataDisplay(data, args.mode);
    let mut stdout = std::io::stdout();
    stdout.write_all(bytes.to_string().as_ref())?;
    Ok(())
}

struct AccountDataDisplay(Vec<u8>, DisplayMode);
impl Display for AccountDataDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let AccountDataDisplay(data, mode) = self;

        for bytes in data.chunks(8) {
            let bytes_str = bytes
                .iter()
                .enumerate()
                .map(|(i, byte)| {
                    let byte = match mode {
                        DisplayMode::Dec => format!("{:03}", byte),
                        DisplayMode::Hex => format!("{:02x}", byte),
                    };
                    let pad = if i == 3 { 2 } else { 1 };
                    format!("{byte}{:pad$}", " ")
                })
                .collect::<String>();
            let ascii_str = bytes
                .iter()
                .map(|byte| {
                    format!(
                        "{:^2}",
                        match char::from_u32(*byte as u32) {
                            Some(ch) if ch.is_alphanumeric() => ch,
                            _ => '.',
                        }
                    )
                })
                .collect::<String>();

            f.write_fmt(format_args!("{bytes_str:<25}|{ascii_str:>17}|\n"))?;
        }

        Ok(())
    }
}
