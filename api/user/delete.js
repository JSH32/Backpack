const argon = require('argon2')
const fs = require('fs')
const chalk = require('chalk')

module.exports = ({ db, app, config, s3 }) => {
    app.post('/api/user/delete', async (req, res) =>{
        const { username, password } = req.body

        const Users = db.collection('users')

        const userExists = Boolean(await Users.findOne({ username }))

        if (userExists) {
            const { lockdown } = await Users.findOne({ username })
            const { password_hash } = await Users.findOne({ username })
            if (await argon.verify(password_hash, password)) {
                if (lockdown) {
                    res.status(400).send('The username/password you entered is incorrect!')
                } else {
                    const Uploads = db.collection('uploads')

                    Users.updateOne({ username }, {$set: { 'lockdown': true }})
                    res.status(200).send('This user has been scheduled for deletion')
    
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
                res.status(400).send('The username/password you entered is incorrect!')
            }
        } else {
            res.status(400).send('The username/password you entered is incorrect!')
        }
    })
    
}  