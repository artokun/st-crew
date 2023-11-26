import { Container, Graphics } from 'pixi.js';
import { Manager } from '../Manager';
import { IScene } from '../types';

export class LoaderScene extends Container implements IScene {
  public name: string = 'LoaderScene';
  // for making our loader graphics...
  private loaderBar: Container;
  private loaderBarBoder: Graphics;
  private loaderBarFill: Graphics;
  public assetBundles: string[] = [];

  constructor() {
    super();

    const loaderBarWidth = Manager.width * 0.8;

    this.loaderBarFill = new Graphics();
    this.loaderBarFill.beginFill(0x008800, 1);
    this.loaderBarFill.drawRect(0, 0, loaderBarWidth, 50);
    this.loaderBarFill.endFill();
    this.loaderBarFill.scale.x = 0;

    this.loaderBarBoder = new Graphics();
    this.loaderBarBoder.lineStyle(10, 0x0, 1);
    this.loaderBarBoder.drawRect(0, 0, loaderBarWidth, 50);

    this.loaderBar = new Container();
    this.loaderBar.addChild(this.loaderBarFill);
    this.loaderBar.addChild(this.loaderBarBoder);
    this.loaderBar.position.x = (Manager.width - this.loaderBar.width) / 2;
    this.loaderBar.position.y = (Manager.height - this.loaderBar.height) / 2;
    this.addChild(this.loaderBar);
  }

  public constructorWithAwaits(): void {
    // To be a scene we must have the constructorWithAssets method even if we don't use it.
  }

  public message(/* message: MessageEvent */): void {
    // To be a scene we must have the message method even if we don't use it.
  }

  public update(/* framesPassed: number */): void {
    // To be a scene we must have the update method even if we don't use it.
  }
}
