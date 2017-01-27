const Bank = require('./Bank.js')

// Bank storage array
const banks = []

/*
This function moves vertices and faces from a parsedOBJ container
 object into F3DEX2 compatiable banks of 32 vertices and faces

Returns [parsedOBJ, banks[Bank]]
*/

function bankOBJ( parsed ){

  // for each face, put that face's vertices into a bank
  parsed.Faces.forEach( (face,i) => {
    // find vertices already in bank[i]
    const vLocs = banks.map( bank => {
      // search for each vertex
      return face.vertIDs.map( vert => bank.findVertex(vert[0]) )
    })
    // sum of vertices to add to this bank
    const vToAdd = vLocs.map(t => {
      return t.reduce( (a,b)=>{
        return b < 0 ? a+1 : a
      }, 0 )
    })
    // check for full banks
    const bankFull = banks.map( (b,i) => {
      console.log(`Vertices in Bank[${i}]: ${b.length}
        Attempt to add ${vToAdd[i]} more...`)

      return b.length + vToAdd[i] <= 32
    })

    // return the index of the first open Bank for this face
    let openBank = bankFull.findIndex( b => b )
    // if none are open, add a new Bank
    if ( openBank < 0 ) {
      openBank = banks.push(new Bank(banks.length))
      openBank--
      console.log(`Adding new Bank[${openBank}] to banks`)
      // update vLocs for this new Bank
      vLocs.push( [-1,-1,-1] )
    }
    // add missing vertices to that Bank
    vLocs[openBank].forEach( (loc,i) => {
      if(loc < 0) banks[openBank].addFullVertex(face.vertIDs[i])
    })
    // add face 'i' to banks[index]
    console.log(`Adding Face[${i}] to Bank[${openBank}]`)
    banks[openBank].faces.push(i)

  })

  return [parsed, banks]
}

module.exports = bankOBJ
