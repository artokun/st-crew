import { Application, Assets } from 'pixi.js';
import { Viewport } from 'pixi-viewport';
import { manifest } from './assets';
import { IScene } from './types';

const WORLD_WIDTH = 2000;
const WORLD_HEIGHT = 2000;

interface GameWebSocket extends WebSocket {
  clientId: string;
}

export class Manager {
  private static app: Application;
  private static currentScene: IScene;

  public static viewport: Viewport;
  public static ws: GameWebSocket;
  public static get screenWidth(): number {
    return Math.max(document.documentElement.clientWidth, window.innerWidth || 0);
  }

  public static get screenHeight(): number {
    return Math.max(document.documentElement.clientHeight, window.innerHeight || 0);
  }

  public static get width(): number {
    return Manager.viewport.worldWidth;
  }

  public static get height(): number {
    return Manager.viewport.worldHeight;
  }

  private static initializeAssetsPromise: Promise<unknown>;
  private static initializeWebsocketPromise?: Promise<void>;

  // Use this function ONCE to start the entire machinery
  public static initialize(background: number): void {
    // Create our pixi app
    Manager.app = new Application({
      view: document.getElementById('pixi-canvas') as HTMLCanvasElement,
      resizeTo: window,
      resolution: window.devicePixelRatio || 1,
      autoDensity: true,
      backgroundColor: background,
    });

    // Initialize pixi debugger
    // @ts-expect-error - pixijs/pixijs#7078
    window.__PIXI_APP__ = Manager.app;

    // Create the viewport
    Manager.viewport = new Viewport({
      screenWidth: Manager.screenWidth,
      screenHeight: Manager.screenHeight,
      worldWidth: WORLD_WIDTH,
      worldHeight: WORLD_HEIGHT,
      events: Manager.app.renderer.events,
      passiveWheel: false,
    });

    // Create world boundaries
    Manager.viewport
      .drag()
      .pinch()
      .wheel()
      .decelerate()
      .moveCenter(0, 0)
      .clamp({
        left: -WORLD_WIDTH,
        right: WORLD_WIDTH,
        top: -WORLD_HEIGHT,
        bottom: WORLD_HEIGHT,
      })
      .clampZoom({
        minWidth: WORLD_WIDTH / 4,
        minHeight: WORLD_HEIGHT / 4,
        maxWidth: WORLD_WIDTH,
        maxHeight: WORLD_HEIGHT,
      })
      .setZoom(1);

    // Add the viewport to the app
    Manager.app.stage.addChild(Manager.viewport);

    // Initialize the websocket
    Manager.initializeWebsocketPromise = Manager.initializeWebsocket();

    // Add the ticker
    Manager.app.ticker.add(Manager.update);

    // listen for the browser telling us that the screen size changed
    window.addEventListener('resize', Manager.resize);

    // call it manually once so we are sure we are the correct size after starting
    Manager.resize();

    // We store it to be sure we can use Assets later on
    Manager.initializeAssetsPromise = Assets.init({ manifest });

    // Black js magic to extract the bundle names into an array.
    const bundleNames = manifest.bundles.map((b) => b.name);

    // Initialize the assets and then start downloading the bundles in the background
    Manager.initializeAssetsPromise.then(() => Assets.backgroundLoadBundle(bundleNames));
  }

  public static async initializeWebsocket(): Promise<void> {
    console.log('Initializing GameServer connection');
    if (Manager.ws) {
      Manager.ws.close();
    }

    let timeout: Timer;

    return Promise.race<void>([
      new Promise((resolve) => {
        Manager.ws = new WebSocket('ws://localhost:3001') as GameWebSocket;
        Manager.ws.onopen = () => {
          console.log('GameServer connection established');
          clearTimeout(timeout);
          resolve();
        };
        Manager.ws.onclose = () => {
          console.log('GameServer connection closed, retrying...');

          setTimeout(() => {
            location.reload();
          }, 1000);
        };
      }),
      new Promise((resolve) => {
        timeout = setTimeout(() => {
          console.log('GameServer connection timed out. retrying...');
          resolve();
        }, 5000);
      }),
    ]).catch(() => Manager.initializeWebsocket());
  }

  public static resize(): void {
    Manager.viewport.resize(Manager.screenWidth, Manager.screenHeight);

    // Recenter the viewport if it is smaller than the world
    if (Manager.viewport.screenWorldWidth < Manager.viewport.worldWidth) {
      Manager.viewport.fitWorld();
      // Manager.viewport.moveCenter(WORLD_WIDTH / 2, WORLD_HEIGHT / 2);
      Manager.viewport.moveCenter(0, 0);
    }

    // current screen size
    const screenWidth = Math.max(document.documentElement.clientWidth, window.innerWidth || 0);
    const screenHeight = Math.max(document.documentElement.clientHeight, window.innerHeight || 0);

    // uniform scale for our game
    const scale = Math.min(screenWidth / Manager.screenWidth, screenHeight / Manager.screenHeight);

    // the "uniformly englarged" size for our game
    const enlargedWidth = Math.floor(scale * Manager.screenWidth);
    const enlargedHeight = Math.floor(scale * Manager.screenHeight);

    // margins for centering our game
    const horizontalMargin = (screenWidth - enlargedWidth) / 2;
    const verticalMargin = (screenHeight - enlargedHeight) / 2;

    // now we use css trickery to set the sizes and margins
    Manager.app.view.style!.width = `${enlargedWidth}px`;
    Manager.app.view.style!.height = `${enlargedHeight}px`;
    // @ts-expect-error - margin does not exist on style dom
    Manager.app.view.style!.marginLeft =
      // @ts-expect-error - margin does not exist on style dom
      Manager.app.view.style!.marginRight = `${horizontalMargin}px`;
    // @ts-expect-error - margin does not exist on style dom
    Manager.app.view.style!.marginTop =
      // @ts-expect-error - margin does not exist on style dom
      Manager.app.view.style!.marginBottom = `${verticalMargin}px`;
  }

  // Call this function when you want to go to a new scene
  public static async changeScene(newScene: IScene): Promise<void> {
    // let's make sure our Assets were initialized correctly
    await Manager.initializeAssetsPromise;

    // Remove and destroy old scene... if we had one..
    if (Manager.currentScene) {
      Manager.viewport.removeChild(Manager.currentScene);
      Manager.currentScene.destroy();
    }

    // If you were to show a loading thingy, this will be the place to show it...

    // Now, let's start downloading the assets we need and wait for them...
    await Promise.all([Assets.loadBundle(newScene.assetBundles), Manager.initializeWebsocketPromise]);

    // If you have shown a loading thingy, this will be the place to hide it...

    // we listen for messages from the server
    Manager.ws.addEventListener('message', newScene.message.bind(newScene));

    // when we have assets and a stable socket connection, we tell that scene
    newScene.constructorWithAwaits();

    // we now store it and show it, as it is completely created
    Manager.currentScene = newScene;
    Manager.viewport.addChild(Manager.currentScene);
  }

  // This update will be called by a pixi ticker and tell the scene that a tick happened
  private static update(framesPassed: number): void {
    // Let the current scene know that we updated it...
    // Just for funzies, sanity check that it exists first.
    if (Manager.currentScene) {
      Manager.currentScene.update(framesPassed);
    }
  }
}
