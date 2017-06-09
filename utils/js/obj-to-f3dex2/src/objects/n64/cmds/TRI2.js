const triVertHelper = require('./tri-helper.js')

/**
 * Represents the G_TR1 FEDZEX/FEDEX2 command
 * @constructor
 * @param {int} v00 - triangle 1, first vertex index
 * @param {int} v01 - triangle 1, second vertex index
 * @param {int} v02 - triangle 1, third vertex index
 * @param {int} v10 - triangle 2, first vertex index
 * @param {int} v11 - triangle 2, second vertex index
 * @param {int} v12 - triangle 2, third vertex index
 * @param {int} flag0 - order of vertices in triangle 1 (where flag == primary vertex index)
 * @param {int} flag1 - order of vertices in triangle 2 (where flag == primary vertex index)
 * @returns {Self}
**/
function G_TRI2(v00, v01, v02, v10, v11, v12, flag0 = 0, flag1 = 0) {
  this.op_code = 0x06
  // hmmm, should I just re-order the vertices here,
  // or should I print the flag and the input order?
  this.flag = 0;
  this.vertIndices = triVertHelper(v00, v01, v02, flag0)
                    .concat(triVertHelper(v10, v11, v12, flag1));

  return this
}

G_TRI2.prototype = {
  /**
   * print the C-style macro string
   * @returns {string}
  **/
  print : function(){
    let v = this.vertIndices,
        flag = this.flag

    return `gsSP2Triangles(${v[0]}, ${v[1]}, ${v[2]}, ${flag}, ${v[3]}, ${v[4]}, ${v[5]}, ${flag})`
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

    let word2  = 0
        word2 &= (v[3] << 2) << 16
        word2 &= (v[4] << 2) << 8
        word2 &= (v[5] << 2)

    return [word1 >>> 0, word2 >>> 0]
  }
}

module.exports = G_TRI2;
