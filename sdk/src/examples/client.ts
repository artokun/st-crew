import SDK, { MessageTypes } from '../index';

const sdk = new SDK();

sdk.on('connect', () => {
  console.log('Connected!');

  setInterval(() => {
    sdk.getServerStats();
  }, 500);
});

sdk.on(MessageTypes.GetServer, (data) => {
  console.log('GetServerT:', data);
});

sdk.on('message', (data) => {
  console.log('Message:', data);
});

sdk.on('error', (error) => {
  console.log('Error:', error);
});

sdk.on('disconnect', (event) => {
  console.log('Disconnected:', event);
});

await sdk.connect();
