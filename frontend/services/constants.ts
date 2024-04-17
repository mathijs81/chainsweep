import { defineChain } from "viem";

const isLocalDev = process.env.NODE_ENV === 'development';

const localContractAddress = '0x906B067e392e2c5f9E4f101f36C0b8CdA4885EBf';
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

const stylusAddress = '0xE015beC2035588e2bCE7CFbA405dAB285b6dD3C7';
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