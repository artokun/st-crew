import { Container, Graphics } from 'pixi.js';
import { IScene } from '../types';

export class TileScene extends Container implements IScene {
  public name: string = 'TileScene';
  public id: number;
  public assetBundles: string[] = [];
  public g!: Graphics;

  constructor(id: number, x: number, y: number) {
    super();
    this.id = id;
    this.name = `TileScene ${id}`;
    this.g = new Graphics();
    this.g.lineStyle(1, 0x000000, 0.5);
    this.g.beginFill(0x00ff00);
    this.g.drawRect(0, 0, 50, 50);
    this.g.endFill();

    this.addChild(this.g);

    this.x = x;
    this.y = y;
  }

  public constructorWithAwaits(): void {}

  public message(/* message: MessageEvent */): void {
    // To be a scene we must have the message method even if we don't use it.
  }

  public update(/* framesPassed: number */): void {
    // To be a scene we must have the update method even if we don't use it.
  }
}
