/**
 * Represents all necessary info for a F3DEX(2) display list
 * @constructor
 */

function dlist() {
  this.vBanks = [];
  this.commands = [];
  this.mesh = [];
  //this.textures = new Map();
}

dlist.prototype = {
  addBank: function(b){
    this['vBanks'].push(b)

    return this
  },
  command : {
    add : function(cmd) {
      this.commands.push(cmd)

      return this
    },
    print : function () {

    }
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
