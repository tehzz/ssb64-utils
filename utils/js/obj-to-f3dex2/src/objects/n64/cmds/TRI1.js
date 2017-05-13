const triVertHelper = require('./tri-helper.js')

/**
 * Represents the G_TR1 FEDZEX/FEDEX2 command
 * @constructor
 * @param {int} v0 - first vertex index
 * @param {int} v1 - second vertex index
 * @param {int} v2 - third vertex index
 * @param {int} flag - order of vertices in command (where flag == primary vertex index)
 * @returns {Self}
**/
function G_TRI1(v0, v1, v2, flag = 0) {
  this.op_code = 0x05
  // hmmm, should I just re-order the vertices here,
  // or should I print the flag and the input order?
  this.flag = 0;
  this.vertIndices = triVertHelper(v0, v1, v2, flag);

  return this
}

G_TRI1.prototype = {
  /**
   * print the C-style macro string
   * @returns {string}
  **/
  print : function(){
    let v = this.vertIndices;

    return `gsSP1Triangle(${v[0]}, ${v[1]}, ${v[2]}, ${flag})`
  },
  /**
   * emit the compiled microcode
   * @returns {int[]}
  **/
  binary : function(){
    const v = this.vertIndices

    let word1  = this.op_code << 24
        word1 &= (v[0] << 2) << 16
        word1 &= (v[1] << 2) << 8
        word1 &= (v[2] << 2)

    return [word1 >>> 0, 0x00000000]
  }
}

module.exports = G_TRI1;
