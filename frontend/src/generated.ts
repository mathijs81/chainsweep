//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// chainsweep
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export const chainsweepAbi = [
  { type: 'error', inputs: [], name: 'FieldAlreadyOpened' },
  { type: 'error', inputs: [], name: 'GameAlreadyOver' },
  { type: 'error', inputs: [], name: 'GameAlreadyStarted' },
  {
    type: 'function',
    inputs: [
      { name: 'x', internalType: 'uint8', type: 'uint8' },
      { name: 'y', internalType: 'uint8', type: 'uint8' },
    ],
    name: 'makeGuess',
    outputs: [{ name: '', internalType: 'uint8', type: 'uint8' }],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [],
    name: 'newGame',
    outputs: [{ name: '', internalType: 'string', type: 'string' }],
    stateMutability: 'nonpayable',
  },
  {
    type: 'function',
    inputs: [{ name: '_address', internalType: 'address', type: 'address' }],
    name: 'viewFor',
    outputs: [{ name: '', internalType: 'string', type: 'string' }],
    stateMutability: 'view',
  },
] as const;
