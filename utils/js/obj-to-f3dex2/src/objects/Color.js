const int = require('../util/int.js')

function Color(r,g,b, a = null) {
  this.r = r
  this.g = g
  this.b = b
  this.a = a
}

// convert from float 0 to 1 to int 0 to 255
Color.prototype.to8bit = function() {
  for (chan in this) {
    const color = this[chan]
    if( color !== null && color >= 0 && color <= 1) {
      this[chan] = int.toU8(color * 0xFF)
    }
  }

  return this
}

module.exports = Color
