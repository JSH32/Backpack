module.exports = {
  port: 8080,
  url: 'http://localhost:8080/',
  inviteOnly: false,

  fileLength: 7, // Length of file names
  maxUploadSize: 100, // In MegaBytes
  serveUpload: true, // Should uploads be served by node
  uploadDir: './uploads/', // Directory to store uploads

  database: {
    mongoUrl: 'mongodb://localhost:27017',
    dbName: 'kawaii'
  },

  admin: {
    register: false,
    key: 'supersecurekey'
  },

  s3: {
    // If using s3 set serveUpload to false
    // If you don't know what to put for the values below, don't use s3
    // uploadDir is used as temp, make sure its valid
    enable: true,
    bucket: 'bucket name here',
    accessKey: 'access key here',
    secretKey: 'secret key here',
    endpoint: 'endpoint key here'
  }
}
