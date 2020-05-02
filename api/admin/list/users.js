const auth = require('../../../lib/middleware/auth')

module.exports = ({ db, app, config }) => {
  const endpoint = "/api/admin/list/users"

  app.use(endpoint, auth(db, { authMethod: "token", database: "admins" }))

  app.post(endpoint, async (req, res) => {
    const { query } = req.body
    const Users = db.collection('users')

    if (!query) {
      return res.status(400).send('Query is not defined')
    }

    if (query == ' ') {
      const results = (await Users.find({}).sort({_id:-1}).toArray() ).map( username  => { return {username: username.username, lockdown: username.lockdown} })
      res.status(200).json(results)
    } else {
      const results = (await Users.find({ "username": query }).sort({_id:-1}).toArray() ).map( username => { return {username: username.username, lockdown: username.lockdown} })
      res.status(200).json(results)
    }
  })
}