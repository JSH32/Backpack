const auth = require('../../lib/middleware/auth')

module.exports = ({ db, app, config }) => {
  let endpoint = "/api/token/valid"

  app.use(endpoint, auth(db, { authMethod: "token" }))

  app.post(endpoint, async (req, res) => {
    res.status(200).send('This token is valid!')
  })
}