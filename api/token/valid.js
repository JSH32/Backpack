module.exports = ({ db, app }) => {
    app.post('/api/token/valid', async (req, res) =>{
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