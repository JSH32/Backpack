require('dotenv').config()

const express = require('express')
const mongo = require('./api/mongo')
const app = express()
const bodyParser = require('body-parser');

app.use(express.json())
app.use(bodyParser.json());

mongo.init().then(db => {
    app.use('/', express.static(process.env.UPLOAD_DIR))
    app.use('/', express.static('./public'))

    require('./api/user/signup')({ 
        db, app 
    })
    require('./api/user/info')({ 
        db, app 
    })
    require('./api/user/passreset')({ 
        db, app 
    })
    require('./api/user/delete')({ 
        db, app 
    })
    require('./api/token/get')({
        db, app
    })
    require('./api/token/valid')({
        db, app
    })
    require('./api/files/upload')({
        db, app
    })
    require('./api/admin/regkeygen')({ 
        db, app 
    })
    require('./api/token/regen')({
        db, app
    })
    require('./api/files/listfiles')({
        db, app
    })
    require('./api/files/delete')({
        db, app
    })
    require('./api/info')({
        db, app
    })
})

app.listen(8080)