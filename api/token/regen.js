const argon = require('argon2')
const uuid = require('uuid/v4')

module.exports = ({ db, app }) => {
    app.post('/token/regen', async (req, res) => {
        // Get data from request
        const { username, password } = req.body 
 
        const Users = db.collection('users')

        const { password_hash, token } = await Users.findOne({ username })

        if (password_hash && await argon.verify(password_hash, password)) {
            await Users.updateOne({ token }, { 
                $set: { 
                    token: uuid() 
                } 
            })
            
            res.status(200).send('Token has been regenerated!')
        } else {
            res.status(400).send('The username/password you entered is incorrect!')
        }
    })
}