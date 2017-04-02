const equals = require('equals')
const int    = require('../../util/int.js')

//put in method to convert x,y,z to N64 fixed point
//put in method to convert s,t to N64 texture fixed point

function Vertex(){
  this.x = null
  this.y = null
  this.z = null
  this.s = null
  this.t = null
}

Vertex.prototype = {
  setPosition: function(x,y,z){
    this.x = x
    this.y = y
    this.z = z
    return this
  },
  scalePosition: function( factor ) {
    this.x *= factor
    this.y *= factor
    this.z *= factor
    return this
  },
  roundPosition: function() {
    this.x = Math.round(this.x)
    this.y = Math.round(this.y)
    this.z = Math.round(this.z)

    return this
  },
  setTexCoord: function(s,t){
    this.s = s
    this.t = t
    return this
  },
  eq: function(rhs){
    return equals(this, rhs)
  }
}

module.exports = Vertex
