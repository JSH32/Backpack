require('dotenv').config()

const express = require('express')
const mongo = require('./lib/mongo')
const app = express()
const bodyParser = require('body-parser');
const minifyHTML = require('express-minify-html');

app.use(express.json())
app.use(bodyParser.json());
app.set('view engine', 'ejs');
app.use(minifyHTML({
    exception_url: false,
    htmlMinifier: { removeComments: true, collapseWhitespace: true, collapseBooleanAttributes: true, removeAttributeQuotes: true, removeEmptyAttributes: true, minifyJS: true }
}));

mongo.init().then(db => {
    // Serve uploaded files
    if (JSON.parse(process.env.UPLOADS_SERVE) == true) { app.use('/', express.static(process.env.UPLOAD_DIR)) }

    app.use('/', express.static('./public')) // Public files
    require('./router')({ db, app }) // Router
})

app.listen(process.env.PORT)