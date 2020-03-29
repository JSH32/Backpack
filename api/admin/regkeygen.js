const uuid = require('uuid/v4')

module.exports = ({ db, app, config }) => {
    app.post('/api/admin/regkeygen', async (req, res) => {
        const { token } = req.body

        const Regkeys = db.collection('regkeys')
        const Admins = db.collection('admins')

        if (config.inviteOnly == false) {
            return res.status(400).send('Regkeys are disabled!')
        } else if (!token) {
            return res.status(400).send('Token is not defined')
        }

        if (Boolean(await Admins.findOne({ token }))) {
            // Create regkey
            var regkey = await uuid()

            // Regen regkey if exists
            while (Boolean(await Regkeys.findOne({ regkey }))) {
                var regkey = uuid()
            }

            // Insert regkey to db
            await Regkeys.insertOne({ regkey })

            // Send regkey back
            res.json({
                'regkey': regkey
            })
        } else {
            res.status(400).send('Invalid Token!')
        }
    
    })
}  