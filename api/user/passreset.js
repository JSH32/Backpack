const argon = require('argon2')
const auth = require('../../lib/middleware/auth')

module.exports = ({ db, app, config }) => {
  const endpoint = "/api/user/passreset"

  app.use(endpoint, auth(db, { authMethod: "password" }))

  app.post(endpoint, async (req, res) => {
    const { username, newpassword } = req.body

    const Users = db.collection('users')

    if (!newpassword) {
      return res.status(400).send('Please enter a new password!')
    }

    if (newpassword.length < 6) {
      return res.status(400).send('Password too short (minimum 6 characters)')
    } else if (newpassword.length > 256) {
      return res.status(400).send('Password too long (maximum 256 characters)')
    }

    Users.updateOne({ username }, { $set: { password_hash: await argon.hash(req.body.newpassword) } })
    res.status(200).send('The password has been reset!')
  })
}
