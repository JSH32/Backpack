const assert = require('assert')
const fs = require('fs')
const chalk = require('chalk')

module.exports = ({ db, app, config, s3 }) => {
    app.post('/api/files/delete', async (req, res) => {
        const { file, token } = req.body
        const Users = db.collection('users')
        const Uploads = db.collection('uploads')
        const tokenExists = Boolean(await Users.findOne({ token }))
    
        if(tokenExists) {
            const { username } = await Users.findOne({ token })
            const { lockdown } = await Users.findOne({ username })
            if (lockdown) {
                res.status(400).send('Invalid token!')
            } else {
                const fileExists = Boolean(await Uploads.findOne({ file }))
                if (fileExists) {
                    const userData = await Users.findOne({ token })
                    
                    const fileInfo = await Uploads.findOne({ file })
                    if (fileInfo.username === userData.username) {
                        Uploads.deleteOne({ file : req.body.file }, function(err, result) {
                            assert.equal(err, null)
                            assert.equal(1, result.result.n)
                        })
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
                        res.status(200).send(file + ' has been deleted!')
                    } else {
                        res.status(400).send('That file does not belong to you!')
                    }
                } else {
                    res.status(400).send('File does not exist!')
            }
            }
        } else {
            res.status(400).send('Invalid token!');
        }
      });
}