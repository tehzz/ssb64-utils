/**
 * Holds F3dex vertices in sets of (max) 32
 * @constructor
**/
function VBank(id) {
  this.id = id;
  this.full = false;
  this.vertices = [];
}

VBank.prototype = {
  /**
  * @param {Vertex} v
  * @returns {int}
  **/
  find: function(v) {
    return this.vertices.findIndex( e => v.eq(e) )
  },
  /**
  * @param {Vertex} v
  * @returns {self}
  **/
  add: function(v) {
    this.vertices.push(v)

    return this
  },
  pushVert: function(v) {
    return this.vertices.push(v)
  },
  length: function() {
    return this.vertices.length
  },
  print: function(bass) {
    return [`Bank${this.id}:`]
      .concat(this.vertices.map( v => v.print(bass) ))
  }
}

module.exports = VBank
