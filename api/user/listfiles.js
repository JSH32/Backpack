const express = require('express');
const argon = require('argon2')
const app = express();
const assert = require('assert')

module.exports = ({ db, app }) => {
    app.post('/user/listfiles', async (req, res) => {
        const { username, password } = req.body
        const Users = db.collection('users')
        const Uploads = db.collection('uploads')

        const { password_hash, token } = await Users.findOne({ username })

        if (password_hash && await argon.verify(password_hash, password)) {
            Uploads.find({username}, {projection:{_id: 0, username: 0}})
            .toArray(function(err, result) {
                if (err) throw err;
                res.status(200).json(result)
            });

        } else {
            res.status(400).send('The username/password you entered is incorrect!')
        }
    }
)}