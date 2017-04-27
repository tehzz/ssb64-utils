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
  //console.log(p)
  //console.log(dl)
  for( mesh in dl.mesh) {
    console.log(dl.mesh[mesh].bankVertices)
  }

  let test = dl.mesh.reduce( (acc, geo, i, arr) => {
    let collect, current;
    //check geometry
    switch (geo.bankVertices.length) {
      case 3 :
        console.log(`Mesh[${i}] is a Triangle`)
        collect = acc['tri']

        break;
      default:
        console.log(`Mesh[${i}] is an unknown geometery :(`)
    }

    current = collect[collect.length-1]

    if ( current === undefined || current.length === 2) {
      // add new set of two triangles
      collect.push( [i])
    } else {
      current.push(i)
    }

    return acc
  }, { "tri": [], "rect": []})

  console.log(test)
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
