const Promise = require('promise'),
      argv    = require('argv'),
      cbfs    = require('fs'),
      path    = require('path'),
      parseOBJ = require('./src/parse-obj.js'),
      bankOBJ = require('./src/bank-obj.js')

// promisify fs calls
const fs = {
  'readFile' : Promise.denodeify(cbfs.readFile),
  'writeFile': Promise.denodeify(cbfs.writeFile)
}

// set up CLI options and grab target+options
argv.option(require('./cli-options.js'))

let {targets, options} = argv.run(),
    file = path.parse(targets[0])

console.log(targets, options)

// read target file
fs.readFile(path.format(file), 'utf-8')
.catch(err => {
  console.log("File Read Error?")
  console.log(err)
  throw err
})
.then( contents => contents.split('\n').map( x => x.trim() ) )
.then( parseOBJ )
.then( bankOBJ )
.then( ([p,b]) =>{
  //parsed['Vertices'].forEach(vertex => vertex.scale(options['scale']).toInt())
  console.log(b)
  console.log(`Total Faces: ${p.Faces.length-1}`)
})
.catch(err => {
  console.log("Unknown Error... :(")
  console.log(err)
  throw err
})
