const uuid = require('uuid/v4')
const auth = require('../../lib/middleware/auth')

module.exports = ({ db, app, config }) => {
  let endpoint = "/api/admin/regkeygen"

  app.use(endpoint, auth(db, { authMethod: "token", database: "admins" }))

  app.post(endpoint, async (req, res) => {

    const Regkeys = db.collection('regkeys')

    let regkey = await uuid()

    // Regenerate regkey if exists in db
    while ((await Regkeys.findOne({ regkey }))) {
      regkey = uuid()
    }

    // Insert regkey to db
    await Regkeys.insertOne({ regkey })

    // Send regkey back
    res.json({
      regkey: regkey
    })
    
  })
}
