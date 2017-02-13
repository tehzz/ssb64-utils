const Promise = require('promise'),
      argv    = require('argv'),
      cbfs    = require('fs'),
      path    = require('path'),
      parseOBJ = require('./src/parse-obj.js'),
      parseMTL = require('./src/parse-mtl.js'),
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
.then( parsed => {
  // read each .mtl file referenced in the main .obj
  const mtls = parsed['mtllib'].map(mtl => {
    return fs.readFile(path.join(file.dir,mtl), "utf-8")
  })

  return Promise.all([ parsed, ...mtls ])
})
.then( parseMTL )
.then(console.log)
/*
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
  console.log("Unknown Error... :(")
  console.log(err)
  throw err
})
