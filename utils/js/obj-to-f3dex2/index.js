const Promise = require('promise'),
      argv    = require('argv'),
      cbfs    = require('fs'),
      path    = require('path'),
      parseOBJ = require('./src/parse-obj.js'),
      parseMTL = require('./src/parse-mtl.js'),
      cvrtToFED3X = require('./src/cvrt-f3dex-geo.js'),
      organizeDL = require('./src/organize-dl.js'),
      genDLcmds = require('./src/gen-dl-cmds.js');

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
  console.log("ERROR: Please specify the input '.obj' file." +
  "Don't enter only the '-f' flag!")
  argv.help();

  process.exitCode = 1;
  return false
}

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
.then( genDLcmds )
.then( ([p, dl]) => {
  const sf = options['scale'] || 1
  // scale all vertices based on scale arg
  // then round to whole value
  dl.vBanks.forEach(vb => {
    vb.vertices.forEach( v => {
      v.scalePosition(sf)
        .roundPosition()
    })
  })

  return [p,dl]
})
.then( ([p, dl]) => {
  // Get path to output file
  let o_file_path
  if ( options['output'] ) {
    o_file_path = path.parse(options['output'])
  } else {
    let temp = Object.assign({}, file, {ext: ".bass" })
    delete temp['base']

    o_file_path = temp
  }
  o_file_path = path.format(o_file_path)

  // Generate contents of output file
  let output = [].concat.apply([], dl.vBanks.map( vb => vb.print(true) )).join("\n")

  output += "\n\ndisplaylist:\n"
  output += dl.printCmds().join("\n")

  // write to output file
  return [ fs.writeFile(o_file_path, output, 'utf-8'),
           o_file_path ]
})
.then( ([_,outputPath]) => {
  console.log(`Displaylist file written to ${outputPath}!`)
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
