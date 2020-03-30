const fileUpload = require('express-fileupload')
const cryptoRandomString = require('crypto-random-string')
const path = require('path')

module.exports = ({ db, app, config }) => {

    app.use(fileUpload({
        limits: { fileSize: config.maxUploadSize * 1024 * 1024 },
        abortOnLimit: true,
        createParentPath: true
    }))
    
    app.post('/api/files/upload', async (req, res) => {
        const { token } = req.headers
        const Users = db.collection('users')
        const Uploads = db.collection('uploads')
        const tokenExists = Boolean(await Users.findOne({ token }))
        if(tokenExists) {
            if (req.files == null || Object.keys(req.files).length === 0) {
                return res.status(400).send('File not uploaded!')
            } else {
                // The name of the input field
                let uploadFile = req.files.uploadFile;
                let md5 = uploadFile.md5

                // Check if file with same md5 exists to avoid duplicates
                if (Boolean(await Uploads.findOne({ md5 })) == true) {
                    const { file } = await Uploads.findOne({ md5 })
                    return res.json({
                        'url': config.url + file
                })
                } else {
                    let randomstring
                    let file

                    // File name generation
                    const extension = path.extname(uploadFile.name);
                    randomstring = cryptoRandomString({length: config.fileLength, type: 'url-safe'})
                    file = (randomstring + extension)
                    // Reroll filename if similar
                    while (Boolean(await Uploads.findOne({ file })) || randomstring.includes (".")) {
                        randomstring = cryptoRandomString({length: config.fileLength, type: 'url-safe'})
                        file = (randomstring + extension)
                    }
                    
                    // Upload file to server and send response
                    uploadFile.mv(config.uploadDir + file).then(async function () {
                    const { username } = await Users.findOne({ token })
                    await Uploads.insertOne({ file, username, md5 })
                        return res.json({
                            'url': config.url + file
                        })
                    })

                }
            }   
        } else {
            return res.status(400).send('Invalid Token!');
        }
      })
}