const fs = require('fs')
const chalk = require('chalk')
const uuid = require('uuid')

module.exports = ({ db, app, config, s3 }) => {
    app.post('/api/admin/delete/user', async (req, res) =>{
        const { username, token } = req.body

        const Users = db.collection('users')
        const Uploads = db.collection('uploads')
        const Admins = db.collection('admins')

        const adminExists = Boolean(await Admins.findOne({ token }))
        const userExists = Boolean(await Users.findOne({ username }))
        const { lockdown } = await Users.findOne({ username })

        if (adminExists) {
            if (userExists) {
                if (lockdown) {
                    res.status(400).send('That user is in lockdown')
                } else {
                    Users.updateOne({ username }, {$set: { 'lockdown': true }})
                    res.status(200).send('This user is now scheduled for deletion')
    
                    var uploadExists = Boolean(await Uploads.findOne({ username }))
                    while (uploadExists) {
                        var { file } = await Uploads.findOne({ username })
                        await Uploads.deleteOne({ file })
                        if (config.s3.enable) {
                            const params = { 
                                Bucket: config.s3.bucket, 
                                Key: file 
                            }
                            s3.deleteObject(params, function(err) {
                                if (err) {
                                    console.log(chalk.yellow(`[WARN] ${config.uploadDir + file} was requested to be deleted but there was an issue!`))
                                }
                            })
                        } else {
                            if (fs.existsSync(config.uploadDir + file)) {
                                fs.unlinkSync(config.uploadDir + file)
                            } else {
                                console.log(chalk.yellow(`[WARN] ${config.uploadDir + file} was requested to be deleted but didn't exist!`))
                            }
                        }
                        var uploadExists = Boolean(await Uploads.findOne({ username }))
                    }
                    await Users.deleteOne({ username })
                }
            } else {
                res.status(400).send('User does not exist')
            }
            
        } else {
            res.status(400).send('Invalid token')
        }
    })
    
}  