/**
 * Represents the G_ENDDL f3dex2 command
 * @constructor
**/
function G_ENDDL(){
  this.op_code = 0xDF
}

G_ENDDL.prototype = {
  /** Print the C-Style macro as a string
   * @returns {string}
  **/
  print: function(){
    return 'gsSPEndDisplayList()'
  },
  /** Emit the compiled microcode
   * @returns {int[]}
  **/
  binary: function() {
    return [0xDF000000 >>> 0, 0x00000000]
  }
}

module.exports = G_ENDDL;
