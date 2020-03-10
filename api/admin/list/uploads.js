const express = require('express');
const argon = require('argon2');

module.exports = ({ db, app }) => {
    app.post('/api/admin/list/uploads', async (req, res) => {
        const { token, query } = req.body
        const Admins = db.collection('admins')
        const Uploads = db.collection('uploads')

        const tokenExists = Boolean(await Admins.findOne({ token }))

        if (!token) {
            return res.status(400).send('Token is not defined')
        } else if (!query) {
            return res.status(400).send('Query is not defined')
        }

        if (tokenExists) {
            if (query == " ") {
                const results = ( await Uploads.find({}).sort({_id:-1}).toArray() ).map( file => { return {file: file.file, username: file.username} } )

                res.status(200).json(results)
            } else {
                const results = ( await Uploads.find({ "file": query }).sort({_id:-1}).toArray() ).map( file => { return {file: file.file, username: file.username} } )

                res.status(200).json(results)
            }
        } else {
            res.status(400).send('Invalid token!')
        } 
    }
)}