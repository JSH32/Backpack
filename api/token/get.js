const argon = require('argon2')

module.exports = ({ db, app, config }) => {
    app.post('/api/token/get', async (req, res) =>{
        const { username, password } = req.body

        const Users = db.collection('users')

        const userExists = Boolean(await Users.findOne({ username }))

        if (userExists) {
            const { lockdown } = await Users.findOne({ username })
            if (lockdown) {
                res.status(400).send('The username/password you entered is incorrect!')
            } else {
                const { password_hash } = await Users.findOne({ username })
                if (await argon.verify(password_hash, password)) {
                    const { token } = await Users.findOne({ username })
                    res.status(200).send(token)
                } else {
                    res.status(400).send('The username/password you entered is incorrect!')
                }
            }
        } else {
            res.status(400).send('The username/password you entered is incorrect!')
        }
    })
}