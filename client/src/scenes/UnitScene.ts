import { Container, Graphics } from 'pixi.js';
import { UnitT } from 'models/unit';
import { IScene } from '../types';

export class UnitScene extends Container implements IScene {
  public name: string = 'UnitScene';
  public id: number;
  public assetBundles: string[] = [];
  public g!: Graphics;

  constructor(player: UnitT) {
    super();
    this.id = Number(player.id);
    this.name = `UnitScene ${player.id}`;
    this.g = new Graphics();
    this.g.beginFill(0x000000);
    this.g.drawCircle(0, 0, 25);
    this.g.endFill();
    this.addChild(this.g);

    // draw heading triangle inside circle
    this.g.lineStyle(1, 0xffffff);
    this.g.beginFill(0x000000);
    this.g.moveTo(0, -25);
    this.g.lineTo(10, 0);
    this.g.lineTo(-10, 0);
    this.g.lineTo(0, -25);

    this.x = player.position?.x || 0;
    this.y = player.position?.y || 0;
    this.angle = 0;
  }

  public constructorWithAwaits(): void {}

  public message(/* message: MessageEvent */): void {
    // To be a scene we must have the message method even if we don't use it.
  }

  public update(/* framesPassed: number */): void {
    // To be a scene we must have the update method even if we don't use it.
  }
}
