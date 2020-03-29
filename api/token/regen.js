const uuid = require('uuid/v4')

module.exports = ({ db, app, config }) => {
    app.post('/api/token/regen', async (req, res) => {
        // Get data from request
        const { token } = req.body 
 
        const Users = db.collection('users')

        const dbtoken = await Users.findOne({ token })

        if (Boolean(await Users.findOne({ token }))) {
            if (dbtoken.token = token) {

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
            } else {
                res.status(400).send('Token does not exist!')
            }
        } else {
            res.status(400).send('Token does not exist!')
        }
    })
}