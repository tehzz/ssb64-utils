function VNormal( i, j, k ){
  this.i = i
  this.j = j
  this.k = k
}

// ensure vertex normal is within -128 to 127 range
/* "The coordinates of the normal range from -1.0 to 1.0.
   In other words, -128 must be specified for the -1.0 value,
   and 128 must be specified for the 1.0 value.
   However, since the precision is signed 8-bit,
   the maximum positive value is actually 127,
   so the 1.0 value cannot be represented exactly.
   Thus, 0.992 is the maximum positive value."
*/
VNormal.prototype.convertTos8 = function(){
  this.i %= 255
}
module.exports = VNormal
