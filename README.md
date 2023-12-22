
# Code example for soroban-kit::oracle


This repository contains the code associated with [this article]() published on dev.to. To use it, clone the repository and follow the guidelines provided in the article.

## Smart contracts

- **Oracle Broker Contract** (see [lib.rs](https://github.com/FredericRezeau/soroban-oracle-example/blob/master/crates/broker/src/lib.rs))
  - Handling sync/async topic-based subscriber requests.
  - Handling publishing requests.
  - Collecting subscriber fees.
  - Managing envelopes for async routing.
  - Managing publishers whitelist.

- **Oracle Subscriber Contract** (see [lib.rs](https://github.com/FredericRezeau/soroban-oracle-example/blob/master/crates/subscriber/src/lib.rs))
  - Routing topic-based data requests to broker contract.
  - Handling sync/async responses.
  - Managing brokers whitelist.
  - Providing data reconciliation.

  _Note that our example data reconciliation function simply replaces the old data with the new but more elaborate mechanisms can be implemented, including price aggregation, average computation, validation and coalescing of results, based on specific use cases._