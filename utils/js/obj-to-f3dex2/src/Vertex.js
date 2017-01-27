function Vertex( x, y, z, w = 1.0) {
  this.x  = x
  this.y  = y
  this.z  = z
  this.w  = w
}

Vertex.prototype.scale = function( factor ){
  this.x *= factor
  this.y *= factor
  this.z *= factor;

  return this
}

Vertex.prototype.toInt = function(){
  this.x = Math.round(this.x)
  this.y = Math.round(this.y)
  this.z = Math.round(this.z)

  return this
}

Vertex.prototype.addId = function( id ) {
  this.id = id

  return this
}

module.exports = Vertex
