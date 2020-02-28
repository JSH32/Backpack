const express = require('express');
const fileUpload = require('express-fileupload');
const cryptoRandomString = require('crypto-random-string');
const path = require('path');
const app = express();

module.exports = ({ db, app }) => {

    app.use(fileUpload({
        limits: { fileSize: parseInt(process.env.MAXUPLOADSIZE) * 1024 * 1024 },
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
                return res.status(400).send('No files were uploaded!');
            } else {
                // The name of the input field
                let uploadFile = req.files.uploadFile;
                let md5 = uploadFile.md5

                // Check if file with same md5 exists to avoid duplicates
                if (Boolean(await Uploads.findOne({ md5 })) == true) {
                    const {file} = await Uploads.findOne({ md5 })
                    return res.json({
                        'url': process.env.URL + file
                    })
                } else {
                    // File name generation
                    const extension = path.extname(uploadFile.name);
                    var randomstring = cryptoRandomString({length: parseInt(process.env.FILELENGTH), type: 'url-safe'});
                    while (randomstring.includes (".")) {
                    var randomstring = cryptoRandomString({length: parseInt(process.env.FILELENGTH), type: 'url-safe'});
                    }
                    var file = (randomstring + extension)
                    // If value found in database then reroll filename
                    while (Boolean(await Uploads.findOne({ file }))) {
                        var randomstring = cryptoRandomString({length: parseInt(process.env.FILELENGTH), type: 'url-safe'});
                        var file = (randomstring + extension)
                    }
                    // Upload file to server
                    uploadFile.mv(process.env.UPLOAD_DIR + file).then(function () {
                        postUpload()
                    })

                    // Send response and filedata to database 
                    async function postUpload () {
                        const { username } = await Users.findOne({ token })
                        await Uploads.insertOne({ file, username, md5 })
                        return res.json({
                            'url': process.env.URL + file
                        })
                    }
                }
            }   
        } else {
            return res.status(400).send('Invalid Token!');
        }
      });
}