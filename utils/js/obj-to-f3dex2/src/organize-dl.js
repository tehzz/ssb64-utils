const {Dlist, VBank} = require('./objects/f3dex.js');
const toF3DEXVtx     = require('./cvrt-f3dex-vert.js');

/**
 * Organize a set of N64 geometeries (mesh) into a display list bank of
 * vertices and geometeries (later to be converted into commands)
 * @param {obj.Container} p
 * @param {Array} mesh       - array of N64/F3DEX geometeries
 * @returns {Array}          - [p, Dlist]
**/
function organize( [p, mesh] ) {
  // instantiate new Display List
  let dl = new Dlist();

  // move mesh geometery into dl
  dl.setMesh(mesh)

  // step through each geometery, converting and adding its vertices to the DL
  mesh.forEach( (geo, i) => {
    // convert each objV into an N64 Vertex (color or normal)
    let verts = geo.objV.map( ( [v, vt, vn, mtl] ) => {
      v  = p['Vertices'][v]
      vt = p['VTex'][vt]
      vn = p['VNormals'][vn]
      let material = p.Materials.get(mtl);

      return toF3DEXVtx( [v, vt, vn], material)
    });

    // find array index of any vertices already in dl.vBanks
    // organized by [bank[verts]]
    let vLocs = dl.vBanks.map( b => {
      return verts.map( v => b.find(v) )
    });

    // Total number of vertices to add to a VBank (if chosen)
    let vToAdd = vLocs.map( vbank => {
      return vbank.reduce( (sum, i) => {
        // 'i' is the location of a vertex within a bank
        // so, i < 0 (or === -1), this vertex needs to be added
        return i < 0 ? sum + 1 : sum
      }, 0)
    });

    // find the first open bank
    let openBank = dl.vBanks.map( (b,i) => {
      return b.length() + vToAdd[i] <= 32
    }).findIndex( b => b );

    // Add new VBank if no vBanks are open
    if ( openBank < 0 ) {
      openBank = dl.vBanks.push( new VBank(dl.vBanks.length) ) - 1;
      // console.log(`Adding new VBank[${openBank}] to dl.vBanks`)
      // add a new vLocs array for the location of vertices
      vLocs.push( verts.map( _ => -1) );
    }

    // add missing vertices to openBank
    let bankedVLocs = vLocs[openBank].map( (loc, i) => {
      if (loc < 0) {
        return ( dl.vBanks[openBank].pushVert(verts[i]) - 1)
      } else {
        return loc
      }
    })

    // update geometery's bank property and bank vertex locations
    geo.setBank(openBank)
    bankedVLocs.forEach( loc => geo.bankVertex(loc) )
  })

  return [p, dl]
}

module.exports = organize
