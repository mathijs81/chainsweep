import { defineChain } from "viem";

const isLocalDev = process.env.NODE_ENV === 'development';

const localContractAddress = '0xF85895D097B2C25946BB95C4d11E2F3c035F8f0C';
const localTargetChain = defineChain({
    id: 412346,
    name: "Stylus Local",
    network: "Arbitrum Stylus Local",
    nativeCurrency: {
        decimals: 18,
        name: "Ether",
        symbol: "ETH",
    },
    rpcUrls: {
        default: {
            http: ["http://localhost:8547"],
        },
        public: {
            http: ["http://localhost:8547"],
        },
    },
    blockExplorers: {
        default: {
            name: "Explorer",
            url: "http://localhost:4000/",
        },
    },
});

const stylusAddress = '0xF19b33779c050294c539b7617ECf6CD1744c4236';
const stylusTestnet = defineChain({
    id: 23011913,
    name: 'Stylus Testnet',
    network: 'stylusTestnet',
    nativeCurrency: {
      decimals: 18,
      name: 'Ether',
      symbol: 'ETH'
    },
    rpcUrls: {
      default: {
        http: [ 'https://stylus-testnet.arbitrum.io/rpc' ]
      },
      public: {
        http: [ 'https://stylus-testnet.arbitrum.io/rpc' ]
      }
    },
    blockExplorers: {
        default: { name: 'Blockscout', url: 'https://stylus-testnet-explorer.arbitrum.io/' },
    },
});

export const contractAddress = isLocalDev ? localContractAddress : stylusAddress;
export const targetChain = isLocalDev ? localTargetChain : stylusTestnet;