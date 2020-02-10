const argon = require('argon2')
const uuid = require('uuid/v4')

module.exports = ({ db, app }) => {
    app.post('/user/passreset', async (req, res) =>{
        const { username, password, newpassword } = req.body

        const Users = db.collection('users')
        

        const userExists = Boolean(await Users.findOne({ username }))

        if (userExists) {
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
        } else {
            res.status(400).send('The username/password you entered is incorrect!')
        }
    })
}