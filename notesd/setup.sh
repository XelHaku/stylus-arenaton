eBlack:~$ cat setup_stylus.txt
rustup self  uninstall 
npm install -g solc@0.8.24
solc --version
solc-select use 0.8.24
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustc --version
cargo --version
rustup install nightly-2024-09-05
rustup default nightly-2024-09-05
rustup uninstall stable-x86_64-unknown-linux-gnu
rustup show
rustup component add rust-src --toolchain nightly-2024-09-05
rustup target add wasm32-unknown-unknown --toolchain nightly-2024-09-05
rustup component add rust-src --toolchain nightly-2024-09-05
rustup target add wasm32-unknown-unknown --toolchain nightly-2024-09-05
sudo apt-get update && sudo apt-get install -y \
    build-essential \
    pkg-config \
    libssl-dev \
    libffi-dev \
    cmake \
    clang \
    llvm \
    libclang-dev \
    zlib1g-dev \
    libzstd-dev \
    liblz4-tool \
    libc6-dev \
    gcc \
    g++ \
    make \
    autoconf \
    automake \
    libtool \
    libgmp-dev \
    libmpfr-dev \
    libmpc-dev \
    libunwind-dev \
    curl \
    wget \
    zip \
    unzip \
    git \
    gnupg \
    ca-certificates \
    apt-transport-https \
    software-properties-common \
    lsb-release \
    jq \
    neofetch \
    python3 \
    python3-pip \
    python3-venv \
    python3-dev \
    xz-utils \
    ninja-build \
    nasm \
    yasm \
    libudev-dev \
    libdbus-1-dev \
    libxcb1-dev \
    libxcb-render0-dev \
    libxcb-shape0-dev \
    libxcb-xfixes0-dev \
    libdrm-dev \
    libxrandr-dev \
    libxi-dev \
    libx11-xcb-dev \
    libxext-dev \
    libwayland-dev \
    libxinerama-dev \
    libxcursor-dev \
    libxkbcommon-dev \
    libgl1-mesa-dev \
    libegl1-mesa-dev \
    libgbm-dev \
    libvulkan-dev \
    libgles2-mesa-dev
cargo install --force cargo-stylus
cargo install --force cargo-stylus cargo-stylus-check


nightly-x86_64-unknown-linux-gnu
rustup install nightly-x86_64-unknown-linux-gnu
rustup target add wasm32-unknown-unknown --toolchain nightly






cargo install --force cargo-stylus cargo-stylus-check
cargo build --release --target wasm32-unknown-unknown -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort
To properly configure the specified `rust-toolchain.toml` file and align your environment with the requirements listed, follow thes
xel@BlueBlack:~$ 

