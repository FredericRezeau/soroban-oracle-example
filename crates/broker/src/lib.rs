/*
    Date: 2023
    Author: Fred Kyung-jin Rezeau <fred@litemint.com>
    Copyright (c) 2023 Litemint LLC

    MIT License
*/

// Oracle Broker contract example.

#![no_std]

use soroban_kit::{oracle, oracle_broker};
use soroban_sdk::{
    contract, contractimpl, contracttype, token, Address, Bytes, Env, TryIntoVal, Vec,
};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    Admin,
    Topic(Bytes),
    PublisherWhitelist(Address),
}

#[contract]
#[oracle_broker(Bytes, Bytes)]
pub struct OracleBrokerContract;

// Implement the Oracle events.
impl oracle::Events<Bytes, Bytes> for OracleBrokerContract {
    fn on_subscribe(env: &Env, topic: &Bytes, envelope: &oracle::Envelope) -> Option<Bytes> {
        // Retrieve the envelopes for this topic.
        let mut envelopes = if env.storage().instance().has::<Bytes>(topic) {
            env.storage()
                .instance()
                .get::<Bytes, Vec<oracle::Envelope>>(topic)
                .unwrap()
        } else {
            Vec::new(env)
        };

        // Here, you typically handle specific authentication,
        // apply filters for routers, subscribers, topics, other checks as needed.

        // Example: Enforce max envelopes per topics.
        // assert!(envelopes.len() < 5);

        // In this example, we demonstrate how to charge a fee for all subscriber requests.
        envelope.subscriber.require_auth();
        token::Client::new(
            &env,
            &Address::from_string(
                &"CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC"
                    .try_into_val(env)
                    .unwrap(),
            ),
        )
        .transfer(
            &envelope.subscriber,
            &env.current_contract_address(),
            &10000000,
        );

        // Return the data synchronously if available or store the envelope for later publishing.
        let storage_instance = env.storage().instance();
        let topic_key = DataKey::Topic(topic.clone());
        match storage_instance.has(&topic_key) {
            true => storage_instance.get::<_, Bytes>(&topic_key),
            false => {
                envelopes.push_back(envelope.clone());
                storage_instance.set::<Bytes, Vec<oracle::Envelope>>(topic, &envelopes);
                None
            }
        }
    }

    fn on_publish(
        env: &Env,
        topic: &Bytes,
        data: &Bytes,
        publisher: &Address,
    ) -> Vec<oracle::Envelope> {
        require_publisher_whitelisted(env, publisher);

        // Store the data for synchronous requests.
        let storage_instance = env.storage().instance();
        storage_instance.set::<_, Bytes>(&DataKey::Topic(topic.clone()), data);

        // In this example, we simply return all envelopes @topic
        let envelopes = storage_instance
            .get::<Bytes, Vec<oracle::Envelope>>(topic)
            .unwrap();
        storage_instance.remove::<Bytes>(topic);
        envelopes
    }
}

#[contractimpl]
impl OracleBrokerContract {
    pub fn set_admin(env: Env, admin: Address) {
        assert!(!env.storage().instance().has(&DataKey::Admin));
        env.storage()
            .instance()
            .set::<DataKey, Address>(&DataKey::Admin, &admin);
    }

    pub fn allow_publisher(env: Env, publisher: Address) {
        require_admin_auth(&env);
        env.storage()
            .instance()
            .set::<DataKey, bool>(&DataKey::PublisherWhitelist(publisher), &true);
    }

    pub fn deny_publisher(env: Env, publisher: Address) {
        require_admin_auth(&env);
        env.storage()
            .instance()
            .remove::<DataKey>(&DataKey::PublisherWhitelist(publisher));
    }
}

fn require_publisher_whitelisted(env: &Env, publisher: &Address) -> bool {
    env.storage()
        .instance()
        .get::<DataKey, bool>(&DataKey::PublisherWhitelist(publisher.clone()))
        .unwrap()
}

fn require_admin_auth(env: &Env) {
    env.storage()
        .instance()
        .get::<DataKey, Address>(&DataKey::Admin)
        .unwrap()
        .require_auth();
}

// That's it! Ready to deploy your Oracle broker contract!
