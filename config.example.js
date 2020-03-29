module.exports = {
    port: 8080,
    url: 'http://localhost:8080/',
    inviteOnly: false,

    fileLength: 2, // Length of file names
    maxUploadSize: 100, // In MegaBytes
    serveUpload: true, // Should uploads be served by node
    uploadDir: './uploads/', // Directory to store uploads

    database: {
        mongoUrl: 'mongodb://localhost:27017',
        dbName: 'nekos'
    },

    admin: {
        register: false,
        key: 'supersecurekey'
    }
}