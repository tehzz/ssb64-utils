const Color = require('./Color.js')

function Material(name) {
  this.name = name;
  this.specularCoef = null;
  this.ambient = null;
  this.diffuse = null;
  this.specular = null;
  this.emitted = null;
  this.opticalDensity = null;
  this.alpha = null;
  this.illumination = null;
  this.uvMap = null;
}

Material.prototype.set = function( prop, val ) {
  this[prop] = val

  return this
}

Material.prototype.setNs = function(coef) {
  return this.set('specularCoef', coef)
}

Material.prototype.setKa = function([r,g,b]) {
  return this.set('ambient', new Color(r,g,b))
}

Material.prototype.setKd = function([r,g,b]) {
  return this.set('diffuse', new Color(r,g,b))
}

Material.prototype.setKs = function([r,g,b]) {
  return this.set('specular', new Color(r,g,b))
}

Material.prototype.setKe = function([r,g,b]) {
  return this.set('emitted', new Color(r,g,b))
}

Material.prototype.setDensity = function(a) {
  return this.set('opticalDensity', a)
}

Material.prototype.setAlpha = function(a) {
  return this.set('alpha', a)
}

module.exports = Material
