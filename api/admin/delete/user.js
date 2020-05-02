const fs = require('fs')
const chalk = require('chalk')
const auth = require('../../../lib/middleware/auth')

module.exports = ({ db, app, config, s3 }) => {
  const endpoint = "/api/admin/delete/user"

  app.use(endpoint, auth(db, { authMethod: "token", database: "admins" }))

  app.post(endpoint, async (req, res) => {
    const { username } = req.body

    const Users = db.collection('users')
    const Uploads = db.collection('uploads')

    await Users.updateOne({ username }, { $set: { lockdown: true } })
    res.status(200).send('This user is now scheduled for deletion')

    let uploadExists = Boolean(await Uploads.findOne({ username }))
    while (uploadExists) {
      let { file } = await Uploads.findOne({ username })
      await Uploads.deleteOne({ file })
      if (config.s3.enable) {
        const params = {
          Bucket: config.s3.bucket,
          Key: file
        }
        s3.deleteObject(params, function (err) {
          if (err) {
            console.log(chalk.yellow(`[WARN] ${config.uploadDir + file} was requested to be deleted but there was an issue!`))
          }
        })
      } else {
        if (fs.existsSync(config.uploadDir + file)) {
          fs.unlinkSync(config.uploadDir + file)
        } else {
          console.log(chalk.yellow(`[WARN] ${config.uploadDir + file} was requested to be deleted but didn't exist!`))
        }
      }
      uploadExists = Boolean(await Uploads.findOne({ username }))
    }

    await Users.deleteOne({ username })
  })
}
