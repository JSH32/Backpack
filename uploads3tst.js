// Filetesting

const fs = require('fs');
const AWS = require('aws-sdk');

const s3 = new AWS.S3({
    accessKeyId: 'WIS9VB83YNIT5IALT7QR',
    secretAccessKey: 'Fie4S9Uz7zBGYHpMidEfVR5whI9VWYlvtrqUE89q',
    endpoint: 's3.us-east-2.wasabisys.com'
});

const fileName = 'cat.txt';

const uploadFile = () => {
  fs.readFile(fileName, (err, data) => {
     if (err) throw err;
     const params = {
         Bucket: 'nekocftest', // pass your bucket name
         Key: fileName, // file will be saved as testBucket/contacts.csv
         Body: JSON.stringify(data, null, 2)
     };
     s3.upload(params, function(s3Err, data) {
         if (s3Err) throw s3Err
         console.log(`File uploaded successfully at ${data.Location}`)
     });
  });
};

uploadFile();