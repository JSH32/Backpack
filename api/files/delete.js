const express = require('express');
const app = express();
const argon = require('argon2')
const assert = require('assert')
const fs = require('fs')

module.exports = ({ db, app }) => {
    app.post('/api/files/delete', async (req, res) => {
        const { file, token } = req.body
        const Users = db.collection('users')
        const Uploads = db.collection('uploads')
        const tokenExists = Boolean(await Users.findOne({ token }))
        if(tokenExists) {
            const fileExists = Boolean(await Uploads.findOne({ file }))
                if (fileExists) {
                    const datafromtoken = await Users.findOne({ token })
                    
                    const { username } = await Uploads.findOne({ file })
                    if (username === datafromtoken.username) {
                        Uploads.deleteOne({ file : req.body.file }, function(err, result) {
                            assert.equal(err, null)
                            assert.equal(1, result.result.n)
                        });
                        fs.unlinkSync(process.env.UPLOAD_DIR + file)
                        res.status(200).send(file + ' has been deleted!')
                    } else {
                        res.status(400).send('That file does not belong to you!')
                    }
                } else {
                    res.status(400).send('File does not exist!')
            }
        } else {
            res.status(400).send('The username/password you entered is incorrect!');
        }
      });
}