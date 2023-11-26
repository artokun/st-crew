/// <reference lib="dom" />
/// <reference lib="dom.iterable" />

import { Manager } from './Manager';
import { GameScene } from './scenes/GameScene';

Manager.initialize(0x6495ed);

Manager.changeScene(new GameScene());
