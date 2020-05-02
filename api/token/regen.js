const uuid = require('uuid/v4')
const auth = require('../../lib/middleware/auth')

module.exports = ({ db, app, config }) => {
  let endpoint = "/api/token/regen"
  
  app.use(endpoint, auth(db, { authMethod: "token", database: "users" }))

  app.post(endpoint, async (req, res) => {
    const Users = db.collection('users')

    let newToken = await uuid()

    // Regen token if exists
    while ((await Users.findOne({ token: newToken }))) {
      newToken = await uuid()
    }

    // Update token in db
    await Users.updateOne({ token: req.body.token }, { $set: { token: newToken } })

    res.status(200).send('Token has been regenerated!')
  }
)}