const MongoClient = require('mongodb').MongoClient;
const chalk = require('chalk');

const init = () => new Promise((resolve, reject) =>
  MongoClient.connect(process.env.MONGOURL, { 
    useUnifiedTopology: true, 
    useNewUrlParser: true 
  }, (err, client) => {
    console.log(chalk.greenBright("[mongo] Connected successfully to server!"))
    const db = client.db(process.env.DBNAME)
    resolve(db)
  })
)

module.exports = {
  init
}
