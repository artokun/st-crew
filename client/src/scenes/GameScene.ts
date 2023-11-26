import { Container, Graphics } from 'pixi.js';
import * as flatbuffers from 'flatbuffers';
import { Manager } from '../Manager';
import { ClientUpdateEventT, InitClientEventT, InitStateEventT, Message, ServerStatEventT } from 'models/message';
import { ClientAction } from 'models/client';
import { UnitScene } from './UnitScene';
import { IScene } from '../types';

export class GameScene extends Container implements IScene {
  public name: string = 'GameScene';
  assetBundles: string[] = ['game', 'sounds'];
  gridGraphics!: Container;

  constructorWithAwaits(): void {
    this.createGrid();

    Manager.ws.send('connected');
  }

  public update(/* framesPassed: number */): void {
    // To be a scene we must have the update method even if we don't use it.
  }

  createGrid(): void {
    this.gridGraphics = new Container();
    this.addChild(this.gridGraphics);

    const gridSize = 50;
    const gridWidth = Manager.width / gridSize;
    const gridHeight = Manager.height / gridSize;

    // make grid
    for (let i = 0; i < gridWidth; i++) {
      for (let j = 0; j < gridHeight; j++) {
        const graphics = new Graphics();
        graphics.lineStyle(1, 0x000000, 0.5);
        graphics.drawRect(0, 0, gridSize, gridSize);
        graphics.position.set(i * gridSize - Manager.width / 2, j * gridSize - Manager.height / 2);
        this.gridGraphics.addChild(graphics);
      }
    }

    // make main axis double width
    const graphics = new Graphics();
    graphics.lineStyle(2, 0x00ff00, 1);
    graphics.moveTo(0, -Manager.height / 2);
    graphics.lineTo(0, Manager.height / 2);
    graphics.lineStyle(2, 0xff0000, 1);
    graphics.moveTo(-Manager.width / 2, 0);
    graphics.lineTo(Manager.width / 2, 0);
    this.gridGraphics.addChild(graphics);
  }

  async message(message: MessageEvent) {
    switch (typeof message.data) {
      case 'string':
        console.log(`${this.name}: ${message.data}`);
        break;
      case 'object': {
        const buffer = new Uint8Array(await message.data.arrayBuffer());
        const event = Message.getRootAsMessage(new flatbuffers.ByteBuffer(buffer)).unpack();
        switch (event.message?.constructor) {
          case InitClientEventT: {
            const { client } = event.message as InitClientEventT;
            Manager.ws.clientId = String(client?.id);
            console.debug('CLIENT ID:', client?.id);
            break;
          }
          case ClientUpdateEventT: {
            const { client, action } = event.message as ClientUpdateEventT;
            console.log(`Client ${client?.name} (${client?.id} just ${ClientAction[action]})`);
            break;
          }
          case InitStateEventT: {
            const { units } = event.message as InitStateEventT;
            for (const unit of units || []) {
              const unitScene = new UnitScene(unit);
              this.addChild(unitScene);
            }
            break;
          }
          case ServerStatEventT: {
            const { clientsConnected } = event.message as ServerStatEventT;
            console.log(`Clients connected: ${clientsConnected}`);
            break;
          }
          default:
            console.error('unknown GameScene event', event);
        }
        break;
      }
      default:
        console.log(`${this.name}: ${message.data}`);
    }
  }

  destroy(/* options?: boolean | IDestroyOptions | undefined */): void {
    Manager.ws.removeEventListener('message', this.message);
  }
}
