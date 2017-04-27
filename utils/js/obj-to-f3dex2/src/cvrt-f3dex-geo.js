const {Triangle} = require('./objects/f3dex.js');


/**
 * Function to convert from the collection of wavefront v / vt / vn with .mtl
 * color and associated 'f' statements info into F3DEX's vertex structure
 * single/double triangles
 * @param {obj.Container} p - The parsed info from the input .obj file
 * @param {float} scale     - Input scaling factor for vertices' x,y,z
 * @returns {Array}         - [p, ...f3dex.geometeries]
**/

function convert( p, scale ) {
  // for each each face,
  //  convert face to f3dex_Triangle? (or maybe square in the future)
  let n64_mesh = p.Faces.map( (face, i) => {
    const material = face['mtl'];
    let geometry;

    // convert face to an f3dex geometry
    switch (face.vertIDs.length) {
      case 3:
        geometry = new Triangle()
      break;

      default:
        console.log(`Face with ${face.vertIDs.length} vertices. Please implement... ):`)
        throw new Error("OBJ Face to F3dex geometry")
    }

    // attach obj v/vt/vn indices to the new geometry
    face.vertIDs.forEach( ([v,vt,vn]) => {
      geometry.addVertex([v,vt,vn, material]);
    })

    return geometry
  })


  return [p, n64_mesh]
}

module.exports = convert;
