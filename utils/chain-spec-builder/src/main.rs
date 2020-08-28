// Copyright 2019-2020 Parity Technologies (UK) Ltd.
// This file is part of Substrate.

// Substrate is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Substrate is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Substrate.  If not, see <http://www.gnu.org/licenses/>.

use std::{
    fs,
    path::{Path, PathBuf},
};

use ansi_term::Style;
use rand::{distributions::Alphanumeric, rngs::OsRng, Rng};
use structopt::StructOpt;

use joystream_node::chain_spec::{
    self, chain_spec_properties, forum_config, initial_members, proposals_config, AccountId,
};

use sc_chain_spec::ChainType;
use sc_keystore::Store as Keystore;
use sc_telemetry::TelemetryEndpoints;
use sp_core::{
    crypto::{Public, Ss58Codec},
    sr25519,
    traits::BareCryptoStore,
};

const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// A utility to easily create a testnet chain spec definition with a given set
/// of authorities and endowed accounts and/or generate random accounts.
#[derive(StructOpt)]
#[structopt(rename_all = "kebab-case")]
enum ChainSpecBuilder {
    /// Create a new chain spec with the given authorities, endowed and sudo
    /// accounts.
    New {
        /// Authority key seed.
        #[structopt(long, short, required = true)]
        authority_seeds: Vec<String>,
        /// Endowed account address (SS58 format).
        #[structopt(long, short)]
        endowed_accounts: Vec<String>,
        /// Sudo account address (SS58 format).
        #[structopt(long, short)]
        sudo_account: String,
        /// The path where the chain spec should be saved.
        #[structopt(long, short, default_value = "./chain_spec.json")]
        chain_spec_path: PathBuf,
        /// The path to an initial members data
        #[structopt(long, short)]
        initial_members_path: Option<PathBuf>,
        /// The path to an initial forum data
        #[structopt(long, short)]
        initial_forum_path: Option<PathBuf>,
    },
    /// Create a new chain spec with the given number of authorities and endowed
    /// accounts. Random keys will be generated as required.
    Generate {
        /// The number of authorities.
        #[structopt(long, short)]
        authorities: usize,
        /// The number of endowed accounts.
        #[structopt(long, short, default_value = "0")]
        endowed: usize,
        /// The path where the chain spec should be saved.
        #[structopt(long, short, default_value = "./chain_spec.json")]
        chain_spec_path: PathBuf,
        /// Path to use when saving generated keystores for each authority.
        ///
        /// At this path, a new folder will be created for each authority's
        /// keystore named `auth-$i` where `i` is the authority index, i.e.
        /// `auth-0`, `auth-1`, etc.
        #[structopt(long, short)]
        keystore_path: Option<PathBuf>,
        /// The path to an initial members data
        #[structopt(long, short)]
        initial_members_path: Option<PathBuf>,
        /// The path to an initial forum data
        #[structopt(long, short)]
        initial_forum_path: Option<PathBuf>,
    },
}

impl ChainSpecBuilder {
    /// Returns the path where the chain spec should be saved.
    fn chain_spec_path(&self) -> &Path {
        match self {
            ChainSpecBuilder::New {
                chain_spec_path, ..
            } => chain_spec_path.as_path(),
            ChainSpecBuilder::Generate {
                chain_spec_path, ..
            } => chain_spec_path.as_path(),
        }
    }

    /// Returns the path where the chain spec should be saved.
    fn initial_members_path(&self) -> &Option<PathBuf> {
        match self {
            ChainSpecBuilder::New {
                initial_members_path,
                ..
            } => initial_members_path,
            ChainSpecBuilder::Generate {
                initial_members_path,
                ..
            } => initial_members_path,
        }
    }

    /// Returns the path where the chain spec should be saved.
    fn initial_forum_path(&self) -> &Option<PathBuf> {
        match self {
            ChainSpecBuilder::New {
                initial_forum_path, ..
            } => initial_forum_path,
            ChainSpecBuilder::Generate {
                initial_forum_path, ..
            } => initial_forum_path,
        }
    }
}

fn genesis_constructor(
    authority_seeds: &[String],
    endowed_accounts: &[AccountId],
    sudo_account: &AccountId,
    initial_members_path: &Option<PathBuf>,
    initial_forum_path: &Option<PathBuf>,
) -> chain_spec::GenesisConfig {
    let authorities = authority_seeds
        .iter()
        .map(AsRef::as_ref)
        .map(chain_spec::get_authority_keys_from_seed)
        .collect::<Vec<_>>();

    let members = if let Some(path) = initial_members_path {
        initial_members::from_json(path.as_path())
    } else {
        initial_members::none()
    };

    let forum_cfg = if let Some(path) = initial_forum_path {
        forum_config::from_json(sudo_account.clone(), path.as_path())
    } else {
        forum_config::empty(sudo_account.clone())
    };

    chain_spec::testnet_genesis(
        authorities,
        sudo_account.clone(),
        endowed_accounts.to_vec(),
        proposals_config::default(),
        members,
        forum_cfg,
    )
}

fn generate_chain_spec(
    authority_seeds: Vec<String>,
    endowed_accounts: Vec<String>,
    sudo_account: String,
    initial_members_path: Option<PathBuf>,
    initial_forum_path: Option<PathBuf>,
) -> Result<String, String> {
    let parse_account = |address: &String| {
        AccountId::from_string(address)
            .map_err(|err| format!("Failed to parse account address: {:?}", err))
    };

    let endowed_accounts = endowed_accounts
        .iter()
        .map(parse_account)
        .collect::<Result<Vec<_>, String>>()?;

    let sudo_account = parse_account(&sudo_account)?;

    // let boot_nodes = vec![String::from(
    //     "/dns4/tesnet.joystream.org/tcp/30333/p2p/QmaTTdEF6YVCtynSjsXmGPSGcEesAahoZ8pmcCmmBwSE7S",
    // )];

    let telemetry_endpoints = TelemetryEndpoints::new(vec![(STAGING_TELEMETRY_URL.to_string(), 0)])
        .map_err(|err| format!("Failed to create telemetry endpoints: {:?}", err))?;

    let chain_spec = chain_spec::ChainSpec::from_genesis(
        "Joystream Testnet",
        "joy_testnet",
        ChainType::Development,
        move || {
            genesis_constructor(
                &authority_seeds,
                &endowed_accounts,
                &sudo_account,
                &initial_members_path,
                &initial_forum_path,
            )
        },
        vec![],
        Some(telemetry_endpoints),
        Some(&*"/joy/testnet/0"),
        Some(chain_spec_properties()),
        None,
    );

    chain_spec.as_json(false).map_err(|err| err)
}

fn generate_authority_keys_and_store(seeds: &[String], keystore_path: &Path) -> Result<(), String> {
    for (n, seed) in seeds.iter().enumerate() {
        let keystore = Keystore::open(keystore_path.join(format!("auth-{}", n)), None)
            .map_err(|err| err.to_string())?;

        let (_, _, grandpa, babe, im_online, _) = chain_spec::get_authority_keys_from_seed(seed);

        let insert_key = |key_type, public| {
            keystore
                .write()
                .insert_unknown(key_type, &format!("//{}", seed), public)
                .map_err(|_| format!("Failed to insert key: {}", grandpa))
        };

        insert_key(sp_core::crypto::key_types::BABE, babe.as_slice())?;

        insert_key(sp_core::crypto::key_types::GRANDPA, grandpa.as_slice())?;

        insert_key(sp_core::crypto::key_types::IM_ONLINE, im_online.as_slice())?;
    }

    Ok(())
}

fn print_seeds(authority_seeds: &[String], endowed_seeds: &[String], sudo_seed: &str) {
    let header = Style::new().bold().underline();
    let entry = Style::new().bold();

    println!("{}", header.paint("Authority seeds"));

    for (n, seed) in authority_seeds.iter().enumerate() {
        println!("{} //{}", entry.paint(format!("auth-{}:", n)), seed,);
    }

    println!();

    if !endowed_seeds.is_empty() {
        println!("{}", header.paint("Endowed seeds"));
        for (n, seed) in endowed_seeds.iter().enumerate() {
            println!("{} //{}", entry.paint(format!("endowed-{}:", n)), seed,);
        }

        println!();
    }

    println!("{}", header.paint("Sudo seed"));
    println!("//{}", sudo_seed);
}

fn main() -> Result<(), String> {
    #[cfg(build_type = "debug")]
    println!(
		"The chain spec builder builds a chain specification that includes a Substrate runtime compiled as WASM. To \
		 ensure proper functioning of the included runtime compile (or run) the chain spec builder binary in \
		 `--release` mode.\n",
	);

    let builder = ChainSpecBuilder::from_args();
    let chain_spec_path = builder.chain_spec_path().to_path_buf();
    let initial_members_path = builder.initial_members_path().clone();
    let initial_forum_path = builder.initial_forum_path().clone();

    let (authority_seeds, endowed_accounts, sudo_account) = match builder {
        ChainSpecBuilder::Generate {
            authorities,
            endowed,
            keystore_path,
            ..
        } => {
            let authorities = authorities.max(1);
            let rand_str = || -> String { OsRng.sample_iter(&Alphanumeric).take(32).collect() };

            let authority_seeds = (0..authorities).map(|_| rand_str()).collect::<Vec<_>>();
            let endowed_seeds = (0..endowed).map(|_| rand_str()).collect::<Vec<_>>();
            let sudo_seed = rand_str();

            print_seeds(&authority_seeds, &endowed_seeds, &sudo_seed);

            if let Some(keystore_path) = keystore_path {
                generate_authority_keys_and_store(&authority_seeds, &keystore_path)?;
            }

            let endowed_accounts = endowed_seeds
                .iter()
                .map(|seed| {
                    chain_spec::get_account_id_from_seed::<sr25519::Public>(seed).to_ss58check()
                })
                .collect();

            let sudo_account_id =
                chain_spec::get_account_id_from_seed::<sr25519::Public>(&sudo_seed);
            let sudo_account = sudo_account_id.clone().to_ss58check();

            (authority_seeds, endowed_accounts, sudo_account)
        }
        ChainSpecBuilder::New {
            authority_seeds,
            endowed_accounts,
            sudo_account,
            ..
        } => (authority_seeds, endowed_accounts, sudo_account),
    };

    let json = generate_chain_spec(
        authority_seeds,
        endowed_accounts,
        sudo_account,
        initial_members_path,
        initial_forum_path,
    )?;

    fs::write(chain_spec_path, json).map_err(|err| err.to_string())
}
