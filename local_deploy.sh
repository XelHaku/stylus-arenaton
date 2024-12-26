#!/bin/bash

# Remove old targets to ensure a clean build
echo "Cleaning up old build targets..."
sudo rm -rf stylus_arenaton_engine/target stylus_erc20aton/target

# Terminal #1: Start Nitro Dev Node
echo "Starting Nitro Dev Node in a new terminal..."
gnome-terminal -- bash -c "./nitro-devnode/run-dev-node.sh; exec bash"

# Terminal #2: Process and deploy stylus_erc20aton
echo "Processing and deploying stylus_erc20aton..."
cd stylus_erc20aton/ || { echo "Failed to enter stylus_erc20aton directory"; exit 1; }
cargo clean
if ! cargo stylus check; then
    echo "Error: stylus_erc20aton check failed!"
    exit 1
fi
cd ..

wasm-opt stylus_erc20aton/target/wasm32-unknown-unknown/release/stylus_erc20aton.wasm -o stylus_erc20aton_opt.wasm -O --intrinsic-lowering
if [ $? -ne 0 ]; then
    echo "Error: wasm-opt failed for stylus_erc20aton!"
    exit 1
fi

cargo stylus deploy --private-key 0xb6b15c8cb491557369f3c7d2c287b053eb229daa9c22138887752191c9520659 --wasm-file stylus_erc20aton_opt.wasm
if [ $? -ne 0 ]; then
    echo "Error: Deployment failed for stylus_erc20aton!"
    exit 1
fi

# Process and deploy stylus_arenaton_engine
echo "Processing and deploying stylus_arenaton_engine..."
cd stylus_arenaton_engine/ || { echo "Failed to enter stylus_arenaton_engine directory"; exit 1; }
cargo clean
if ! cargo stylus check; then
    echo "Error: stylus_arenaton_engine check failed!"
    exit 1
fi
cd ..

wasm-opt stylus_arenaton_engine/target/wasm32-unknown-unknown/release/stylus_arenaton_engine.wasm -o stylus_arenaton_engine_opt.wasm -O --intrinsic-lowering
if [ $? -ne 0 ]; then
    echo "Error: wasm-opt failed for stylus_arenaton_engine!"
    exit 1
fi

cargo stylus deploy --private-key 0xb6b15c8cb491557369f3c7d2c287b053eb229daa9c22138887752191c9520659 --wasm-file stylus_arenaton_engine_opt.wasm
if [ $? -ne 0 ]; then
    echo "Error: Deployment failed for stylus_arenaton_engine!"
    exit 1
fi

echo "Deployment completed successfully!"
