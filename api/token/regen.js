const uuid = require('uuid/v4')

module.exports = ({ db, app, config }) => {
    app.post('/api/token/regen', async (req, res) => {
        // Get data from request
        const { token } = req.body 
 
        const Users = db.collection('users')


        if (Boolean(await Users.findOne({ token }))) {
            const { username } = await Users.findOne({ token })
            const { lockdown } = await Users.findOne({ username })
            if (lockdown) {
                res.status(400).send('Token does not exist!')
            } else {
                var newtoken = await uuid()

                // Regen token if exists
                while (Boolean(await Users.findOne({ newtoken }))) {
                    var newtoken = await uuid()
                }

                await Users.updateOne({ token }, { 
                    $set: { 
                        token: newtoken 
                    } 
                })
                res.status(200).send('Token has been regenerated!')
            }
        } else {
            res.status(400).send('Token does not exist!')
        }
    })
}