const argon = require('argon2')
const fs = require('fs')

module.exports = ({ db, app }) => {
    app.post('/api/user/delete', async (req, res) =>{
        const { username, password } = req.body

        const Users = db.collection('users')

        const userExists = Boolean(await Users.findOne({ username }))

        if (userExists) {
            const { password_hash } = await Users.findOne({ username })
            if (await argon.verify(password_hash, password)) {
                const Uploads = db.collection('uploads')

                var uploadexist = Boolean(await Uploads.findOne({ username }))
                while (uploadexist) {
                    var { file } = await Uploads.findOne({ username })
                    await Uploads.deleteOne({ file })
                    var uploadexist = Boolean(await Uploads.findOne({ username }))
                    fs.unlinkSync(process.env.UPLOAD_DIR + file)
                }

                await Users.deleteOne({ username })
                res.status(200).send('This user has been deleted')
                
            } else {
                res.status(400).send('The username/password you entered is incorrect!')
            }
        } else {
            res.status(400).send('The username/password you entered is incorrect!')
        }
    })
    
}  