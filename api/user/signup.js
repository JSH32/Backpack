const argon = require('argon2')
const uuid = require('uuid/v4')

module.exports = ({ db, app, config }) => {
    app.post('/api/user/signup', async (req, res) => {
        const { username, password, regkey } = req.body

        const Regkeys = db.collection('regkeys')
        const Users = db.collection('users')

        if (!username) {
            return res.status(400).send('Username is not defined')
        } else if (!password) {
            return res.status(400).send('Password is not defined')
        } else if (config.inviteOnly == true) {
            if (!regkey) {
                return res.status(400).send('Regkey is not defined')
            }
        }

        const userExists = Boolean(await Users.findOne({ username }))
        const regExists = Boolean(await Regkeys.findOne({ regkey }))

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
        } else if (config.inviteOnly == true) {
            if (await Regkeys.findOne({ regkey })) {
                const password_hash = await argon.hash(password)
    
                var token = await uuid()
            
                // Regen token if exists
                while (Boolean(await Users.findOne({ token }))) {
                    var token = uuid()
                }

                await Users.insertOne({ username, password_hash, token })
    
                await Regkeys.deleteOne({ regkey })
    
                res.status(200).send('Success!')
            } else {
                res.status(400).send('Invalid regkey')
            }
        } else if (config.inviteOnly == false) {
            const password_hash = await argon.hash(password)
    
            var token = await uuid()
            
            // Regen token if exists
            while (Boolean(await Users.findOne({ token }))) {
                var token = uuid()
            }
    
            await Users.insertOne({ username, password_hash, token })
    
            res.status(200).send('Success!')
        } else {
            res.status(400).send('Error!')
        }
    })
}  