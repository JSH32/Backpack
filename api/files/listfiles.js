const auth = require('../../lib/middleware/auth')

module.exports = ({ db, app, config }) => {
  const endpoint = "/api/files/list"

  app.use(endpoint, auth(db, { authMethod: "token" }))

  app.post(endpoint, async (req, res) => {
    const { token } = req.body
    const Users = db.collection('users')
    const Uploads = db.collection('uploads')

    const { username } = await Users.findOne({ token })

    const results = (
      await Uploads.find({ username }).sort({ _id: -1 }).toArray()
    ).map(({ file }) => file)

    res.status(200).json(results)
  })
}
