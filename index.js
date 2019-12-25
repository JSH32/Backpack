require('dotenv').config()

const express = require('express')
const mongo = require('./api/mongo')
const app = express()
const cookieParser = require('cookie-parser')

app.use(express.json())
app.use(cookieParser())

mongo.init().then(db => {
    app.use('/', express.static('uploads'))

    require('./api/authentication')({ 
        db, app 
    })
    require('./api/token/get')({
        db, app
    })
    require('./api/upload')({
        db, app
    })
    require('./api/token/regen')({
        db, app
    })
    require('./api/user/listfiles')({
        db, app
    })
    require('./api/delete')({
        db, app
    })
})

app.listen(8080)