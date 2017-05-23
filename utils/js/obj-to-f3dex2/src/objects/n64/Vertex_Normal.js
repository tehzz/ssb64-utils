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
Vertex_Normal.prototype.getNormalAsFixed = function(){
  return {
    nx : int.toFixedS8_7(this.nx),
    ny : int.toFixedS8_7(this.ny),
    nz : int.toFixedS8_7(this.nz),
  }
}

// Fixed Point signed Q10.5 / 1:10:5
Vertex_Normal.prototype.getTextAsFixed = function() {
  return {
    s : int.toFixedS16_5(this.s),
    t : int.toFixedS16_5(this.t)
  }
}

Vertex_Normal.prototype.print = function(bass){
  let vn = this;
  let nx, ny, nz, s, t;

  if (bass) {
    ({nx, ny, nz} = vn.getNormalAsFixed());
    ({s, t}       = vn.getTextAsFixed());
  } else {
    ({nx, ny, nz, s, t} = vn);
  }

  return `Vertex_Normal(${vn.x}, ${vn.y}, ${vn.z}, ` +
          `${s? s : 0}, ${t? t : 0}, ` +
          `${nx? nx : 0}, ${ny? ny : 0}, ${nz? nz : 0})`
}

module.exports = Vertex_Normal
