const argon = require('argon2')

module.exports = ({ db, app }) => {
    app.post('/api/admin/token/get', async (req, res) =>{
        const { username, password } = req.body

        const Admins = db.collection('admins')

        const userExists = Boolean(await Admins.findOne({ username }))

        if (userExists) {
            const { password_hash } = await Admins.findOne({ username })
            if (await argon.verify(password_hash, password)) {
                const { token } = await Admins.findOne({ username })
                res.status(200).send(token)
            } else {
                res.status(400).send('The username/password you entered is incorrect!')
            }
        } else {
            res.status(400).send('The username/password you entered is incorrect!')
        }
    })
}