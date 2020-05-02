const auth = require('../../../lib/middleware/auth')

module.exports = ({ db, app, config }) => {
  const endpoint = "/api/admin/list/uploads"

  app.use(endpoint, auth(db, { authMethod: "token", database: "admins" }))

  app.post(endpoint, async (req, res) => {
    const { query } = req.body
    const Uploads = db.collection('uploads')

    if (!query) {
      return res.status(400).send('Query is not defined')
    }

    if (query == ' ') {
      const results = (await Uploads.find({}).sort({_id:-1}).toArray() ).map( file => { return {file: file.file, username: file.username} })
      res.status(200).json(results)
    } else {
      const results = (await Uploads.find({ "file": query }).sort({_id:-1}).toArray() ).map( file => { return {file: file.file, username: file.username} })
      res.status(200).json(results)
    }
  })
}