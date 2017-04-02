const Vertex = require('./Vertex.js'),
      Color = require('../Color.js');

function Vertex_Color(){
  this.x = null
  this.y = null
  this.z = null
  this.s = null
  this.t = null
  this.color = null;
}

Vertex_Color.prototype = new Vertex()
Vertex_Color.prototype.constructor = Vertex_Color

Vertex_Color.prototype.setColor = function(r,g,b,a){
  this.color = new Color(r, g, b, a)
  return this
}

module.exports = Vertex_Color
