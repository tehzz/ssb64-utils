/**
 * Represents the G_RDPPIPESYNC f3dex2 command
 * @constructor
**/
function G_RDPPIPESYNC(){
  this.op_code = 0xE7
}

G_RDPPIPESYNC.prototype = {
  /** Print the C-Style macro as a string
   * @returns {string}
  **/
  print: function(){
    return 'gsDPPipeSync()'
  },
  /** Emit the compiled microcode
   * @returns {int[]}
  **/
  binary: function() {
    return [0xE7000000 >>> 0, 0x00000000]
  }
}

module.exports = G_RDPPIPESYNC;
