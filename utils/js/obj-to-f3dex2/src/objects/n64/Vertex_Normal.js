const Vertex = require('./Vertex.js')
const int = require('../../util/int.js')

function Vertex_Normal(){
  this.x = null
  this.y = null
  this.z = null
  this.s = null
  this.t = null
  this.nx = null
  this.ny = null
  this.nz = null
  this.alpha = null;
}

Vertex_Normal.prototype = new Vertex()
Vertex_Normal.prototype.constructor = Vertex_Normal
/**
 * This sets the x,y,z values of the normal.
 * Limit to N64 -1.0 to 1.0 range?
  * @param {float} nx - x value of normal
  * @param {float} ny - y value of normal
  * @param {float} nz - z value of normal
**/
Vertex_Normal.prototype.setNormal = function(nx, ny, nz){
  this.nx = nx
  this.ny = ny
  this.nz = nz
  return this
}

/**
 * Set alpha of vertex normal.
 * Optionally, convert from [0,1.0] to 8-bit representation
  * @param {float} a - alpha level. Normally [0,1] range
  * @param {bool} convert - convert alpha level to 8 bit representation
**/
Vertex_Normal.prototype.setAlpha = function(a, convert = false){
  this.alpha = convert ? ( (a * 0xFF) >>> 0 ) : a;

  return this
}

/**
 * "The coordinates of the normal range from -1.0 to 1.0.
 * In other words, -128 must be specified for the -1.0 value,
 * and 128 must be specified for the 1.0 value.
 * However, since the precision is signed 8-bit,
 * the maximum positive value is actually 127,
 * so the 1.0 value cannot be represented exactly.
 * Thus, 0.992 is the maximum positive value."
**/
Vertex_Normal.prototype.getCoordsAsFixedS8_7 = function(){
  return {
    i : int.toFixedS8_7(this.i),
    j : int.toFixedS8_7(this.j),
    k : int.toFixedS8_7(this.k),
  }
}

Vertex_Normal.prototype.print = function(){
  let vn = this

  return `Vertex_Normal(${vn.x}, ${vn.y}, ${vn.z}, ` +
          `${vn.s}, ${vn.t}, ` +
          `${vn.nx}, ${vn.ny}, ${vn.nz})`
}

module.exports = Vertex_Normal
