const fileUpload = require('express-fileupload')
const cryptoRandomString = require('crypto-random-string')
const path = require('path')
const fs = require('fs')

module.exports = ({ db, app, config, s3 }) => {

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
            const { username } = await Users.findOne({ token })
            const { lockdown } = await Users.findOne({ username })
            if (lockdown) {
                return res.status(400).send('Invalid Token!')
            } else {
                if (req.files.uploadFile == null || Object.keys(req.files.uploadFile).length === 0) {
                    return res.status(400).send('File not uploaded!')
                } else {
                    // The name of the input field
                    let uploadFile = req.files.uploadFile
                    let md5 = uploadFile.md5
                    
                    // Check if file with same md5 exists to avoid duplicate uploads
                    if (Boolean(await Uploads.findOne({ md5 })) == true) {
                        const { file } = await Uploads.findOne({ md5 })
                        return res.json({
                            'url': config.url + file
                        })
                    } else {
                        let randomString
                        let file
    
                        // File name generation
                        const extension = path.extname(uploadFile.name);
                        randomString = cryptoRandomString({length: config.fileLength, type: 'url-safe'})
                        file = (randomString + extension)
                        // Reroll filename if exists
                        while (Boolean(await Uploads.findOne({ file })) || randomString.includes (".")) {
                            randomString = cryptoRandomString({length: config.fileLength, type: 'url-safe'})
                            file = (randomString + extension)
                        }
                        
                        // Upload file to server and send response
                        uploadFile.mv(config.uploadDir + file).then(async function() {
                            // S3 upload 
                            if (config.s3.enable) {
                                const params = {
                                    Bucket: config.s3.bucket,
                                    Key: file,
                                    Body: fs.readFileSync(config.uploadDir + file),
                                    ACL: 'public-read',
                                    ContentType: uploadFile.mimetype
                                }
                                s3.upload(params, async function(s3Err) {
                                    if (s3Err) throw s3Err
    
                                    await Uploads.insertOne({ file, username, md5 })
                                    fs.unlinkSync(config.uploadDir + file)
                                    return res.json({
                                        'url': config.url + file
                                    })                                   
                                })
                            } else {
                                // Regular upload
                                await Uploads.insertOne({ file, username, md5 })
                                return res.json({
                                    'url': config.url + file
                                })  
                            }
                        })
                    }
                }   
            }
        } else {
            return res.status(400).send('Invalid Token!');
        }
      })
}