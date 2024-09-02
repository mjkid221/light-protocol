#!/usr/bin/env sh

set -eux

root_dir=$(git rev-parse --show-toplevel)
out_dir="$root_dir/cli/bin"
if [ ! -e "$out_dir" ]; then
    mkdir -p "$out_dir"
fi
# build programs with program ids on mainnet (without test feature)
anchor build -- --no-default-features --features "custom-heap" "idl-build"
keys="account_compression light_system_program light_compressed_token light_registry"
for key in $keys
do
    cp "$root_dir/target/deploy/$key.so" "$out_dir"/"$key".so
done
cp "$root_dir"/third-party/solana-program-library/spl_noop.so "$out_dir"/spl_noop.so
# rebuild programs with test feature enabled
anchor build