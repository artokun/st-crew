import { DisplayObject } from 'pixi.js';

export interface IScene extends DisplayObject {
  name: string;
  update(framesPassed: number): void;
  message<T>(message: MessageEvent<T>): void;
  constructorWithAwaits(): void;
  assetBundles: string[];
}
