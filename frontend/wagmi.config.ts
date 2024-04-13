import { defineConfig } from '@wagmi/cli';
import sweeperAbi from '@/data/abi.json';
import type { Abi } from 'viem';

export default defineConfig({
    out: 'src/generated.ts',
    contracts: [
        {
            name: 'chainsweep',
            abi: sweeperAbi as Abi,
        },
    ], plugins: []
});
