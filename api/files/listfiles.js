const express = require('express');
const argon = require('argon2')
const app = express();
const assert = require('assert')

module.exports = ({ db, app }) => {
    app.post('/files/list', async (req, res) => {
        const { username, password } = req.body
        const Users = db.collection('users')
        const Uploads = db.collection('uploads')

        const userExists = Boolean(await Users.findOne({ username }))
        if (userExists) {
            const { password_hash } = await Users.findOne({ username })

            if (password_hash && await argon.verify(password_hash, password)) {
                const results = (
                    await Uploads.find({ username }).toArray()
                ).map( ({ file }) => file )
                res.status(200).json(results)
    
            } else {
                res.status(400).send('The username/password you entered is incorrect!')
            }
        } else {
            res.status(400).send('The username/password you entered is incorrect!')
        }
    }
)}