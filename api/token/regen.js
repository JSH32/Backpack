const argon = require('argon2')
const uuid = require('uuid/v4')

module.exports = ({ db, app }) => {
    app.post('/token/regen', async (req, res) => {
        // Get data from request
        const { username, password } = req.body 
 
        const Users = db.collection('users')
        const userExists = Boolean(await Users.findOne({ username }))

        if (userExists) {
            const { password_hash } = await Users.findOne({ username })
            if (await argon.verify(password_hash, password)) {
                
                const { token } = await Users.findOne({ username })
                const newtoken = await uuid()
                await Users.updateOne({ token :  token }, { $set: { token : newtoken } })
                res.status(200).send('Token has been regenerated!')
            } else {
                res.status(400).send('The username/password you entered is incorrect!')
            }
        } else {
            res.status(400).send('The username/password you entered is incorrect!')
        }
    })
}