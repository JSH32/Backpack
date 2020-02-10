const argon = require('argon2')
const uuid = require('uuid/v4')

module.exports = ({ db, app }) => {
    app.post('/user/signup', async (req, res) => {
        const { username, password, regkey } = req.body

        const Users = db.collection('users')
        const Uploads = db.collection('uploads')
        // const Sessions = db.collection('sessions')

        if (!username) {
            return res.status(400).send('Username is not defined')
        } else if (!password) {
            return res.status(400).send('Password is not defined')
        } else if (!regkey) {
            return res.status(400).send('Regkey is not defined')
        }

        const userExists = Boolean(await Users.findOne({ username }))

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
        } else if (regkey == process.env.regkey) {
            const password_hash = await argon.hash(password)

            const token = await uuid()

            await Users.insertOne({ username, password_hash, token })

            res.status(200).send('Success!')
        } else {
            res.status(400).send('Invalid regkey')
        }
    })
}  