const Promise = require('promise'),
      argv    = require('argv'),
      cbfs    = require('fs'),
      path    = require('path'),
      parseOBJ = require('./src/parse-obj.js'),
      parseMTL = require('./src/parse-mtl.js'),
      cvrtToFED3X = require('./src/convert-to-f3dex2.js')
      bankOBJ  = require('./src/bank-obj.js')

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
.then( ([p, n64]) => {
  //console.log(p)
  console.log(n64[0])

  return [p,n64]
})
/*
.then( bankOBJ )
.then(([p,b]) =>{
  console.log(p['Materials'])
  let vertex = p['Faces'][0].vertIDs[0][0],
      vn = p['Faces'][0].vertIDs[0][2]

  console.log(p['Faces'][0], p['Vertices'][vertex])
  console.log(p['VNormals'][vn])
  console.log(p['VNormals'][vn].convertToN64())
})

.then( bankOBJ )
.then( ([p,b]) =>{
  //parsed['Vertices'].forEach(vertex => vertex.scale(options['scale']).toInt())
  //console.log(b)
  console.log(`Total Faces: ${p.Faces.length-1}`)
  p.VNormals.forEach( vn => vn.convertToS8() )

  return [p,b]
})
.catch( err => {
  console.log('Error parsing .obj file?')
  console.log(err)

  throw err
})
/*
.then( ([p,b]) => {
  let output = ''
  b.forEach( (bank,i) => {
    output += `// Drawing Bank ${i} \n`
    output += `gsSPVertex(Bank${i}, ${bank.length}, 0) \n`
    bank.faces.forEach(face => {
      //console.log(p.Faces[face].vertIDs)
      //find vert ids within bank...
      let [v0,v1,v2] = p.Faces[face].vertIDs.map( v => {
        //console.log(v[0])
        return bank.findVertex(v[0])
      })

      output += `gsSP1Triangle(${v0}, ${v1}, ${v2}, 0) \n`
    })
  })

  return output
})
.then(console.log)
*/
.catch(err => {
  switch (err.message) {
    case "fs.readFile":
      console.log("fs.readFile error propogated to final catch. \nAborting Program")
      break;
    default:
      console.log("Unknown Error in final Promise 'catch' statment. \nLogging and throwing to Node")
      console.log(err)
      throw err
  }
})
