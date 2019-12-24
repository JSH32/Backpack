require('dotenv').config()

const express = require('express')
const mongo = require('./api/mongo')
const app = express()
const cookieParser = require('cookie-parser')

app.use(express.json())
app.use(cookieParser())

mongo.init().then(db => {
    app.get('/', (req, res) => {
        res.send('hello world')
    })

    require('./api/authentication')({ 
        db, 
        app 
    })
})

app.listen(8080)