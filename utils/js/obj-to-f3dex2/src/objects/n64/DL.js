/**
 * Represents all necessary info for a F3DEX(2) display list
 * @constructor
 */

function dlist() {
  /** {VBank[]} */
  this.vBanks = [];
  /** {F3DEX2[]} */
  this.commands = [];
  /** {Geometry[]} **/
  this.mesh = [];
  //this.textures = new Map();
}

dlist.prototype = {
  addBank: function(b){
    this['vBanks'].push(b)

    return this
  },
  /** @param {FEDEX2[]} cmdArr - array of f3dex2 objects **/
  concatCmds : function(cmdArr) {
    this.commands = this.commands.concat(cmdArr)

    return this
  },
  /** @param {FEDEX2} cmd - a single f3dex2 objects **/
  pushCmd : function(cmd) {
    this.commands.push(cmd)
    return this
  },
  /** @returns {string[]} **/
  printCmds : function () {
    return this.commands.map(cmd => cmd.print())
  },
  setMesh : function( mesh ) {
    this.mesh = mesh

    return this
  },
  addMesh : function( m ) {
    this.mesh.push(m)

    return this
  }
}

module.exports = dlist;
