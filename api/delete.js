const express = require('express');
const app = express();
const argon = require('argon2')
const assert = require('assert')
const fs = require('fs')

module.exports = ({ db, app }) => {
    app.post('/delete', async (req, res) => {
        const { file, username, password } = req.body
        const Users = db.collection('users')
        const Uploads = db.collection('uploads')
        const userExists = Boolean(await Users.findOne({ username }))
        if(userExists) {
            const { password_hash } = await Users.findOne({ username })
            if (await argon.verify(password_hash, password)) {
                const fileExists = Boolean(await Uploads.findOne({ file }))
                if (fileExists) {
                    const { username } = await Uploads.findOne({ file })
                    if (username === req.body.username) {
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
                res.status(400).send('The username/password you entered is incorrect!')
            }
        } else {
            res.status(400).send('The username/password you entered is incorrect!');
        }
      });
}