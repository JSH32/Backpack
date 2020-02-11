const argon = require('argon2')
const uuid = require('uuid/v4')

module.exports = ({ db, app }) => {
    app.post('/token/valid', async (req, res) =>{
        const { token } = req.body

        const Users = db.collection('users')

        const tokenExists = Boolean(await Users.findOne({ token }))

        if (tokenExists) {
            res.status(200).send('This token is valid!')
        } else {
            res.status(400).send('This token is invalid!')
        }
    })
}