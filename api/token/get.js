const auth = require('../../lib/middleware/auth')

module.exports = ({ db, app, config }) => {
  const endpoint = "/api/token/get"

  app.use(endpoint, auth(db, { authMethod: "password" }))

  app.post(endpoint, async (req, res) => {
    const { token } = await db.collection("users").findOne({ "username": req.body.username })
    res.status(200).send(token)
  })
}
