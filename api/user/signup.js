const argon = require('argon2')
const uuid = require('uuid/v4')

module.exports = ({ db, app, config }) => {
  app.post('/api/user/signup', async (req, res) => {
    const { username, password, regkey } = req.body

    const Regkeys = db.collection('regkeys')
    const Users = db.collection('users')

    if (!username) {
      return res.status(400).send('Username is not defined')
    } else if (!password) {
      return res.status(400).send('Password is not defined')
    } else if (config.inviteOnly && !regkey) {
      return res.status(400).send('Regkey is not defined')
    }

    const userExists = Boolean(await Users.findOne({ username }))
    const regExists = Boolean(await Regkeys.findOne({ regkey }))

    if (username.length < 4) {
      return res.status(400).send('Username too short (minimum 4 characters)')
    } else if (username.length > 15) {
      return res.status(400).send('Username too long (maximum 15 characters)')
    } else if (userExists) {
      return res.status(400).send('User already exists')
    } else if (password.length < 6) {
      return res.status(400).send('Password too short (minimum 6 characters)')
    } else if (password.length > 256) {
      return res.status(400).send('Password too long (maximum 256 characters)')
    } 
    
    if (config.inviteOnly) {
      if (!regExists) {
        return res.status(400).send('Invalid regkey')
      }

      await Regkeys.deleteOne({ regkey }) // Delete old regkey
    }

    // Generate token
    let token = await uuid()
    while ((await Users.findOne({ token }))) {
      token = uuid()
    }

    await Users.insertOne({ username, password_hash: await argon.hash(password), token, lockdown: false })

    res.status(200).send('Success!')
  })
}
