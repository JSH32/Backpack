const argon = require('argon2')

module.exports = ({ db, app }) => {
    app.post('/api/user/passreset', async (req, res) =>{
        const { username, password, newpassword } = req.body

        const Users = db.collection('users')
        
        if (!username) {
            return res.status(400).send('Please enter your username!')
        } else if (!password) {
            return res.status(400).send('Please enter your password!')
        } else if (!newpassword) {
            return res.status(400).send('Please enter a new password!')
        }

        const userExists = Boolean(await Users.findOne({ username }))

        if (userExists) {


            if (newpassword.length < 6) {
                res.status(400).send('Password too short (minimum 6 characters)')
            } else if (newpassword.length > 256) {
                res.status(400).send('Password too long (maximum 256 characters)')
            } else {
                const { password_hash } = await Users.findOne({ username })
                if (await argon.verify(password_hash, password)) {
                    const newpassword_hash = await argon.hash(req.body.newpassword)
                    Users.updateOne({ username }, {$set: { "password_hash": newpassword_hash }})
                    res.status(200).send('The password has been reset!')
                } else {
                    res.status(400).send('The password you entered is incorrect!')
                }
            }

        } else {
            res.status(400).send('The password you entered is incorrect!')
        }
    })
}