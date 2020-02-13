const express = require('express');
const app = express();

module.exports = ({ db, app }) => {
    app.get('/api/files/total', async (req, res) => {
        const Uploads = db.collection('uploads')
        var filecount = await Uploads.countDocuments()

        res.status(200).send(`${filecount}`)
    });
}