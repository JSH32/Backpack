const auth = require('../../../lib/middleware/auth')

module.exports = ({ db, app, config }) => {
  const endpoint = "/api/admin/token/get"

  app.use(endpoint, auth(db, { authMethod: "password", database: "admins" }))

  app.post(endpoint, async (req, res) => {
    const { token } = await db.collection("admins").findOne({ "username": req.body.username })
    res.status(200).send(token)
  })
}
