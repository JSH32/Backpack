const express = require('express');

module.exports = ({ db, app, totalsize }) => {
    app.get('/api/info', async (req, res) => {
        const Uploads = db.collection('uploads')
        var filecount = await Uploads.countDocuments()

        res.json({
            'inviteonly': JSON.parse(process.env.INVITEONLY),
            'adminreg': JSON.parse(process.env.ADMINREGISTER),
            'totalfiles': filecount,
            'uploadURL': process.env.URL
        })
    }
)}