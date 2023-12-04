import SDK from '../index';

const sdk = new SDK();

sdk.on('connect', () => {
  console.log('Connected!');

  setInterval(async () => {
    const result = await sdk.getServerStats();

    console.log('rpc response:', result);
  }, 3000);
});

sdk.on('server_info', (data) => {
  console.log('server_info:', data);
});

sdk.on('energy_changed', (data) => {
  console.log('energy_changed:', data);
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
