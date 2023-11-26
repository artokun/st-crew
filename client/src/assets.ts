import type { AssetsManifest } from 'pixi.js';

export const manifest: AssetsManifest = {
  bundles: [
    {
      name: 'game',
      assets: {
        bunny: '/images/bunny.png',
        // bg: "/images/basic.png",
      },
    },
    {
      name: 'sounds',
      assets: {},
    },
  ],
};
