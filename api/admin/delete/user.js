const argon = require('argon2')
const fs = require('fs')

module.exports = ({ db, app }) => {
    app.post('/api/admin/delete/user', async (req, res) =>{
        const { username, token } = req.body

        const Users = db.collection('users')
        const Uploads = db.collection('uploads')
        const Admins = db.collection('admins')

        const adminExists = Boolean(await Admins.findOne({ token }))
        const userExists = Boolean(await Users.findOne({ username }))

        if (adminExists) {
            if (userExists) {

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
                res.status(400).send('User does not exist')
            }
            
        } else {
            res.status(400).send('Invalid token')
        }
    })
    
}  