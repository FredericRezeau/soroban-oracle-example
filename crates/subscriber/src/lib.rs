/*
    Date: 2023
    Author: Fred Kyung-jin Rezeau <fred@litemint.com>
    Copyright (c) 2023 Litemint LLC
a
    MIT License
*/

// Oracle Subscriber contract example.

#![no_std]

use soroban_kit::{oracle, oracle_subscriber};
use soroban_sdk::{contract, contractimpl, contracttype, Address, Bytes, Env};

#[contract]
#[oracle_subscriber(Bytes, Bytes)]
pub struct OracleSubscriberContract;

#[contracttype]
pub enum DataKey {
    Admin,
    Topic(Bytes),
    BrokerWhitelist(Address),
}

// Implement the Oracle events.
impl oracle::Events<Bytes, Bytes> for OracleSubscriberContract {
    fn on_request(env: &Env, _topic: &Bytes, envelope: &oracle::Envelope) {
        require_broker_whitelisted(env, &envelope.broker);
        envelope.subscriber.require_auth();
    }

    fn on_sync_receive(env: &Env, topic: &Bytes, envelope: &oracle::Envelope, data: &Bytes) {
        require_broker_whitelisted(env, &envelope.broker);

        // Save the data received synchronously.
        env.storage().instance().set(
            &DataKey::Topic(topic.clone()),
            reconcile_data(&mut data.clone()),
        );
    }

    fn on_async_receive(env: &Env, topic: &Bytes, envelope: &oracle::Envelope, data: &Bytes) {
        require_broker_whitelisted(env, &envelope.broker);
        envelope.broker.require_auth(); // Make sure this cross-contract call is from broker.

        // Save the data received asynchronously.
        env.storage().instance().set(
            &DataKey::Topic(topic.clone()),
            reconcile_data(&mut data.clone()),
        );
    }
}

#[contractimpl]
impl OracleSubscriberContract {
    pub fn set_admin(env: Env, admin: Address) {
        assert!(!env.storage().instance().has(&DataKey::Admin));
        env.storage()
            .instance()
            .set::<DataKey, Address>(&DataKey::Admin, &admin);
    }

    pub fn allow_broker(env: Env, broker: Address) {
        require_admin_auth(&env);
        env.storage()
            .instance()
            .set::<DataKey, bool>(&DataKey::BrokerWhitelist(broker), &true);
    }

    pub fn deny_broker(env: Env, broker: Address) {
        require_admin_auth(&env);
        env.storage()
            .instance()
            .remove::<DataKey>(&DataKey::BrokerWhitelist(broker));
    }

    pub fn get_data(env: Env, topic: Bytes) -> Option<Bytes> {
        env.storage()
            .instance()
            .get::<DataKey, Bytes>(&DataKey::Topic(topic))
    }
}

fn reconcile_data<'a>(data: &'a mut Bytes) -> &'a mut Bytes {
    // Here you would typically reconcile your data
    // e.g, prices aggregation, validating, coalescing results from several publishers.
    // In this example, we update from latest version.
    data
}

fn require_admin_auth(env: &Env) {
    env.storage()
        .instance()
        .get::<DataKey, Address>(&DataKey::Admin)
        .unwrap()
        .require_auth();
}

fn require_broker_whitelisted(env: &Env, broker: &Address) -> bool {
    env.storage()
        .instance()
        .get::<DataKey, bool>(&DataKey::BrokerWhitelist(broker.clone()))
        .unwrap()
}

// That's it! Ready to deploy your Oracle subscriber contract!
