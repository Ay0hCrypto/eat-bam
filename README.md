# Jito's BAM fork of the Solana validator with mod's from Allnodes edited/tweaked by EAT TRIBE

<p align="center">
    <br /><br />
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="allnodes/images/bam-dark-mode.png">
EAT Tribe Validator Tweak      <img alt="Jito Allnodes Edition" src="allnodes/images/bam-light-mode.png" style="width: 16em">
    </picture>
</p>

on [Allnodes Bare-Metal Servers](https://www.allnodes.com/hosting)

## Original Mod's made by Allnodes

This repository features the following enhancements to the Jito-Solana (BAM) codebase:

### 1. Fast snapshot distribution

Includes modifications that improve default snapshot downloading, which combined with
ultra-high-speed channels deliver ultra-fast snapshot downloads. This dramatically reduces the initial sync time for
new validators and enables faster deployment and recovery scenarios. The use of snapshot-finder or any other 3rd party
download tools is no longer needed.

### 2. Enhanced voting logic modifications

Validator implementation includes voting modifications developed by **Zantetsu | Shinobi Systems** that enhance the
original voting logic.

These modifications work by:
- Taking the next votable slot that the original codebase identifies as potentially ready for voting
- Applying additional criteria before casting the vote
- Providing more sophisticated voting decision-making
This enhancement improves validator consensus participation through more intelligent vote timing and slot evaluation.
### 3. Automatic Performance Optimization for Proof-of-History


Your Solana node will automatically select the fastest CPU core for Proof-of-History processing, maximizing performance
out of the box.

### 4. Hardware-optimized SHA256 patch

Our validator implementation includes a third-party performance patch developed by **kagren**. It optimizes SHA256
hashing operations using SHA-NI instructions available on modern AMD processors (Zen3, Zen4, and Zen5
architectures). This enhancement significantly improves hashing performance for block verification and other
cryptographic operations.

## Building and running

> [!NOTE]
> We recommend checking out Jito's [Gitbook](https://jito-foundation.gitbook.io/mev/jito-solana/building-the-software)
> for more detailed instructions on building and running Jito-Solana.

### 1. Install rustc, cargo and rustfmt

```bash
$ curl https://sh.rustup.rs -sSf | sh
$ source $HOME/.cargo/env
$ rustup component add rustfmt
```

The `rust-toolchain.toml` file pins a specific rust version and ensures that
cargo commands run with that version. Note that cargo will automatically install
the correct version if it is not already installed.

On Linux systems you may need to install libssl-dev, pkg-config, zlib1g-dev, protobuf etc.

On Ubuntu:

```bash
$ sudo apt-get update
$ sudo apt-get install libssl-dev libudev-dev pkg-config zlib1g-dev llvm clang cmake make libprotobuf-dev protobuf-compiler libclang-dev curl git
```

On Fedora:

```bash
$ sudo dnf install openssl-devel systemd-devel pkg-config zlib-devel llvm clang cmake make protobuf-devel protobuf-compiler perl-core libclang-dev curl git
```

### 2. Download the source code

To download the source code, run (substitute `<version>` with the version tag you want to build):

```bash
$ git clone --recursive https://github.com/allnodes/solana-bam --branch <version>
$ cd solana-bam
```

### 3. Release build

```bash
$ ./cargo build --release
```

### 4. Voting mod configuration

Voting mod (also known as "mostly confirmed threshold" voting patch) is enabled by default and comes with a predefined
configuration which should work for most users. If you wish to use a custom configuration:

1. create a configuration file (default filename is `mostly_confirmed_threshold` located in the current directory from
   where you run the validator). Values in this example are defaults, their meanings will be explained in the next
   section:

```bash
echo '0.45 4 0 24' > ./mostly_confirmed_threshold
```

2. optionally, you can provide a different filename and/or path for the config file using the
   `--mostly-confirmed-threshold-config <path/to/config/file>` argument.

> In order to disable the voting mod, you need to add the `--disable-mostly-confirmed-threshold` flag to the validator
command.

## Mostly confirmed threshold configuration file format:

The `mostly_confirmed_threshold` file contains a simple whitespace-separated list of four values:

```
a b c d
```

### Parameters

#### *a* (float) - vote weight threshold
The minimum vote weight threshold required before voting on a slot. Slots that haven't achieved this vote weight will
not be voted on, except for:

- Slots within the "vote ahead of threshold" region
- When the escape hatch distance has been reached

#### *b* (integer) - vote ahead of threshold
The number of slots ahead of the threshold slot to vote on, regardless of vote weight. This parameter reduces vote
latency by allowing voting on recent slots even if they haven't met the threshold.

#### *c* (integer) - skip recovery mode
Controls the stake-weighted vote percentage required on a slot after skips have occurred. Must be one of:

- `0` - No restriction
- `1` - Slot after a skip must have `mostly_confirmed_threshold` before voting
- `2` - Slot after a skip must be confirmed before voting

#### *d* (integer) - escape hatch distance
The maximum number of slots to wait without voting while waiting for the threshold to be met. After this many slots of non-voting, the validator will vote anyway.

**Purpose**: This escape hatch prevents network deadlock by ensuring progress even when the threshold isn't being achieved. Without this mechanism, if multiple forks occur simultaneously and all have less than the threshold vote weight, validators could become stuck waiting indefinitely.

### Default values

When the configuration file is absent, the following default values are used:

```
0.45 4 0 24
```

- Threshold: 45% vote weight
- Vote ahead: 4 slots
- Skip recovery: No restriction
- Escape hatch: 24 slots

-  Need RPC or Validator BMS? [Allnodes Bare-Metal Servers](https://www.allnodes.com/hosting)

##DO NOT USE BELOW IF LINE REMAINS
___________________________________________________________________________________________________________________________________________________________________________
[![Build status](https://badge.buildkite.com/3a7c88c0f777e1a0fddacc190823565271ae4c251ef78d83a8.svg)](https://buildkite.com/jito/jito-solana)

# About
# Solana 2.3.11-BAM — Full Copy-Ready Validator Setup (Mainnet & Testnet)

**What this contains (copy/paste ready):**

* system preparation & sysctl tuning (network buffers / file limits).
* building `jito-labs/bam-client` v`v2.3.11-bam` (with the `perf-libs` build fix).
* correct BAM usage: **`--bam-url <BAM_NODE_URL>` is required** for BAM.
* correct Jito/BAM program IDs (mainnet & testnet) for `--tip-payment-program-pubkey` and `--tip-distribution-program-pubkey`.
* included `--relayer-url`, `--block-engine-url` (where applicable) and **`--merkle-root-upload-authority`** flag.
* copy-ready `http://validator-startup.sh` (single script, network selected by env var) and the `systemd` unit file.

---

## Quick pointers before you paste

* Replace `YOURUSERNAME` with the system user running the validator.
* Replace `BAM_URL`, `RELAYER_URL`, `BLOCK_ENGINE_URL` if you have a specific region preference. Use the BAM URL that matches your region/latency.
* `MERKLE_ROOT_UPLOAD_AUTH` can be either the **Jito default** `GZctHpWXmsZC1YHACTGGcHhYxjdRqQvTpYkb9LMvxDib` (let Jito run merkle airdrops) **or** your validator key if you will upload merkle roots yourself.

---

## 0) System prep (Ubuntu 24.04)

```bash
sudo apt update && sudo apt upgrade -y # (PersonallyI dont Run This)
sudo apt install -y curl git build-essential pkg-config libssl-dev clang \
  nvme-cli jq net-tools unzip lz4 ufw

# Disable swap, enable THP off (recommended for Solana/Jito)
sudo swapoff -a
echo never | sudo tee /sys/kernel/mm/transparent_hugepage/enabled
```

### Kernel / network tuning (paste as root)

```bash
sudo tee /etc/sysctl.d/99-solana-validator.conf > /dev/null <<'EOF'
# Increase UDP/TCP buffer sizes
net.core.rmem_default = 134217728
net.core.rmem_max     = 134217728
net.core.wmem_default = 134217728
net.core.wmem_max     = 134217728

# Increase backlog and NIC queue
net.core.netdev_max_backlog = 250000

# File handles
fs.file-max = 1000000

# IPv4 tcp window scaling
net.ipv4.tcp_moderate_rcvbuf = 1
EOF
sudo sysctl --system
```

(These buffer values are the common, proven values used for high-throughput Solana validators.)

---

## 1) Disk layout (examples — **keep ledger/accounts/snapshots OFF the OS disk**)

Create mount points:

```bash
sudo mkdir -p /mnt/{ledger,accounts,snapshots}
sudo chown $USER:$USER /mnt/{ledger,accounts,snapshots}
```

* **Testnet (recommended)**: place ledger/accounts/snapshots on a single spare NVMe (e.g. `/mnt/ledger`) — *off the OS disk*.
* **Mainnet**: use **two spare NVMe drives**: one for `/mnt/ledger` or `/mnt/snapshots/ledger` and the other for `/mnt/accounts` + `/mnt/snapshots`. (Example below uses `/mnt/snapshots/ledger`, `/mnt/accounts`, `/mnt/snapshots`).

---

## 2) Open required ports & check availability

```bash
# allow Solana/Known ports (dynamic range + gossip + RPC)
sudo ufw allow 8000:8020/tcp comment 'Solana dynamic range'
sudo ufw allow 8001/udp comment 'Solana gossip'
sudo ufw allow 8899/tcp comment 'Solana RPC'
sudo ufw --force enable
sudo ufw status

# make sure the ports are free before starting
ss -ltnp | egrep ':(8000|8001|8899)' || true
sudo lsof -i :8899 || true
```

If another process uses these ports, stop/disable it or change Solana's `--dynamic-port-range` / `--gossip-port` accordingly.

---

## 3) Clone & build BAM client v2.3.11-bam
 --  BUILD FAILS 99.9% of times - if not in /bam-client: at that point goto next Step this is expected 

```bash
# clone and checkout the release tag
git clone https://github.com/jito-labs/bam-client.git ~/bam-client
cd ~/bam-client
git fetch --tags
git checkout v2.3.11-bam

# Perf-libs build issue workaround (safe)
rm -rf target/release/perf-libs
mkdir -p target/release/perf-libs

# build
cargo build --release

# add binary to PATH
echo 'export PATH=$HOME/bam-client/target/release:$PATH' >> ~/.bashrc
source ~/.bashrc

#
**
cd ~/bam-client

# 1. Reset permissions to your user (in case you used sudo earlier)
sudo chown -R $USER:$USER .

# 2. Make sure the anchor submodules are initialized correctly
git submodule update --init --recursive

# 3. Double-check file permissions
find anchor -type f -name "Cargo.toml" -exec chmod 644 {} \;

# 4. Clean and rebuild
cargo clean
cargo build --release

# verify
solana-validator --version
```

If `cargo build` errors referencing `perf-libs` or missing perf-libs, the above workaround (create an empty folder) is known to resolve the most common cargo/perf-libs build failure patterns.  - this never happened to me but when researching the above fix, I saw others with per-libs errors so added that fix as well.

---

## 4) Create keys 
```bash
solana-keygen new --outfile ~/validator-keypair.json
solana-keygen new --outfile ~/vote-account-keypair.json
solana-keygen new --outfile ~/authorized-withdrawer-keypair.json  #Not recommended to keep these keys on server use a USB drive or ledger or paper wallet
```

(Keep these files safe — move to an HSM or encrypted storage for mainnet.)

---

## 5) Configure cluster endpoint (CLI)

```bash
# Testnet:
solana config set --url https://api.testnet.solana.com

# Mainnet:
# solana config set --url https://api.mainnet-beta.solana.com
```
DOUBLR CHECK CONFIG and PATHS
solana config get
---

## 6) Create vote account

```bash
solana create-vote-account ~/vote-account-keypair.json \
  ~/validator-keypair.json \
  ~/authorized-withdrawer-keypair.json
```

---

## 7) Official program IDs (use these exact IDs for the `--tip-*` flags)

**Mainnet (on-chain addresses)**

* **Tip Payment Program:** `T1pyyaTNZsKv2WcRAB8oVnk93mLJw2XzjtVYqCsaHqt`
* **Tip Distribution Program:** `4R3gSG8BpU4t19KYj8CfnbtRpnT8gtk4dvTHxVRwc2r7`
  **Testnet (on-chain addresses)**
* **Tip Payment Program (testnet):** `GJHtFqM9agxPmkeKjHny6qiRKrXZALvvFGiKf11QE7hy`
* **Tip Distribution Program (testnet):** `F2Zu7QZiTYUhPd7u9ukRVwxh7B71oA3NMJcHuCHc29P2`

(These program IDs come from the Jito on-chain addresses documentation — use the testnet pair when on testnet and the mainnet pair on mainnet.)

---

## 8) BAM / Relayer / Block-engine URLs & flags

* BAM requires `--bam-url <BAM_NODE_URL>` when running the BAM validator client.
* Jito/Jito-Solana also accepts `--relayer-url` and `--block-engine-url` for direct connections; include them if you want redundancy or to point at a local/region block-engine as well.

**Example BAM URL choices (pick one near you / your DC):**

* `https://frankfurt.mainnet.bam.jito.wtf` (mainnet)
* `https://ny.mainnet.bam.jito.wtf` (mainnet)
* `https://amsterdam.mainnet.bam.jito.wtf` (mainnet)
* `https://ny.testnet.bam.jito.wtf` (testnet)
* `https://dallas.testnet.bam.jito.wtf` (testnet)

(If you have a preferred `block-engine` or `relayer` URL, set `RELAYER_URL`/`BLOCK_ENGINE_URL` below.)

---

## 9) Copyable `http://validator-startup.sh` (single script — mainnet/testnet switch via NETWORK env var)

Save as `~/validator-startup.sh` and **DO NOT** commit your key files to VCS.

# ------------------------
# You will need to refrence these values and replace them in your chosen .sh - double check all paths to make sure they align with your build.
# ------------------------
: "${NETWORK:=mainnet}"   # set to "testnet" for testnet
: "${USER_HOME:=$HOME}"
: "${SOLANA_VALIDATOR_BIN:=$HOME/bam-client/target/release/solana-validator}"
: "${BAM_URL:=https://frankfurt.mainnet.bam.jito.wtf}"
: "${RELAYER_URL:=https://frankfurt.mainnet.block-engine.jito.wtf}"
: "${BLOCK_ENGINE_URL:=https://block-engine.mainnet.frankfurt.jito.wtf}"
: "${MERKLE_ROOT_UPLOAD_AUTH:=GZctHpWXmsZC1YHACTGGcHhYxjdRqQvTpYkb9LMvxDib}" # Jito default (or set your validator pubkey)
: "${COMMISSION_BPS:=500}" # example: 500 = 5.00%

# Keys (must exist and be readable by this user)
IDENTITY_KEY="${USER_HOME}/validator-keypair.json"
VOTE_KEY="${USER_HOME}/vote-account-keypair.json"
WITHDRAWER_KEY="${USER_HOME}/authorized-withdrawer-keypair.json"

LEDGER_DIR="/mnt/ledger"
  ACCOUNTS_DIR="/mnt/ledger/accounts"
  SNAPSHOTS_DIR="/mnt/ledger/snapshots"
  BAM_URL="${BAM_URL:-https://ny.testnet.bam.jito.wtf}"
  TIP_PAYMENT_PROG="GJHtFqM9agxPmkeKjHny6qiRKrXZALvvFGiKf11QE7hy"
  TIP_DISTRIBUTION_PROG="F2Zu7QZiTYUhPd7u9ukRVwxh7B71oA3NMJcHuCHc29P2"
else
  LEDGER_DIR="/mnt/snapshots/ledger"
  ACCOUNTS_DIR="/mnt/accounts"
  SNAPSHOTS_DIR="/mnt/snapshots"
  BAM_URL="${BAM_URL:-https://frankfurt.mainnet.bam.jito.wtf}"
  TIP_PAYMENT_PROG="T1pyyaTNZsKv2WcRAB8oVnk93mLJw2XzjtVYqCsaHqt"
  TIP_DISTRIBUTION_PROG="4R3gSG8BpU4t19KYj8CfnbtRpnT8gtk4dvTHxVRwc2r7"

------ sudo nano /home/$USER/validator-startup.sh

#!/usr/bin/env bash

set -euo pipefail

exec ${SOLANA_VALIDATOR_BIN} \
  --identity "${IDENTITY_KEY}" \
  --vote-account "${VOTE_KEY}" \
  --ledger "${LEDGER_DIR}" \
  --accounts "${ACCOUNTS_DIR}" \
  --snapshots "${SNAPSHOTS_DIR}" \
  --log "${SNAPSHOTS_DIR}/validator.log" \
  --limit-ledger-size 50000000 \
  --no-wait-for-vote-to-start-leader \
  --accounts-db-cache-limit-mb 50000 \
  --wal-recovery-mode skip_any_corrupted_record \
  --only-known-rpc \
  --enable-rpc-transaction-history \
  --rpc-bind-address 0.0.0.0 \
  --gossip-port 8001 \
  --dynamic-port-range 8000-8020 \
  --bam-url "${BAM_URL}" \
  --block-engine-url "${BLOCK_ENGINE_URL}" \
  --tip-payment-program-pubkey "${TIP_PAYMENT_PROG}" \
  --tip-distribution-program-pubkey "${TIP_DISTRIBUTION_PROG}" \
  --merkle-root-upload-authority "${MERKLE_ROOT_UPLOAD_AUTH}" \
  --commission-bps "${COMMISSION_BPS}" \

Make executable:

```bash
chmod +x ~/validator-startup.sh
```

---

## 10) systemd service (persistent)
---   sudo nano /etc/systemd/system/validator.service

Save as `/etc/systemd/system/validator.service` (replace `YOURUSERNAME` with the sys user):

```ini
[Unit]
Description=Solana Validator (BAM/Jito) - persistent service
After=http://network.target

[Service]
User=YOURUSERNAME
LimitNOFILE=1000000
LimitNPROC=500000
Restart=always
RestartSec=5
Environment="RUST_BACKTRACE=1"
ExecStart=/home/YOURUSERNAME/validator-startup.sh
ExecReload=/bin/kill -HUP $MAINPID
KillSignal=SIGINT
TimeoutStopSec=300
Nice=-5

[Install]
WantedBy=http://multi-user.target
```

Enable & start:

```bash
---   sudo systemctl daemon-reload
---   sudo systemctl enable validator
---   sudo systemctl start validator
---   tail -n 200 /mnt/snapshots/validator.log
```

---

## 11) Post-start checks & useful commands

```bash
# Check logs
tail -n 200 /mnt/snapshots/validator.log

# Check if validator identity is in gossip - this will not be applicable for new builds.
solana gossip | grep $(solana-keygen pubkey ~/validator-keypair.json) || true

# Leader schedule check
solana leader-schedule | grep $(solana-keygen pubkey ~/vote-account-keypair.json) || true

# Confirm program ids (sanity)
echo "Tip payment program (mainnet): T1pyya..."  # compare with the docs
```

---

## 12) Notes & citations
* Buffer and Port updates only needed on new builds. AS-IS - USE-AT-YOUR-OWN-RISK = IT WORKED FOR ME, ALL RESULTS MAY NOT BE EQUAL.
* BAM requires `--bam-url` to be set for the BAM scheduler. Use the BAM URL closest to your data-center/region for lowest latency.
* Jito validator CLI exposes `--relayer-url`, `--block-engine-url`, `--tip-*` flags and `--merkle-root-upload-authority` — use them per your policy. 
* Official on-chain program IDs (tip payment/distribution) for testnet & mainnet are taken from the Jito on-chain addresses doc — **use the testnet pair on testnet and the mainnet pair on mainnet**.
* If you hit the `perf-libs` cargo / build error, the `rm -rf target/release/perf-libs && mkdir -p ...` step above is a commonly used fix.
* The network/sysctl buffer settings above are the standard high-values used for high-throughput validators. Tune them only if you understand the implications.

###BONUS### TESTNET and MAINNET SELF-STAKE

---

### 🪙 **Self-Stake Setup**

#### **Mainnet**

1. Send your SOL (for example, 100 SOL) to the validator’s **identity address**. ##MAINNET Edit for your Authorized-withdrawer is recommended, for security.

   ```bash
   solana address -k ~/validator-keypair.json
   ```

2. Check your balance:

   ```bash
   solana balance
   ```

3. Create a stake account and delegate it:

   ```bash
   solana-keygen new --outfile ~/self-stake.json
   solana create-stake-account ~/self-stake.json 100
   solana delegate-stake ~/self-stake.json ~/vote-account-keypair.json
   ```

4. Verify the stake:

   ```bash
   solana stake-account ~/self-stake.json
   ```

---

#### **Testnet**

1. Go to the **Solana Testnet Faucet**:
   👉 [https://faucet.solana.com](https://faucet.solana.com)
   Request **5 SOL twice daily** to your validator’s identity address.

   ```bash
   solana address -k ~/validator-keypair.json
   ```

2. Check the balance:

   ```bash
   solana balance
   ```

3. Create and delegate a small test stake:

   ```bash
   solana-keygen new --outfile ~/testnet-stake.json
   solana create-stake-account ~/testnet-stake.json 2
   solana delegate-stake ~/testnet-stake.json ~/vote-account-keypair.json
   ```

4. Confirm:

   ```bash
   solana stake-account ~/testnet-stake.json
   ```

---

✅ **That’s it —** both networks will now show your validator as self-staked and eligible for leader rotation once active.

-- @ay0hcrypto | 
@ay0h_sol
 (X)

IF THIS IS OUTDATED:::

If your **BAM (Jito-Solana)** build is outdated and you want to safely update to the **latest release**, here’s the correct and clean sequence of commands — it keeps your build directory intact and avoids redoing key setup or configs:

---

### 🧩 **Update to the Latest BAM Release**

```bash
cd ~/bam-client

# 1. Make sure you're on the main branch
git checkout main

# 2. Pull the latest code and tags
git fetch --all --tags

# 3. View available BAM versions
git tag -l | grep bam
```

Find the newest version (for example, `v9.9.99-bam`).

---

### **Then update and rebuild**

```bash
git checkout v9.9.99-bam
git submodule update --init --recursive

### (Optional cleanup to prevent old artifacts causing build issues)
cargo clean
rm -rf target/release/perf-libs
mkdir -p target/release/perf-libs

# 4. Rebuild
cargo build --release
```

---

### **Verify update**

```bash
solana --version
```

✅ Should now show something like:
`solana-cli 9.9.99-bam (jito-labs)`

---

### **Restart your validator**

sudo systemctl restart validator


# Release Process

The release process for this project is described [here](RELEASE.md).

# Code coverage

To generate code coverage statistics:

```bash
$ scripts/coverage.sh
$ open target/cov/lcov-local/index.html
```
# Even if you're using this fork, it's Jito-Lab's work - the least you can do is enable code coverage xXx
Why coverage? While most see coverage as a code quality metric, we see it primarily as a developer
productivity metric. When a developer makes a change to the codebase, presumably it's a *solution* to
some problem. Our unit-test suite is how we encode the set of *problems* the codebase solves. Running
the test suite should indicate that your change didn't *infringe* on anyone else's solutions. Adding a
test *protects* your solution from future changes. Say you don't understand why a line of code exists,
try deleting it and running the unit-tests. The nearest test failure should tell you what problem
was solved by that code. If no test fails, go ahead and submit a Pull Request that asks, "what
problem is solved by this code?" On the other hand, if a test does fail and you can think of a
better way to solve the same problem, a Pull Request with your solution would most certainly be
welcome! Likewise, if rewriting a test can better communicate what code it's protecting, please
send us that patch!
Mod's by [Allnodes Bare-Metal Servers](https://www.allnodes.com/hosting)  IF #do not use# line is above this install walkthrough - do not use it - several keys are just random strings - I'll get to editing and optimizing this when I feel like it- unless stated assume this broken and wont work, because I broke it. If you want a working version goto the upstream repo by allnodes
