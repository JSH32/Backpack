const assert = require('assert')
const fs = require('fs')
const chalk = require('chalk')

module.exports = ({ db, app, config }) => {
    app.post('/api/admin/delete/file', async (req, res) => {
        const { file, token } = req.body
        const Admins = db.collection('admins')
        const Uploads = db.collection('uploads')
        const tokenExists = Boolean(await Admins.findOne({ token }))
        if(tokenExists) {
            const fileExists = Boolean(await Uploads.findOne({ file }))
                if (fileExists) {
                        Uploads.deleteOne({ file : req.body.file }, function(err, result) {
                            assert.equal(err, null)
                            assert.equal(1, result.result.n)
                        });
                        if (fs.existsSync(config.uploadDir + file)) {
                            fs.unlinkSync(config.uploadDir + file)
                        } else {
                            console.log(chalk.yellow(`[WARN] ${config.uploadDir + file} was requested to be deleted but didn't exist!`))
                        }
                        res.status(200).send(file + ' has been deleted!')   
                } else {
                    res.status(400).send('File does not exist!')
            }
        } else {
            res.status(400).send('Invalid token!')
        }
      });
}