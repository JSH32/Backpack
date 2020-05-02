const fileUpload = require('express-fileupload')
const cryptoRandomString = require('crypto-random-string')
const path = require('path')
const fs = require('fs')
const auth = require('../../lib/middleware/auth')

module.exports = ({ db, app, config, s3 }) => {
  const endpoint = "/api/files/upload"

  app.use(endpoint, fileUpload({
    limits: { fileSize: config.maxUploadSize * 1024 * 1024 },
    abortOnLimit: true,
    createParentPath: true
  }))

  app.use(endpoint, auth(db, { authMethod: "token", useTokenHeaders: true }))

  app.post(endpoint, async (req, res) => {
    const { token } = req.headers

    if (!req.files || !req.files.uploadFile || Object.keys(req.files.uploadFile).length === 0) {
      return res.status(400).send("File not uploaded!")
    }

    const Uploads = db.collection("uploads")
    const Users = db.collection("users")
    
    const { username } = await Users.findOne({ token })

    const uploadFile = req.files.uploadFile
    const md5 = uploadFile.md5

    // Check if file with same md5 exists to avoid duplicate uploads
    if (Boolean(await Uploads.findOne({ md5 }))) {
      const { file } = await Uploads.findOne({ md5 })
      return res.json({ url: config.url + file })
    }

    let randomString
    let fileName

    // File name generation
    const extension = path.extname(uploadFile.name)
    randomString = cryptoRandomString({ length: config.fileLength, type: 'url-safe' })
    fileName = (randomString + extension)

    // Reroll filename if exists
    while (Boolean(await Uploads.findOne({ file: fileName })) || randomString.includes('.')) {
      randomString = cryptoRandomString({ length: config.fileLength, type: 'url-safe' })
      fileName = (randomString + extension)
    }

    // Upload file to server and send response
    uploadFile.mv(config.uploadDir + fileName).then(async function () {
    // S3 upload
    if (config.s3.enable) {
      const params = {
        Bucket: config.s3.bucket,
        Key: fileName,
        Body: fs.readFileSync(config.uploadDir + fileName),
        ACL: 'public-read',
        ContentType: uploadFile.mimetype
      }
      s3.upload(params, async function (s3Err) {
        if (s3Err) throw s3Err
  
        await Uploads.insertOne({ file: fileName, username, md5 })
        fs.unlinkSync(config.uploadDir + fileName)
        return res.json({ url: config.url + fileName })
      })
    } else {
      // Regular upload
      await Uploads.insertOne({ file: fileName, username, md5 })
      return res.json({ url: config.url + fileName })
    }
    })
  })
}