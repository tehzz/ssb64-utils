const Vertex    = require('./Vertex.js'),
      VTexture  = require('./VTexture.js'),
      VNormal   = require('./VNormal.js'),
      Face      = require('./Face.js'),
      parsedOBJ = require('./OBJContain.js');

const parsed = new parsedOBJ();


function parse(lines){
  lines.forEach( line => {
    const keyword = line.split(/\b/)[0].trim();

    switch ( keyword ) {
      // empty line
      case '': break;

      // A single vertex
      case 'v' :
        let points = line.match(/\-?[0-9.]+/g).map(parseFloat);

        parsed['Vertices'].push( new Vertex(...points) );
        break;
      // Vertex Texture
      case 'vt':
        let coordinates = line.match(/\-?[0-9.]+/g).map(parseFloat);
        parsed['VTex'].push( new VTexture(...coordinates) )
        break;
      // Vertex Normal
      case 'vn':
        let normals = line.match(/\-?[0-9.]+/g).map(parseFloat);
        parsed['VNormals'].push( new VNormal(...normals) )
        break;

      case '#' :
        parsed['Comments'].push(line); break;
      case 'f':
        let face = line.match(/([0-9]+\/[0-9]?\/[0-9]?)/g)
                   .map( point => point.split('/').map( parseInt ) );
        face = new Face(face)
        // map face vertices to currently parsed vertex, vTextures, and vNormals
        face.setVIds(parsed['Vertices'].length,
                     parsed['VTex'].length,
                     parsed['VNormals'].length)
        // push to array
        parsed['Faces'].push(face)
        // link objects .... maybe remove
        parsed.linkFace(face)

        break;
      default:
        console.log(`Unknown line with keyword: '${keyword}'`)
        parsed['Unknown'].push(line)
    }
  })

  return parsed
}

module.exports = parse
