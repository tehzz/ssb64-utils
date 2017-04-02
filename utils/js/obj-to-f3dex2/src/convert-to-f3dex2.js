const {Triangle, Vertex_Normal, Vertex_Color} = require('./objects/f3dex.js')


/**
 * Function to convert from the collection of wavefront v / vt / vn with .mtl
 * color and associated 'f' statements info into F3DEX's vertex structure
 * single/double triangles
 * @param {obj.Container} p - The parsed info from the input .obj file
 * @param {float} scale     - Input scaling factor for vertices' x,y,z
**/

function convert( p, scale ) {
  // for each each face,
  //  convert face to f3dex_Triangle?
  //  convert face's vertices to n64 Vertex_Color or Vertex_Normal
  const n64_mesh = []

  p.Faces.forEach( (face, i) => {
    const material = p['Materials'].get(face['mtl']);
    let geometery;

    // convert face to an f3dex geometery
    switch (face.vertIDs.length) {
      case 3:
        geometery = new Triangle()
      break;

      default:
        console.log(`Face with ${face.vertIDs.length} vertices. Please implement... ):`)
    }
    // convert face's vert/vt/vn triplet into Vertex_Color or Vertex_Normal
    // based on the presence of vn value
    face.vertIDs.forEach( ([v,vt,vn]) => {
      const objVertex = p['Vertices'][v],    //convert x,y,z right here into pos
            texCoord  = vt !== null ? p['VTex'][vt] : null;

      let vertex;

      if ( vn !== null ) {
      // create a vertex normal
        let objNormal = p['VNormals'][vn]

        vertex = new Vertex_Normal()
        vertex.setPosition(objVertex.x, objVertex.y, objVertex.z)
              .setNormal(objNormal.i, objNormal.j, objNormal.k)
              .setAlpha( material['alpha'], true)
              //.setTextCoor(s,t)
      } else {
        // make a vertex color with color info from the .mtl file
        let {r, g, b} = material['ambient'],
                    a = material['alpha'];

        vertex = new Vertex_Color()

        vertex.setPosition(objVertex.x, objVertex.y, objVertex.z)
              .setColor(r * 0xFF, g * 0xFF, b * 0xFF, a * 0xFF)
              //.setTextCoor(s,t)
      }
      geometery.addVertex(vertex)
    })
    n64_mesh.push(geometery)
  })

  return [p, n64_mesh]
}

module.exports = convert;
