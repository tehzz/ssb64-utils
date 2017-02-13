const int = Object.create(null)

int.toS8 = function( a ){
  let out = typeof a === 'number' ? a << 0 : parseInt(a,16)

  out &= 0xFF

  if ( out & 0x80 ) out -= 0x100

  return out
}

int.toU8 = function( a ){
  let out = typeof a === 'number' ? a << 0 : parseInt(a,16)

  return (out >>> 0) & 0xFF
}

module.exports = int
