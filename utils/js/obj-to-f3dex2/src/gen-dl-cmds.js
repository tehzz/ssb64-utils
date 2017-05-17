/**
 * This object holds an array of an array of triangles indices or
 * a an array of quad indices. It also contains the id and number of
 * vertices in the banks that the geometries are linked to.
 * @constructor
**/
function accBank(id) {
  this.bank = id;
  this.vertsToLoad = 0;
  this.tris  = [];
  this.rects = [];
}

accBank.prototype = {
  getBank: function() {
    return bank
  }
}

/**
 * @param {DL.mesh} meshes - an array of f3dex geometry objects
 * @param {int[]} bVertLen - an array of the number of vertices in a bank, orderd by bank number
 * @returns {accBank[]}
**/
function organizeGeo(meshes, bVertLen) {
  // Transform the meshes object into an array of accBanks
  let output = meshes.reduce( (acc, geo, i, arr) => {
    let curBank, geoBank, geoType, geoTypeArr, curTriSet;
    // get the "accBank" from acc [] that this geo's verts are in
    geoBank = geo.bank
    curBank = acc[geoBank]
    // add a new accBank if not in acc[]
    if (curBank === undefined) {
      curBank = acc[geoBank] = new accBank(geoBank)

    }

    // check for which type of geometry
    switch (geo.bankVertices.length) {
      case 3:
        //console.log(`Mesh[${i}] is a Triangle`)
        //console.log(geo.bankVertices.slice())
        geoTypeArr = curBank['tris']
        // for triangles, can load two at once to RDP
        curTriSet  = geoTypeArr[geoTypeArr.length - 1]
        if ( curTriSet === undefined || curTriSet.length === 2 ) {
          // convert each accBank.tri trianlge index into the meshes' vertIndex[]
          geoTypeArr.push( [geo.bankVertices.slice()] )
        } else {
          // convert each accBank.tri trianlge index into the meshes' vertIndex[]
          curTriSet.push(geo.bankVertices.slice())
        }
        // then, convert from triangle
        break

      default:
        console.log(`Mesh[${i}] is an unknown geometry`)
    }
    return acc
  }, [])

  // set each accBank's number of verts to load
  output.forEach( (accBank, i) => accBank.vertsToLoad = bVertLen[i])

  return output
}

//----------
const f3dex2 = require('./objects/n64/Commands.js')
/**
 * This takes an array of accBank and turns them into a proper sequence
 * of F3DEX2 command objects
 * @param {accBank[]} acc
 * @returns {DLcmds[]}
**/
function emitCmds(acc) {
  //start with a sync 0xE7 command
  return [ new f3dex2.G_RDPPIPESYNC() ].concat(
    acc.reduce( (output, bank, i) => {
      // first, load vertices with 0x01
      output.push(
        new f3dex2.G_VTX(`Bank${bank.bank}`, bank.vertsToLoad, 0)
      )
      // then, generate triangle commmands based on the amount of ids in
      // the accBank.tris array
      return output.concat( bank.tris.map(ts => {
        let t1, t2;

        switch (ts.length) {
          case 1 :
            t1 = ts[0]
            return new f3dex2.G_TRI1(t1[0], t1[1], t1[2])
          break;
          case 2 :
            t1 = ts[0]
            t2 = ts[1]
            return new f3dex2.G_TRI2(t1[0], t1[1], t1[2], t2[0], t2[1], t2[2])
          break;
          default:
            console.log("Unknown length in AccBank.tris")
            console.log(ts)
            throw new Error("Error generating DL Commands: Unknown Triangle Array")
        }
        return false
      }))
    }, [])
  ).concat( [new f3dex2.G_ENDDL()] )
  // end with a 0xDF DL end command
}

function genDLcmds([p, dl]) {
  // Organize mesh from DL object into accBank[]
  const sets = organizeGeo(dl.mesh, dl.vBanks.map( b => b.length() ) )
  // Convert the accBank[] into DLcmds[]
  let emit = emitCmds(sets)

  // attach commands to the DL
  dl.concatCmds(emit)

  return [p, dl]
}
module.exports = genDLcmds;
