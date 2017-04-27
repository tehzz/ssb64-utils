function accBank(id) {
  var bank = id;
  var geoTypes = new Map();

  this.tris  = [];
  this.rects = [];
}

accBank.prototype = {
  getBank: function() {
    return bank
  },
  getGeoType : function(gtEnum) {
    return geoTypes.get(gtEnum)
  },
  setGeoType : function(gtEnum) {
    return geoTypes.set(gtEnum, []).get(gtEnum)
  }
}

const geoTypesEnum = {
  Unknown  : 0,
  Triangle : 1,
  Quad     : 2
}
/**
 * @param {DL.mesh} meshes - an array of f3dex geometry objects
**/
function organizeGeo(meshes) {
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
        geoTypeArr = curBank['tris']
        // for triangles, can load two at once to RDP
        curTriSet  = geoTypeArr[geoTypeArr.length - 1]
        if ( curTriSet === undefined || curTriSet.length === 2 ) {
          geoTypeArr.push( [i] )
        } else {
          curTriSet.push(i)
        }
        break

      default:
        console.log(`Mesh[${i}] is an unknown geometry`)
    }
    return acc
  }, [])

  return output
}

module.exports = organizeGeo;
