const fastify = require('fastify')({ logger: false });
const port = Number(process.env.PORT || 3000);

fastify.get('/', async () => {
  return { message: 'Hello from Codedock Node.js Fastify Example!' };
});

fastify.listen({ port: port, host: '0.0.0.0' }).then(() => {
  console.log('App listening on port ' + port);
}).catch((err) => {
  console.error(err);
  process.exit(1);
});