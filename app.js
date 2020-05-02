const express = require('express')
const mongo = require('./lib/mongo')
const app = express()
const config = require('./config')
const bodyParser = require('body-parser')
const minifyHTML = require('express-minify-html')
const AWS = require('aws-sdk')
const errorhandler = require('./lib/middleware/errorhandler')

app.use(express.json())
app.use(bodyParser.json())
app.set('view engine', 'ejs')
app.use(minifyHTML({
  exception_url: false,
  htmlMinifier: { removeComments: true, collapseWhitespace: true, collapseBooleanAttributes: true, removeAttributeQuotes: true, removeEmptyAttributes: true, minifyJS: true }
}))

const s3 = new AWS.S3({
  accessKeyId: config.s3.accessKey,
  secretAccessKey: config.s3.secretKey,
  endpoint: config.s3.endpoint
})

mongo.init().then(db => {
  // Serve uploaded files
  if (config.serveUpload == true) { app.use('/', express.static(config.uploadDir)) }

  // Initialize middleware
  app.use(errorhandler)

  app.use('/', express.static('./public')) // Public files
  require('./router')({ db, app, s3 }) // Router
})

app.listen(config.port)
