const argon = require('argon2')
const uuid = require('uuid/v4')

module.exports = ({ db, app, config }) => {
  app.post('/api/admin/signup', async (req, res) => {
    const { username, password, adminkey } = req.body

    const Admins = db.collection('admins')

    if (!username) {
      return res.status(400).send('Username is not defined')
    } else if (!password) {
      return res.status(400).send('Password is not defined')
    } else if (!adminkey) {
      return res.status(400).send('Adminkey is not defined')
    }

    const userExists = Boolean(await Admins.findOne({ username }))

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
    
    if (req.body.adminkey != config.admin.key) {
      return res.status(400).send('Invalid adminkey!')
    }

    // Regenerate token if exists
    let token = await uuid()
    while ((await Admins.findOne({ token }))) {
      token = uuid()
    }

    await Admins.insertOne({ username, password_hash: await argon.hash(password), token })

    res.status(200).send('Success!')
  })
}
