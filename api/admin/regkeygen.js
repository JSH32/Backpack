const argon = require('argon2')
const uuid = require('uuid/v4')

module.exports = ({ db, app }) => {
    app.post('/api/admin/regkeygen', async (req, res) => {
        const { password } = req.body

        const Regkeys = db.collection('regkeys')

        if (!password) {
            return res.status(400).send('Password is not defined')
        }

        if (await argon.verify(process.env.ADMINPASS, password)) {
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
            res.status(400).send('Incorrect password!')
        }
    
    })
}  