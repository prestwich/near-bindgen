use crate::account::*;
use crate::agent::Agent;
use crate::asset::*;
use crate::rate::*;
use borsh::{BorshDeserialize, BorshSerialize};
use near_bindgen::{env, near_bindgen};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type AccountId = Vec<u8>;

#[near_bindgen]
#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
pub struct MissionControl {
    account: Account,
    agents: HashMap<AccountId, Agent>,
    rates: HashMap<Exchange, Rate>,
}

#[near_bindgen]
impl MissionControl {
    pub fn add_agent(&mut self) {
        let account_id = env::signer_account_id().as_bytes().to_vec();
        self.agents.insert(account_id, Agent { account: agent_default(), is_alive: true });
    }

    pub fn assets_quantity(&self, account_id: String, asset: Asset) -> Option<Quantity> {
        let account_id = account_id.into_bytes();
        self.agents.get(&account_id).and_then(|agent| (agent.account.0).get(&asset).cloned())
    }

    pub fn simulate(&mut self, account_id: String) -> Option<bool> {
        let account_id = account_id.into_bytes();
        let Self { agents, rates, account } = self;
        agents.get_mut(&account_id).map(|agent| {
            agent.simulate(rates, account);
            agent.is_alive
        })
    }
}

impl Default for MissionControl {
    fn default() -> Self {
        Self { account: mission_default(), agents: Default::default(), rates: rates_default() }
    }
}

fn mission_default() -> Account {
    Account(hashmap![
        Asset::MissionTime => Quantity(1000000),
    ])
}

fn agent_default() -> Account {
    Account(hashmap![
        Asset::MissionTime => Quantity(1),
        Asset::Trust => Quantity(10000),
        Asset::Resource(Resource::Battery) => Quantity(10000),
        Asset::Resource(Resource::RgbSensor) => Quantity(10000),
        Asset::Resource(Resource::ThermalSensor) => Quantity(10000),
        Asset::Resource(Resource::PoseEstimation) => Quantity(10000),
    ])
}

fn rates_default() -> HashMap<Exchange, Rate> {
    hashmap![
        Exchange::MissionTimeWithResource =>
        Rate {
            credit: hashmap![Asset::MissionTime => Quantity(1)],
            debit: hashmap![
                Asset::Resource(Resource::Battery) => Quantity(20),
                Asset::Resource(Resource::ThermalSensor) => Quantity(9),
                Asset::Resource(Resource::RgbSensor) => Quantity(3),
                Asset::Resource(Resource::PoseEstimation) => Quantity(1),
            ],
        },
        Exchange::MissionTimeWithTrust =>
        Rate {
            credit: hashmap![Asset::MissionTime => Quantity(1)],
            debit: hashmap![Asset::Trust => Quantity(1)],
        },
    ]
}

#[cfg(feature = "env_test")]
#[cfg(test)]
mod tests {
    use super::*;
    use near_bindgen::MockedBlockchain;
    use near_bindgen::{testing_env, Config, VMContext};

    fn get_context(input: Vec<u8>) -> VMContext {
        VMContext {
            current_account_id: "alice.near".to_string(),
            signer_account_id: "bob.near".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "carol.near".to_string(),
            input,
            block_index: 0,
            account_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(9),
            random_seed: vec![0, 1, 2],
            free_of_charge: false,
            output_data_receivers: vec![],
        }
    }

    #[test]
    fn add_agent() {
        let context = get_context(vec![]);
        let account_id = context.signer_account_id.clone();
        let config = Config::default();
        testing_env!(context, config);

        let mut contract = MissionControl::default();
        contract.add_agent();
        assert_eq!(Some(true), contract.simulate(account_id.clone()));
        assert_eq!(
            Some(Quantity(2)),
            contract.assets_quantity(account_id.clone(), Asset::MissionTime)
        );
    }
}
