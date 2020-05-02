const auth = require('../../lib/middleware/auth')

module.exports = ({ db, app, config }) => {
  const endpoint = "/api/user/info"

  app.use(endpoint, auth(db, { authMethod: "token" }))

  app.post(endpoint, async (req, res) => {
    const userDat = await db.collection("users").findOne({ token: req.body.token })
    const filecount = await db.collection("uploads").countDocuments({ username: userDat.username })

    // Send user info
    res.status(200).json({
      username: userDat.username,
      filecount: filecount
    })

  })
}
