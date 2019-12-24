const express = require('express');
const fileUpload = require('express-fileupload');
const cryptoRandomString = require('crypto-random-string');
const path = require('path');
const app = express();

module.exports = ({ db, app }) => {
    app.use(fileUpload());
    
    app.post('/upload', async (req, res) => {
        const { token } = req.headers
        const Users = db.collection('users')
        const Uploads = db.collection('uploads')
        const tokenExists = Boolean(await Users.findOne({ token }))
        if(tokenExists) {
            if (!req.files.uploadFile || Object.keys(req.files).length === 0) {
                return res.status(400).send('No files were uploaded!');
            } else {
                // The name of the input field
                let uploadFile = req.files.uploadFile;
                // Random file name generation
                var extension = path.extname(uploadFile.name);
                const randomstring = cryptoRandomString({length: 6, type: 'url-safe'});
                const file = (randomstring + extension)
                // Upload file to server
                uploadFile.mv(process.env.UPLOAD_DIR + file)
                // Send URL and put in mongo
                const { username } = await Users.findOne({ token })
                await Uploads.insertOne({ file, username })
                return res.send(process.env.URL + file)
            }   
        } else {
            return res.status(400).send('Invalid Token!');
        }
      });
}