const argon = require('argon2')
const uuid = require('uuid/v4')

module.exports = ({ db, app }) => {
    app.post('/token/regen', async (req, res) => {
        // Get data from request
        const { token } = req.body 
 
        const Users = db.collection('users')

        const dbtoken = await Users.findOne({ token })

        if (Boolean(await Users.findOne({ token }))) {
            if (dbtoken.token = token) {
                await Users.updateOne({ token }, { 
                    $set: { 
                        token: uuid() 
                    } 
                })
                
                res.status(200).send('Token has been regenerated!')
            } else {
                res.status(400).send('Token does not exist!')
            }
        } else {
            res.status(400).send('Token does not exist!')
        }
    })
}