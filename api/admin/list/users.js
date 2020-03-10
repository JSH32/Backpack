const express = require('express');
const argon = require('argon2');

module.exports = ({ db, app }) => {
    app.post('/api/admin/list/users', async (req, res) => {
        const { token, query } = req.body
        const Admins = db.collection('admins')
        const Users = db.collection('users')

        const tokenExists = Boolean(await Admins.findOne({ token }))

        if (!token) {
            return res.status(400).send('Token is not defined')
        } else if (!query) {
            return res.status(400).send('Query is not defined')
        }

        if (tokenExists) {
            if (query == " ") {
                const results = ( await Users.find({}).sort({_id:-1}).toArray() ).map( ({ username }) => username )

                res.status(200).json(results)
            } else {
                const results = ( await Users.find({ "username": query }).sort({_id:-1}).toArray() ).map( ({ username }) => username )

                res.status(200).json(results)
            }
        } else {
            res.status(400).send('Invalid token!')
        } 
    }
)}