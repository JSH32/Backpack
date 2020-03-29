module.exports = ({ db, app }) => {
    app.post('/api/admin/token/valid', async (req, res) =>{
        const { token } = req.body

        const Admins = db.collection('admins')

        const tokenExists = Boolean(await Admins.findOne({ token }))

        if (tokenExists) {
            res.status(200).send('This token is valid!')
        } else {
            res.status(400).send('This token is invalid!')
        }
    })
}