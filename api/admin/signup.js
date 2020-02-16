const argon = require('argon2')
const uuid = require('uuid/v4')

module.exports = ({ db, app }) => {
    app.post('/api/admin/signup', async (req, res) => {
        const { username, password, adminkey } = req.body

        const Admins = db.collection('admins')

        if (!username) {
            return res.status(400).send('Username is not defined')
        } else if (!password) {
            return res.status(400).send('Password is not defined')
        }  else if (!adminkey) {
            return res.status(400).send('Adminkey is not defined')
        }

        const userExists = Boolean(await Admins.findOne({ username }))

        if (username.length < 4) {
            res.status(400).send('Username too short (minimum 4 characters)')
        } else if (username.length > 15) {
            res.status(400).send('Username too long (maximum 15 characters)')
        } else if (userExists) {
            res.status(400).send('User already exists')
        } else if (password.length < 6) {
            res.status(400).send('Password too short (minimum 6 characters)')
        } else if (password.length > 256) {
            res.status(400).send('Password too long (maximum 256 characters)')
        } else if (req.body.adminkey == process.env.ADMINKEY) {
            const password_hash = await argon.hash(password)
    
            var token = await uuid()
            
            // Regen token if exists
            while (Boolean(await Admins.findOne({ token }))) {
                var token = uuid()
            }
    
            await Admins.insertOne({ username, password_hash, token })
    
            res.status(200).send('Success!')
        } else {
            res.status(400).send('Incorrect adminkey!')
        }
    })
}  