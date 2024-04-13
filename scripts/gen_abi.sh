#!/bin/bash
mkdir -p frontend/data

(
  cd chainsweep
  cargo stylus export-abi --json | sed -n '/^\[/p' | tee ../frontend/data/abi.json
)

(
  cd frontend
  pnpm wagmi generate
)