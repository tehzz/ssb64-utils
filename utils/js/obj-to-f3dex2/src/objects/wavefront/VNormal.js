const int = require('../../util/int.js')

/* "The coordinates of the normal range from -1.0 to 1.0.
   In other words, -128 must be specified for the -1.0 value,
   and 128 must be specified for the 1.0 value.
   However, since the precision is signed 8-bit,
   the maximum positive value is actually 127,
   so the 1.0 value cannot be represented exactly.
   Thus, 0.992 is the maximum positive value."
*/
function unitToS8( unitVec ) {
  let output
  // convert a real unit vector into N64 s8
  output = unitVec < 0 ? unitVec * 128 : unitVec * 127

  return int.toS8(output)
}


function VNormal( i, j, k ){
  this.i = i
  this.j = j
  this.k = k
}


VNormal.prototype.getCoordsAsN64 = function(){
  return {
    i : int.toFixedS8_7(this.i),
    j : int.toFixedS8_7(this.j),
    k : int.toFixedS8_7(this.k),
  }
}

module.exports = VNormal
