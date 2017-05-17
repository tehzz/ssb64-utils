const Promise = require('promise'),
      argv    = require('argv'),
      cbfs    = require('fs'),
      path    = require('path'),
      parseOBJ = require('./src/parse-obj.js'),
      parseMTL = require('./src/parse-mtl.js'),
      cvrtToFED3X = require('./src/cvrt-f3dex-geo.js'),
      organizeDL = require('./src/organize-dl.js')

// promisify fs calls
const fs = {
  'readFile' : Promise.denodeify(cbfs.readFile),
  'writeFile': Promise.denodeify(cbfs.writeFile)
}

// set up CLI options and grab target+options
argv.option(require('./cli-options.js'))

let {targets, options} = argv.run(),
    file;

// check if there is an input file
if ( !options['file'] ) {
  console.log("ERROR: Enter an input '.obj' to parse!")
  argv.help();

  process.exitCode = 1;
  return false
}

file = path.parse(options['file'])

// make sure that a file was specified, not just the '-f' flag
if ( file.base === 'true' ) {
  console.log("ERROR: Please specify the input '.obj' file; don't just enter the '-f' flag!")
  argv.help();

  process.exitCode = 1;
  return false
}

//console.log(targets, options)
//console.log(file)

// read target file
fs.readFile(path.format(file), 'utf-8')
.catch(err => {
  console.log("File Read Error")
  console.log(err)
  throw new Error("fs.readFile")
})
.then( contents => contents.split('\n').map( x => x.trim() ) )
.then( parseOBJ )
.then( parsed => {
  // read each .mtl file referenced in the main .obj
  const mtls = parsed['mtllib'].map(mtl => {
    return fs.readFile(path.join(file.dir,mtl), "utf-8")
  })

  return Promise.all([ parsed, ...mtls ])
})
.then( parseMTL )
.then( cvrtToFED3X )
.then( organizeDL )
.then( ([p, dl]) => {
  console.log(dl.mesh.length)
  console.log(dl.mesh[0])
  for (vBank of dl.vBanks) {
    console.log(`vBank id ${vBank.id} length = ${vBank.length()}`)
    //console.log(vBank)
  }

  /*for( mesh in dl.mesh) {
    console.log(dl.mesh[mesh].bankVertices)
  }*/

  const testfn = require('./src/gen-dl-cmds.js')
  let test = testfn([p,dl])
  console.log(test[1])
  console.log(test[1].printCmds())
  console.log( [].concat.apply([], dl.vBanks.map( vb => vb.print() )) )
  return [p,dl]
})
.catch(err => {
  switch (err.message) {
    case "fs.readFile":
      console.log("fs.readFile error propogated to final catch. \nAborting Program")
      process.exitCode = 1;
      break;
    default:
      console.log("Unknown Error in final Promise 'catch' statment. \nLogging and throwing to Node")
      console.log(err)
      process.exitCode = 1;
      throw err
  }
})
