#![cfg_attr(not(feature = "std"), no_std, no_main)]
extern crate alloc;

use pink_extension as pink;
use ink::env;

#[pink::contract(env=PinkEnvironment)]
#[pink(inner=ink::contract)]
mod phalaworld {
    use super::pink;
    use core::fmt;
    use alloc::format;
    use alloc::string::String;
    use alloc::vec;
    use alloc::vec::Vec;
    use ink::storage::Mapping;
    use ink::storage::traits::StorageLayout;
    use pink::{PinkEnvironment, http_post};
    use pink::chain_extension::pink_extension_instance as ext;
    use pink_subrpc::{get_ss58addr_version, Ss58Codec};
    use serde::Deserialize;
    use scale::{Decode, Encode};

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, StorageLayout))]
    pub enum RarityType {
        Prime,
        Magic,
        Legendary,
    }

    impl fmt::Display for RarityType {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                RarityType::Prime => write!(f, "Prime"),
                RarityType::Magic => write!(f, "Magic"),
                RarityType::Legendary => write!(f, "Legendary"),
            }
        }
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, StorageLayout))]
    pub enum RaceType {
        Cyborg,
        AISpectre,
        XGene,
        Pandroid,
    }

    impl fmt::Display for RaceType {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                RaceType::Cyborg => write!(f, "Cyborg"),
                RaceType::AISpectre => write!(f, "AISpectre"),
                RaceType::XGene => write!(f, "XGene"),
                RaceType::Pandroid => write!(f, "Pandroid"),
            }
        }
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, StorageLayout))]
    pub enum CareerType {
        HackerWizard,
        HardwareDruid,
        RoboWarrior,
        TradeNegotiator,
        Web3Monk,
    }

    impl fmt::Display for CareerType {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                CareerType::HackerWizard => write!(f, "HackerWizard"),
                CareerType::HardwareDruid => write!(f, "HardwareDruid"),
                CareerType::RoboWarrior => write!(f, "RoboWarrior"),
                CareerType::TradeNegotiator => write!(f, "TradeNegotiator"),
                CareerType::Web3Monk => write!(f, "Web3Monk"),
            }
        }
    }

    #[derive(Decode, Encode)]
    #[cfg_attr(
        feature = "std",
        derive(Debug, PartialEq, Eq, scale_info::TypeInfo, StorageLayout)
    )]
    pub struct Nft {
        id: u32,
        owner: AccountId,
        generation: u32,
        rarity: RarityType,
        race: RaceType,
        career: CareerType,
    }

    #[derive(Decode, Encode)]
    #[cfg_attr(
        feature = "std",
        derive(Debug, PartialEq, Eq, scale_info::TypeInfo, StorageLayout)
    )]
    pub struct NftAttribtue {
        trait_type: String,
        value: String,
    }

    #[derive(Decode, Encode)]
    #[cfg_attr(
        feature = "std",
        derive(Debug, PartialEq, Eq, scale_info::TypeInfo, StorageLayout)
    )]
    pub struct NftMetadata {
        name: String,
        description: String,
        external_url: String,
        image: String,
        attributes: Vec<NftAttribtue>,
    }

    #[derive(Decode, Encode)]
    #[cfg_attr(
        feature = "std",
        derive(Debug, PartialEq, Eq, scale_info::TypeInfo, StorageLayout)
    )]
    pub struct SpiritAttributes {
        int: u32,
        dex: u32,
        will: u32,
        str: u32,
        updated_at: Option<u64>,
    }

    #[derive(Decode, Encode)]
    #[cfg_attr(
        feature = "std",
        derive(Debug, PartialEq, Eq, scale_info::TypeInfo, StorageLayout)
    )]
    pub struct KhalaComputationStats {
        total_idle_worker_count: u64,
        total_delegation_value: String,
    }

    #[derive(Decode, Encode)]
    #[cfg_attr(
        feature = "std",
        derive(Debug, PartialEq, Eq, scale_info::TypeInfo, StorageLayout)
    )]
    pub enum TokenIdOrAddress {
        TokenId(u32),
        Address(AccountId),
    }

    #[derive(Encode, Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        TokenNotFound,
        InvalidStatusCode,
        InvalidBody,
        ParseError,
        InvalidAddress,
        ExecuteError,

        NotOverlord,

        NotReady,
        ProvenFailed,
    }

    #[ink(storage)]
    pub struct PhalaWorld {
        overlord: AccountId,
        name: String,
        description: String,
        total_nfts: u32,
        nfts: Mapping<u32, Nft>,
        generation: u32,
        proven_formula: Option<String>,
        init_attributes: SpiritAttributes,
    }

    ///

    #[derive(Debug, Deserialize)]
    struct DelegationQueryResponse {
        data: DelegationQueryData,
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct DelegationQueryData {
        base_pools_connection: BasePoolsConnection,
        delegations_connection: DelegationsConnection,
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct BasePoolsConnection {
        edges: Vec<BasePoolsEdge>,
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct BasePoolsEdge {
        node: BasePoolNode,
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct BasePoolNode {
        stake_pool: Option<StakePoolNode>,
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct StakePoolNode {
        // id: String,
        idle_worker_count: u64,
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct DelegationsConnection {
        edges: Vec<DelegationsEdge>,
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct DelegationsEdge {
        node: DelegationNode,
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct DelegationNode {
        value: String,
    }

    ///

    #[derive(Debug, Deserialize)]
    struct ProvenResult {
        int: u32,
        str: u32,
        dex: u32,
        will: u32,
    }

    ///

    impl PhalaWorld {
        #[ink(constructor)]
        pub fn create(name: String, description: Option<String>) -> Self {
            Self {
                name: name.clone(),
                description: description.unwrap_or(String::from("")),
                overlord: Self::env().caller(),
                total_nfts: 0,
                nfts: Mapping::new(),
                generation: 0,
                proven_formula: None,
                init_attributes: SpiritAttributes {
                    int: 0,
                    dex: 0,
                    will: 0,
                    str: 0,
                    updated_at: None,
                },
            }
        }

        /// Get the NFT Collection name.
        ///
        /// @category Basic Information
        #[ink(message)]
        pub fn get_collection_name(&self) -> String {
            let val = self.name.clone();
            return val
        }

        /// Get the NFT Collection description.
        ///
        /// @category Basic Information
        #[ink(message)]
        pub fn get_collection_description(&self) -> String {
            let val = self.description.clone();
            return val
        }

        /// Get the initial attributes for a bew Spirit Nft.
        ///
        /// @category Overlord Operations
        #[ink(message)]
        pub fn set_init_attributes(&mut self, payload: SpiritAttributes) {
            if self.overlord != self.env().caller() {
                panic!("Only overlord can set init attributes")
            }
            self.init_attributes = payload;
        }

        /// Set the description text for the NFT Collection.
        ///
        /// For the collection owner, it can update the NFT collection description anytime.
        ///
        /// # Arguments
        ///
        /// * `description` - The new description for the NFT Collection.
        ///
        /// @ui description widget codemirror
        /// @ui description options.lang markdown
        ///
        /// @category Overlord Operations
        #[ink(message)]
        pub fn set_collection_description(&mut self, description: String) {
            if self.overlord != Self::env().caller() {
                panic!("Only overlord can set collection description")
            }
            self.description = description;
        }

        /// Get the Account ID of overlord.
        ///
        /// @category Basic Information
        #[ink(message)]
        pub fn overlord(&self) -> AccountId {
            ink::env::debug_println!("call overlord");
            return self.overlord.clone()
        }

        /// Get the total number of NFTs minted.
        ///
        /// @category Nft
        #[ink(message)]
        pub fn total_minted(&self, race: Option<RaceType>) -> u32 {
            if race.is_none() {
                return self.total_nfts
            }
            let expected = race.unwrap();
            let mut counts = 0;
            let mut token_id = 0;
            while token_id < self.total_nfts {
                let nft = self.nfts.get(token_id);
                if nft.is_some() {
                    if nft.unwrap().race == expected {
                        counts += 1;
                    }
                }
                token_id += 1;
            }
            return counts
        }

        /// Get the total number of NFTs minted.
        ///
        /// @category Nft
        #[ink(message)]
        pub fn mint(&mut self, rarity: RarityType, race: RaceType, career: CareerType) -> Result<u32, Error> {
            let id = self.total_nfts;
            let nft = Nft {
                id,
                owner: Self::env().caller(),
                generation: self.generation,
                rarity,
                race,
                career,
            };
            self.nfts.insert(id, &nft);
            self.total_nfts += 1;
            Ok(id)
        }

        /// Get metadata of a specific NFT.
        ///
        /// @category Nft
        #[ink(message)]
        pub fn metadata_of(&self, token_id: u32) -> Result<NftMetadata, Error> {
            let nft = self.nfts.get(&token_id).ok_or(Error::TokenNotFound)?;
            let proven = self.prove_attributes(Some(nft.owner))?;

            let mut attrs: Vec<NftAttribtue> = vec![];
            attrs.push(NftAttribtue {
                trait_type: String::from("Generation"),
                value: format!("{}", nft.generation),
            });
            attrs.push(NftAttribtue {
                trait_type: String::from("Rarity"),
                value: format!("{}", nft.rarity),
            });
            attrs.push(NftAttribtue {
                trait_type: String::from("Race"),
                value: format!("{}", nft.race),
            });
            attrs.push(NftAttribtue {
                trait_type: String::from("Career"),
                value: format!("{}", nft.career),
            });
            attrs.push(NftAttribtue {
                trait_type: String::from("Dexterity"),
                value: format!("{}", proven.dex),
            });
            attrs.push(NftAttribtue {
                trait_type: String::from("Willpower"),
                value: format!("{}", proven.will),
            });
            attrs.push(NftAttribtue {
                trait_type: String::from("Intelligence"),
                value: format!("{}", proven.int),
            });
            attrs.push(NftAttribtue {
                trait_type: String::from("Strength"),
                value: format!("{}", proven.str),
            });

            Ok(NftMetadata {
                name: format!("{} #{}", self.name, token_id),
                description: self.description.clone(),
                external_url: String::from(""),
                image: String::from(""),
                attributes: attrs,
            })
        }

        /// Get basic information of a specific NFT.
        ///
        /// @category Nft
        #[ink(message)]
        pub fn nft_of(&self, token_id: u32) -> Result<Nft, Error> {
            let nft = self.nfts.get(&token_id).ok_or(Error::TokenNotFound)?;
            Ok(nft)
        }


        /// Get basic information of a list of NFT.
        ///
        /// If you want to get all NFTs, just pass `None` as the argument. Otherwise, you can pass a list of token IDs.
        ///
        /// @category Nft
        #[ink(message)]
        pub fn bulk_nft_of(&self, token_ids: Option<Vec<u32>>) -> Result<Vec<Nft>, Error> {
            let mut nfts: Vec<Nft> = vec![];
            if token_ids.is_none() {
                let mut token_id = 0;
                while token_id < self.total_nfts {
                    let nft = self.nfts.get(token_id);
                    if nft.is_some() {
                        nfts.push(nft.unwrap());
                    }
                    token_id += 1;
                }
            } else {
                for id in token_ids.unwrap() {
                    let nft = self.nfts.get(&id).ok_or(Error::TokenNotFound)?;
                    nfts.push(nft);
                }
            }
            Ok(nfts)
        }

        /// Set the proven formula, only available from overlord.
        ///
        /// @ui formula widget codemirror
        /// @ui formula options.lang javascript
        ///
        /// @category Overlord Operations
        #[ink(message)]
        pub fn set_proven_formula(&mut self, formula: String) -> Result<(), Error> {
            if Self::env().caller() != self.overlord {
                return Err(Error::NotOverlord)
            }
            self.proven_formula = Some(formula);
            Ok(())
        }

        /// Check the proven formula, only available from overlord.
        ///
        /// @category Overlord Operations
        #[ink(message)]
        pub fn get_proven_formula(&self) -> Result<Option<String>, Error> {
            if Self::env().caller() != self.overlord {
                return Err(Error::NotOverlord)
            }
            return Ok(self.proven_formula.clone())
        }

        /// Get the Spirit Attributes of a specific account, or the caller if not specified.
        ///
        /// @category Nft
        #[ink(message)]
        pub fn prove_attributes(&self, account: Option<AccountId>) -> Result<SpiritAttributes, Error> {
            if self.proven_formula.is_none() {
                return Err(Error::NotReady)
            }

            let target = account.unwrap_or(Self::env().caller());
            let account: [u8; 32] = *target.as_ref();
            let version = get_ss58addr_version("phala".into()).unwrap();
            let addr = account.to_ss58check_with_version(version.prefix());

            let khala_computation_factor = self.get_khala_computation_factor(addr.clone())?;

            let total_idle_worker_count: String = format!("{:?}", khala_computation_factor.total_idle_worker_count);
            let total_delegation_value: String = khala_computation_factor.total_delegation_value;

            let proven_formula = self.proven_formula.clone().unwrap();
            let js_code = format!(r#"
                const facts = {{
                    total_idle_worker_count: Number("{total_idle_worker_count}"),
                    total_delegation_value: Number("{total_delegation_value}"),
                }};
                {proven_formula}
            "#);
            // pink::debug!("js_code: {}", js_code);
            let result_raw = self.get_js_result(js_code, vec![])?;
            // pink::debug!("result: {}", result_raw);
            let result_u8: Vec<u8> = result_raw.as_bytes().to_vec();
            let result: ProvenResult = pink_json::from_slice(&*result_u8).or(Err(Error::ProvenFailed))?;

            Ok(SpiritAttributes {
                int: result.int + self.init_attributes.int,
                str: result.str + self.init_attributes.str,
                dex: result.dex + self.init_attributes.dex,
                will: result.will + self.init_attributes.will,
                updated_at: Some(ext().untrusted_millis_since_unix_epoch()),
            })
        }

        // TODO: transfer fee from caller to the overlord.
        fn get_khala_computation_factor(&self, ss58_address: String) -> Result<KhalaComputationStats, Error> {
            let url: String = "https://squid.subsquid.io/khala-computation/graphql".into();
            let query = format!(
                r#"{{
                    "query": "query Query {{ basePoolsConnection(orderBy: id_ASC, where: {{ owner: {{ id_eq: \"{ss58_address}\" }} }}) {{ edges {{ node {{ stakePool {{ id idleWorkerCount }} }} }} }} delegationsConnection(orderBy: id_ASC, where: {{ account: {{ id_eq: \"{ss58_address}\" }} }}) {{ edges {{ node {{ basePool {{ pid }} value }} }} }} }}",
                    "variables": null,
                    "operationName": "Query"
                }}"#
            );
            let headers: Vec<(String, String)> = vec![
                ("Content-Type".into(), "application/json".into()),
            ];
            let response = http_post!(&url, query, headers);
            if response.status_code != 200 {
                return Err(Error::InvalidStatusCode);
            }
            let payload: DelegationQueryResponse = pink_json::from_slice(&response.body)
                .or(Err(Error::InvalidBody))?;

            let mut idle_worker_count: u64 = 0;
            for edge in payload.data.base_pools_connection.edges {
                if edge.node.stake_pool.is_some() {
                    idle_worker_count += edge.node.stake_pool.unwrap().idle_worker_count;
                }
            }

            let mut values_literal: Vec<String> = vec![];
            for edge in payload.data.delegations_connection.edges {
                values_literal.push(format!("{:?}", edge.node.value));
            }
            let all_values = values_literal.join(",");

            let js_code = format!(
                r#"
                    (() => {{
                        let values = [{all_values}];
                        let total = values.reduce((a, b) => Number(a) + Number(b), 0);
                        return total
                    }})();
                "#
            );
            let result = self.get_js_result(js_code, vec![])?;

            Ok(KhalaComputationStats {
                total_idle_worker_count: idle_worker_count,
                total_delegation_value: result,
            })
        }

        fn get_js_result(&self, js_code: String, args: Vec<String>) -> Result<String, Error> {
            let output = phat_js::eval(&js_code, &args).unwrap();
            let output_as_bytes = match output {
                phat_js::Output::String(s) => s.into_bytes(),
                phat_js::Output::Bytes(b) => b,
            };
            Ok(String::from_utf8(output_as_bytes).unwrap())
        }
    }
}
