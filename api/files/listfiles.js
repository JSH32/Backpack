const express = require('express');
const argon = require('argon2')
const app = express();
const assert = require('assert')

module.exports = ({ db, app }) => {
    app.post('/api/files/list', async (req, res) => {
        const { token } = req.body
        const Users = db.collection('users')
        const Uploads = db.collection('uploads')

        const tokenExists = Boolean(await Users.findOne({ token }))
        if (tokenExists) {
                
                const { username } = await Users.findOne({ token })
                const results = (
                    await Uploads.find({ username }).toArray()
                ).map( ({ file }) => file )

                res.status(200).json(results)
    
        } else {
            res.status(400).send('The username/password you entered is incorrect!')
        }
    }
)}