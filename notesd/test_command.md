# Deploying and Testing Stylus Contracts

This document outlines the steps to deploy and test Stylus contracts using a Nitro development node.

*1. Start the Nitro Development Node**

Execute the `run-dev-node.sh` script to start a local Nitro development node. This provides a simulated blockchain environment for testing your contracts.

**2. Deploy the `nitro-devnode` Contract**

* Navigate to the `nitro-devnode` directory.
* Run the following command to deploy the contract:

``` bash
cargo stylus deploy --private-key 0xb6b15c8cb491557369f3c7d2c287b053eb229daa9c22138887752191c9520659
```

Replace the private key with your actual private key if needed.

**3. Deploy the `arenaton_engine` Contract**

* Navigate to the `arenaton_engine` directory.
* Run the following command to deploy the contract:

``` bash
cargo stylus deploy --private-key 0xb6b15c8cb491557369f3c7d2c287b053eb229daa9c22138887752191c9520659
```

Replace the private key with your actual private key if needed.

**4. Run the `test_contracts` Project**

* Navigate to the `test_contracts` directory.
* Execute the following command to build and run the project:

``` bash
cargo run
```

This will execute the code in the `test_contracts` project, which likely interacts with the deployed `nitro-devnode` and `arenaton_engine` contracts.

**Note:** This assumes you have the necessary Stylus tooling and dependencies installed and configured correctly.
