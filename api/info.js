module.exports = ({ db, app, config }) => {
  app.get('/api/info', async (req, res) => {
    const Uploads = db.collection('uploads')
    var fileCount = await Uploads.countDocuments()

    res.json({
      inviteonly: config.inviteOnly,
      adminreg: config.admin.register,
      totalfiles: fileCount,
      uploadURL: config.url,
      maxuploadsize: config.maxUploadSize
    })
  }
  )
}
