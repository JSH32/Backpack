const argon = require('argon2')
const uuid = require('uuid/v4');

module.exports = ({ db, app }) => {
    app.post('/signup', async (req, res) => {
        const { username, password } = req.body

        const Users = db.collection('users')
        // const Sessions = db.collection('sessions')

        if (!username) {
            return res.status(400).send('Username is not defined')
        } else if (!password) {
            return res.status(400).send('Password is not defined')
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
        }  else {
            const password_hash = await argon.hash(password)

            await Users.insertOne({ username, password_hash })

            res.status(200).send('Success!')
        }
    })

    app.post('/login', async (req, res) => {
        
        // Get data from request
        const { username, password } = req.body 
 
        const Users = db.collection('users')
        // const Sessions = db.collection('sessions')
        const userExists = Boolean(await Users.findOne({ username }))
   

        if (userExists) {
            const { password_hash } = await Users.findOne({ username })
            if (await argon.verify(password_hash, password)) {
                res.status(200).send('You have accessed the matrix!')
            } else {
                res.status(400).send('The username/password you entered is incorrect!')
            }
        } else {
            res.status(400).send('The username/password you entered is incorrect!')
        }
    })
}  