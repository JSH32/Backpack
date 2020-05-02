const assert = require('assert')
const fs = require('fs')
const chalk = require('chalk')
const auth = require('../../../lib/middleware/auth')

module.exports = ({ db, app, config, s3 }) => {
  const endpoint = "/api/admin/delete/file"

  app.use(endpoint, auth(db, { authMethod: "token", database: "admins" }))
  
  app.post(endpoint, async (req, res) => {
    const { file } = req.body
    const Uploads = db.collection('uploads')

    const fileExists = Boolean(await Uploads.findOne({ file }))

    if (!fileExists) {
      return res.status(400).send('File does not exist!')
    }

    await Uploads.deleteOne({ file: req.body.file })

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
      if (fs.existsSync(config.uploadDir + file))  {
        fs.unlinkSync(config.uploadDir + file)
      } else {
        console.log(chalk.yellow(`[WARN] ${config.uploadDir + file} was requested to be deleted but didn't exist!`))
      }
    }

    res.status(200).send(file + ' has been deleted!')
  })
}
