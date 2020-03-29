const fs = require('fs')
const chalk = require('chalk')

module.exports = ({ db, app, config }) => {
    app.post('/api/admin/delete/user', async (req, res) =>{
        const { username, token } = req.body

        const Users = db.collection('users')
        const Uploads = db.collection('uploads')
        const Admins = db.collection('admins')

        const adminExists = Boolean(await Admins.findOne({ token }))
        const userExists = Boolean(await Users.findOne({ username }))

        if (adminExists) {
            if (userExists) {

                var uploadExists = Boolean(await Uploads.findOne({ username }))
                while (uploadExists) {
                    var { file } = await Uploads.findOne({ username })
                    await Uploads.deleteOne({ file })
                    var uploadExists = Boolean(await Uploads.findOne({ username }))
                    if (fs.existsSync(config.uploadDir + file)) {
                        fs.unlinkSync(config.uploadDir + file)
                    } else {
                        console.log(chalk.yellow(`[WARN] ${config.uploadDir + file} was requested to be deleted but didn't exist!`))
                    }
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