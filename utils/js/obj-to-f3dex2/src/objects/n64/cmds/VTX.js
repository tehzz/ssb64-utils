/**
 * Represents the G_VTX f3dex2 command
 * @constructor
 * @param {int} vaddr - address of first vertex to load
 * @param {int} numv  - number of vertices to load from 1 to 32
 * @param {int} vbidx - starting RSP index for first loaded vertex
**/
function G_VTX(vaddr, numv, vbidx) {
  this.op_code = 0x01
  this.vaddr = vaddr;
  this.numv = numv;
  this.vbidx = vbidx
}

G_VTX.prototype = {
  /** Print the C-Style macro as a string
   * @returns {string}
  **/
  print : function() {
    return `gsSPVertex(${this.vaddr}, ${this.numv}, ${this.vbidx})`
  },
  /** Emit the compiled microcode
   * @returns {int[]}
  **/
  binary : function() {
    let word1  = this.op_code << 24
        word1 &= this.numv << 12
        word1 &= ((this.vbidx + this.numv) & 0x7F) << 1

    // Add isInt check? Since normally the vaddr will be a bass label..
    let word2 = this.vaddr & 0xFFFFFFFF

    return [word1 >>> 0, word2 >>> 0]
  }
}

module.exports = G_VTX
