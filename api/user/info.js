const express = require('express');
const app = express();

module.exports = ({ db, app }) => {
    app.post('/api/user/info', async (req, res) => {
        const { token } = req.body
        
        const Uploads = db.collection('uploads')
        const Users = db.collection('users')

        const tokenExists = Boolean(await Users.findOne({ token }))

        if (tokenExists) {
            const { username } = await Users.findOne({ token })
            
            var filecount = await Uploads.countDocuments({ username })
    
            res.json({ 
                'username': username,
                'filecount': filecount
            })
        } else {
            res.status(400).send('Invalid token!')
        }
    });
}