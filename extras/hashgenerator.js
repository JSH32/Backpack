const argon = require('argon2');
const prompt = require('prompt');


// Run this to get the admin password
prompt.start();
prompt.get([{
  name: 'password',
  hidden: true,
}], async (err, result) => {
  if (err) {
    console.log(err);
    return;
  }

  const hash = await argon.hash(result.password);
  console.log('hash: ' + hash);
});