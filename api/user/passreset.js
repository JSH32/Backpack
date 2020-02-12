const argon = require('argon2')
const uuid = require('uuid/v4')

module.exports = ({ db, app }) => {
    app.post('/user/passreset', async (req, res) =>{
        const { username, password, newpassword } = req.body

        const Users = db.collection('users')
        
        if (!username) {
            return res.status(400).send('Username is not defined')
        } else if (!password) {
            return res.status(400).send('Password is not defined')
        } else if (!newpassword) {
            return res.status(400).send('New password is not defined')
        }

        const userExists = Boolean(await Users.findOne({ username }))

        if (userExists) {


            if (username.length < 4) {
                res.status(400).send('Username too short (minimum 4 characters)')
            } else if (username.length > 15) {
                res.status(400).send('Username too long (maximum 15 characters)')
            } else if (newpassword.length < 6) {
                res.status(400).send('Password too short (minimum 6 characters)')
            } else if (newpassword.length > 256) {
                res.status(400).send('Password too long (maximum 256 characters)')
            } else {
                const { password_hash } = await Users.findOne({ username })
                if (!newpassword) {
                res.status(400).send('No new password provided!')

                } else if (await argon.verify(password_hash, password)) {
                    const password_hash = await argon.hash(req.body.newpassword)
                    Users.updateOne({ username }, {$set: { "password_hash": password_hash }})
                    res.status(200).send('The password has been reset!')
                } else {
                    res.status(400).send('The username/password you entered is incorrect!')
                }
            }

        } else {
            res.status(400).send('The username/password you entered is incorrect!')
        }
    })
}